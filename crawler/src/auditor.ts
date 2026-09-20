import { chromium } from 'playwright';
import { AuditNode, AuditReport, AuditSummary } from './types';

export interface AuditOptions {
  timeout?: number;
  viewportWidth?: number;
  viewportHeight?: number;
  maxNodes?: number;
}

export async function auditUrl(url: string, options: AuditOptions = {}): Promise<AuditReport> {
  const startTime = Date.now();
  const timeout = options.timeout ?? 30000;
  const viewport = {
    width: options.viewportWidth ?? 1280,
    height: options.viewportHeight ?? 800,
  };

  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport });
  const page = await context.newPage();

  try {
    await page.goto(url, { waitUntil: 'domcontentloaded', timeout });
    await page.waitForTimeout(1000);

    const rawNodes = await page.evaluate(() => {
      function srgbToLinear(c: number): number {
        return c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
      }

      function relativeLuminance(r: number, g: number, b: number): number {
        return 0.2126729 * srgbToLinear(r / 255) + 0.7151522 * srgbToLinear(g / 255) + 0.0721750 * srgbToLinear(b / 255);
      }

      function parseRgba(str: string): { r: number; g: number; b: number; a: number } {
        const match = str.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.]+))?\)/);
        if (!match) return { r: 0, g: 0, b: 0, a: 1 };
        return {
          r: parseInt(match[1], 10),
          g: parseInt(match[2], 10),
          b: parseInt(match[3], 10),
          a: match[4] !== undefined ? parseFloat(match[4]) : 1,
        };
      }

      function rgbToHex(r: number, g: number, b: number): string {
        const toHex = (n: number) => Math.round(Math.max(0, Math.min(255, n))).toString(16).padStart(2, '0');
        return `#${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase();
      }

      function compositeRgba(fg: { r: number; g: number; b: number; a: number }, bg: { r: number; g: number; b: number; a: number }) {
        const alpha = fg.a + bg.a * (1 - fg.a);
        if (alpha === 0) return { r: 255, g: 255, b: 255, a: 1 };
        const r = (fg.r * fg.a + bg.r * bg.a * (1 - fg.a)) / alpha;
        const g = (fg.g * fg.a + bg.g * bg.a * (1 - fg.a)) / alpha;
        const b = (fg.b * fg.a + bg.b * bg.a * (1 - fg.a)) / alpha;
        return { r, g, b, a: alpha };
      }

      function getEffectiveBg(el: HTMLElement): { r: number; g: number; b: number; a: number } {
        let current: HTMLElement | null = el;
        let accum = { r: 255, g: 255, b: 255, a: 0 };

        while (current) {
          const style = window.getComputedStyle(current);
          const bgStr = style.backgroundColor;
          if (bgStr && bgStr !== 'transparent' && bgStr !== 'rgba(0, 0, 0, 0)') {
            const parsed = parseRgba(bgStr);
            if (parsed.a > 0) {
              accum = compositeRgba(accum, parsed);
              if (accum.a >= 0.99) break;
            }
          }
          current = current.parentElement;
        }

        if (accum.a < 0.99) {
          accum = compositeRgba(accum, { r: 255, g: 255, b: 255, a: 1 });
        }
        return accum;
      }

      function apcaY(r: number, g: number, b: number): number {
        const rLin = Math.pow(r / 255, 2.4);
        const gLin = Math.pow(g / 255, 2.4);
        const bLin = Math.pow(b / 255, 2.4);
        let y = 0.2126729 * rLin + 0.7151522 * gLin + 0.0721750 * bLin;
        if (y < 0.022) {
          y += Math.pow(0.022 - y, 1.414);
        }
        return y;
      }

      function calculateApca(txt: { r: number; g: number; b: number }, bg: { r: number; g: number; b: number }): number {
        const yTxt = apcaY(txt.r, txt.g, txt.b);
        const yBg = apcaY(bg.r, bg.g, bg.b);
        if (Math.abs(yTxt - yBg) < 0.0005) return 0;
        const isDarkOnLight = yBg > yTxt;
        return isDarkOnLight
          ? -(Math.pow(yBg, 0.65) - Math.pow(yTxt, 0.56)) * 1.141445 * 100
          : (Math.pow(yTxt, 0.62) - Math.pow(yBg, 0.57)) * 1.141445 * 100;
      }

      const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_ELEMENT);
      const items: any[] = [];

      while (walker.nextNode()) {
        const el = walker.currentNode as HTMLElement;
        if (!el || el.children.length > 0) continue;

        const text = el.innerText?.trim();
        if (!text || text.length === 0) continue;

        const rect = el.getBoundingClientRect();
        if (rect.width === 0 || rect.height === 0) continue;

        const style = window.getComputedStyle(el);
        if (style.display === 'none' || style.visibility === 'hidden' || parseFloat(style.opacity) === 0) continue;

        const fg = parseRgba(style.color);
        const bg = getEffectiveBg(el);

        const l1 = relativeLuminance(fg.r, fg.g, fg.b);
        const l2 = relativeLuminance(bg.r, bg.g, bg.b);
        const ratio = (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05);

        const fontSize = parseFloat(style.fontSize) || 16;
        const fontWeight = parseInt(style.fontWeight, 10) || 400;
        const isLarge = fontSize >= 24 || (fontSize >= 18.66 && fontWeight >= 700);

        const apca = calculateApca(fg, bg);
        const absApca = Math.abs(apca);

        const wcagAA = isLarge ? ratio >= 3.0 : ratio >= 4.5;
        const wcagAAA = isLarge ? ratio >= 4.5 : ratio >= 7.0;

        items.push({
          selector: el.tagName.toLowerCase() + (el.id ? '#' + el.id : el.className ? '.' + el.className.split(' ').slice(0, 2).join('.') : ''),
          tag: el.tagName.toLowerCase(),
          textSnippet: text.substring(0, 60),
          fgHex: rgbToHex(fg.r, fg.g, fg.b),
          bgHex: rgbToHex(bg.r, bg.g, bg.b),
          fontSize,
          fontWeight,
          isLargeText: isLarge,
          wcagRatio: parseFloat(ratio.toFixed(2)),
          wcagPassAA: wcagAA,
          wcagPassAAA: wcagAAA,
          apcaLc: parseFloat(apca.toFixed(1)),
          apcaPassBody: absApca >= 75.0,
          apcaPassLarge: absApca >= 45.0,
          boundingBox: {
            x: Math.round(rect.x),
            y: Math.round(rect.y),
            width: Math.round(rect.width),
            height: Math.round(rect.height),
          },
        });
      }

      return items;
    });

    await browser.close();

    const totalAudited = rawNodes.length;
    const passedWcagAA = rawNodes.filter((n) => n.wcagPassAA).length;
    const passedWcagAAA = rawNodes.filter((n) => n.wcagPassAAA).length;
    const passedApcaBody = rawNodes.filter((n) => n.apcaPassBody).length;
    const passedApcaLarge = rawNodes.filter((n) => n.apcaPassLarge).length;

    const sumApca = rawNodes.reduce((acc, n) => acc + Math.abs(n.apcaLc), 0);
    const averageApcaLc = totalAudited > 0 ? parseFloat((sumApca / totalAudited).toFixed(1)) : 0;

    const summary: AuditSummary = {
      url,
      timestamp: new Date().toISOString(),
      totalAudited,
      passedWcagAA,
      passedWcagAAA,
      passedApcaBody,
      passedApcaLarge,
      wcagAaComplianceRate: totalAudited > 0 ? parseFloat(((passedWcagAA / totalAudited) * 100).toFixed(1)) : 100,
      apcaBodyComplianceRate: totalAudited > 0 ? parseFloat(((passedApcaBody / totalAudited) * 100).toFixed(1)) : 100,
      averageApcaLc,
      executionTimeMs: Date.now() - startTime,
    };

    const criticalFailures = rawNodes.filter((n) => !n.wcagPassAA || !n.apcaPassLarge);

    return {
      summary,
      criticalFailures,
      allNodes: rawNodes,
    };
  } catch (err) {
    await browser.close();
    throw err;
  }
}

export function generateMarkdownReport(report: AuditReport): string {
  const { summary, criticalFailures } = report;

  let md = `# Accessibility Contrast Audit Report\n\n`;
  md += `- **URL Audited:** \`${summary.url}\`\n`;
  md += `- **Timestamp:** \`${summary.timestamp}\`\n`;
  md += `- **Execution Time:** \`${summary.executionTimeMs} ms\`\n`;
  md += `- **Total Elements Audited:** \`${summary.totalAudited}\`\n\n`;

  md += `## Compliance Summary\n\n`;
  md += `| Standard | Threshold | Passed Elements | Compliance Rate |\n`;
  md += `| :--- | :--- | :--- | :--- |\n`;
  md += `| **WCAG 2.1 Level AA** | 4.5:1 (Normal) / 3.0:1 (Large) | ${summary.passedWcagAA} / ${summary.totalAudited} | **${summary.wcagAaComplianceRate}%** |\n`;
  md += `| **WCAG 2.1 Level AAA** | 7.0:1 (Normal) / 4.5:1 (Large) | ${summary.passedWcagAAA} / ${summary.totalAudited} | **${((summary.passedWcagAAA / Math.max(1, summary.totalAudited)) * 100).toFixed(1)}%** |\n`;
  md += `| **APCA WCAG 3.0 Body** | $L_c \\ge 75$ | ${summary.passedApcaBody} / ${summary.totalAudited} | **${summary.apcaBodyComplianceRate}%** |\n`;
  md += `| **APCA WCAG 3.0 Large** | $L_c \\ge 45$ | ${summary.passedApcaLarge} / ${summary.totalAudited} | **${((summary.passedApcaLarge / Math.max(1, summary.totalAudited)) * 100).toFixed(1)}%** |\n`;
  md += `| **Mean Perceptual Contrast** | Average $L_c$ | — | **${summary.averageApcaLc} Lc** |\n\n`;

  if (criticalFailures.length > 0) {
    md += `## Critical Contrast Failures (${criticalFailures.length})\n\n`;
    md += `| Selector | Text Snippet | FG | BG | WCAG Ratio | APCA $L_c$ | Status |\n`;
    md += `| :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n`;

    for (const f of criticalFailures.slice(0, 30)) {
      md += `| \`${f.selector}\` | "${f.textSnippet.replace(/\|/g, '\\|')}" | \`${f.fgHex}\` | \`${f.bgHex}\` | **${f.wcagRatio}:1** | **${f.apcaLc}** | ❌ FAIL |\n`;
    }
  } else {
    md += `## Critical Contrast Failures\n\nNo critical contrast violations detected. All elements meet WCAG 2.1 Level AA and APCA requirements.\n`;
  }

  return md;
}
