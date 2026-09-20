export interface AuditNode {
  selector: string;
  tag: string;
  textSnippet: string;
  fgHex: string;
  bgHex: string;
  fontSize: number;
  fontWeight: number;
  isLargeText: boolean;
  wcagRatio: number;
  wcagPassAA: boolean;
  wcagPassAAA: boolean;
  apcaLc: number;
  apcaPassBody: boolean;
  apcaPassLarge: boolean;
  boundingBox: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
}

export interface AuditSummary {
  url: string;
  timestamp: string;
  totalAudited: number;
  passedWcagAA: number;
  passedWcagAAA: number;
  passedApcaBody: number;
  passedApcaLarge: number;
  wcagAaComplianceRate: number;
  apcaBodyComplianceRate: number;
  averageApcaLc: number;
  executionTimeMs: number;
}

export interface AuditReport {
  summary: AuditSummary;
  criticalFailures: AuditNode[];
  allNodes: AuditNode[];
}
