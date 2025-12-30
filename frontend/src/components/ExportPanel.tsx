import { Download, Copy } from 'lucide-react';
import type { CrochetPattern, AmigurumiConfig } from '../types';

interface Props {
  pattern: CrochetPattern | null;
  config: AmigurumiConfig;
}

export default function ExportPanel({ pattern, config }: Props) {
  if (!pattern) {
    return (
      <div className="card p-8 text-center">
        <p className="text-slate-600">
          No pattern to export. Generate a pattern first.
        </p>
      </div>
    );
  }

  const handleCopyText = () => {
    const text = generateTextPattern(pattern, config);
    navigator.clipboard.writeText(text);
    alert('Pattern copied to clipboard!');
  };

  const handleDownloadJSON = () => {
    const json = JSON.stringify({ pattern, config }, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'crochet-pattern.json';
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleDownloadText = () => {
    const text = generateTextPattern(pattern, config);
    const blob = new Blob([text], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'crochet-pattern.txt';
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-6">
      <div className="card p-8">
        <h2 className="text-xl font-semibold text-slate-900 mb-6">
          Export Options
        </h2>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <button
            onClick={handleCopyText}
            className="btn-secondary flex items-center justify-center gap-2"
          >
            <Copy size={20} />
            Copy to Clipboard
          </button>

          <button
            onClick={handleDownloadText}
            className="btn-secondary flex items-center justify-center gap-2"
          >
            <Download size={20} />
            Download Text
          </button>

          <button
            onClick={handleDownloadJSON}
            className="btn-secondary flex items-center justify-center gap-2"
          >
            <Download size={20} />
            Download JSON
          </button>
        </div>
      </div>

      <div className="card p-8">
        <h2 className="text-xl font-semibold text-slate-900 mb-4">
          Pattern Preview
        </h2>

        <div className="bg-cream-100 p-6 rounded-lg font-mono text-sm whitespace-pre-wrap max-h-[600px] overflow-y-auto">
          {generateTextPattern(pattern, config)}
        </div>
      </div>
    </div>
  );
}

function generateTextPattern(pattern: CrochetPattern, config: AmigurumiConfig): string {
  let text = 'CROCHET AMIGURUMI PATTERN\n';
  text += '='.repeat(50) + '\n\n';

  text += 'MATERIALS:\n';
  text += `- Yarn: ${pattern.metadata.yarn_length_meters.toFixed(1)}m (plus 20% extra)\n`;
  text += `- Hook: ${config.yarn.recommended_hook_size_mm}mm\n`;
  text += '- Stitch marker\n';
  text += '- Yarn needle\n';
  text += '- Polyester fiberfill stuffing\n\n';

  text += 'GAUGE:\n';
  text += `- ${config.yarn.gauge_stitches_per_cm} stitches per cm\n`;
  text += `- ${config.yarn.gauge_rows_per_cm} rows per cm\n\n`;

  text += 'FINISHED SIZE:\n';
  text += `- Height: ${config.total_height_cm} cm\n`;
  text += `- Bottom diameter: ${config.start_diameter_cm} cm\n`;
  text += `- Top diameter: ${config.end_diameter_cm} cm\n\n`;

  text += 'ABBREVIATIONS:\n';
  text += '- SC: Single Crochet\n';
  text += '- INC: Increase (2 SC in same stitch)\n';
  text += '- DEC: Decrease (2 stitches together)\n';
  text += '- INVDEC: Invisible Decrease\n\n';

  text += 'PATTERN:\n';
  text += '-'.repeat(50) + '\n\n';

  for (const row of pattern.rows) {
    text += `Row ${row.row_number}: `;
    
    if (row.pattern && row.pattern.length > 0) {
      const counts: Record<string, number> = {};
      
      for (const stitch of row.pattern) {
        const type = stitch.stitch_type;
        counts[type] = (counts[type] || 0) + 1;
      }

      const parts = Object.entries(counts)
        .map(([type, count]) => `${count} ${type}`)
        .join(', ');
      
      text += `${parts} (${row.total_stitches} stitches total)\n`;
    } else {
      text += `${row.total_stitches} SC\n`;
    }
  }

  text += '\n' + '='.repeat(50) + '\n';
  text += `Total Rows: ${pattern.metadata.total_rows}\n`;
  text += `Total Stitches: ${pattern.metadata.total_stitches}\n`;
  text += `Estimated Time: ${Math.round(pattern.metadata.estimated_time_minutes)} minutes\n`;

  return text;
}
