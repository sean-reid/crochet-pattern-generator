import type { CrochetPattern, AmigurumiConfig } from '../types';

interface Props {
  pattern: CrochetPattern | null;
  config: AmigurumiConfig;
}

export default function PatternPreview({ pattern, config }: Props) {
  if (!pattern) {
    return (
      <div className="card p-8 text-center">
        <p className="text-slate-600">
          No pattern generated yet. Configure and generate a pattern first.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Metadata */}
      <div className="card p-8">
        <h2 className="text-xl font-semibold text-slate-900 mb-4">
          Pattern Summary
        </h2>
        
        <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
          <div>
            <p className="text-sm text-slate-600">Total Rows</p>
            <p className="text-2xl font-bold text-terracotta-500">
              {pattern.metadata.total_rows}
            </p>
          </div>
          
          <div>
            <p className="text-sm text-slate-600">Total Stitches</p>
            <p className="text-2xl font-bold text-terracotta-500">
              {pattern.metadata.total_stitches}
            </p>
          </div>
          
          <div>
            <p className="text-sm text-slate-600">Estimated Time</p>
            <p className="text-2xl font-bold text-terracotta-500">
              {Math.round(pattern.metadata.estimated_time_minutes)} min
            </p>
          </div>
          
          <div>
            <p className="text-sm text-slate-600">Yarn Needed</p>
            <p className="text-2xl font-bold text-terracotta-500">
              {pattern.metadata.yarn_length_meters.toFixed(1)} m
            </p>
          </div>
        </div>
      </div>

      {/* Row-by-row instructions */}
      <div className="card p-8">
        <h2 className="text-xl font-semibold text-slate-900 mb-4">
          Row Instructions
        </h2>
        
        <div className="space-y-2 max-h-[500px] overflow-y-auto">
          {pattern.rows.map((row) => (
            <div
              key={row.row_number}
              className="flex items-start gap-4 p-3 rounded-lg hover:bg-cream-100"
            >
              <div className="flex-shrink-0 w-16">
                <span className="inline-block px-3 py-1 bg-terracotta-500 text-white text-sm font-semibold rounded-lg">
                  R{row.row_number}
                </span>
              </div>
              
              <div className="flex-1">
                <p className="text-sm font-medium text-slate-900">
                  {row.total_stitches} stitches
                </p>
                <p className="text-sm text-slate-600 font-mono">
                  {row.pattern_string ? row.pattern_string() : formatPattern(row)}
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Materials needed */}
      <div className="card p-8">
        <h2 className="text-xl font-semibold text-slate-900 mb-4">
          Materials Needed
        </h2>
        
        <ul className="space-y-2 text-slate-700">
          <li>• Yarn: {pattern.metadata.yarn_length_meters.toFixed(1)}m (plus 20% extra)</li>
          <li>• Hook: {config.yarn.recommended_hook_size_mm}mm</li>
          <li>• Stitch marker</li>
          <li>• Yarn needle for weaving in ends</li>
          <li>• Stuffing (polyester fiberfill)</li>
        </ul>
      </div>
    </div>
  );
}

function formatPattern(row: any): string {
  if (!row.pattern || row.pattern.length === 0) {
    return `${row.total_stitches} SC`;
  }

  const counts: Record<string, number> = {};
  
  for (const stitch of row.pattern) {
    const type = stitch.stitch_type;
    counts[type] = (counts[type] || 0) + 1;
  }

  const parts = Object.entries(counts)
    .map(([type, count]) => `${count} ${type}`)
    .join(', ');

  return parts;
}
