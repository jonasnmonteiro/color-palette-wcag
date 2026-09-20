import { auditUrl, generateMarkdownReport } from './auditor';

async function run() {
  const args = process.argv.slice(2);
  const targetUrl = args.find((a) => a.startsWith('http://') || a.startsWith('https://'));

  if (!targetUrl) {
    console.error('Usage: bun run src/cli.ts <URL> [--format json|markdown]');
    process.exit(1);
  }

  const isJson = args.includes('--json') || args.includes('--format=json');

  try {
    const report = await auditUrl(targetUrl);
    if (isJson) {
      console.log(JSON.stringify(report, null, 2));
    } else {
      console.log(generateMarkdownReport(report));
    }
  } catch (err: any) {
    console.error(`Audit failed: ${err.message}`);
    process.exit(1);
  }
}

run();
