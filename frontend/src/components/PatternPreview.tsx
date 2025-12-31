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
                <p className="text-sm text-slate-600 font-mono">
                  {formatPattern(row)}
                </p>
                <p className="text-xs text-slate-500 mt-1">
                  ({row.total_stitches} stitches total)
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

  // Special case for Row 1 (magic ring)
  if (row.row_number === 1) {
    return `${row.total_stitches} SC in magic ring`;
  }

  // Detect repeating patterns for cleaner notation
  const sequence = detectRepeatingSequence(row.pattern);
  
  if (sequence) {
    return sequence;
  }
  
  // Fall back to grouped consecutive stitches
  return formatConsecutiveGroups(row.pattern);
}

function detectRepeatingSequence(pattern: any[]): string | null {
  // Try to find a repeating sequence
  for (let seqLen = 1; seqLen <= pattern.length / 2; seqLen++) {
    if (pattern.length % seqLen === 0) {
      const firstSeq = pattern.slice(0, seqLen);
      let repeats = true;
      
      for (let i = seqLen; i < pattern.length; i += seqLen) {
        const currentSeq = pattern.slice(i, i + seqLen);
        if (!sequencesEqual(firstSeq, currentSeq)) {
          repeats = false;
          break;
        }
      }
      
      if (repeats && pattern.length / seqLen > 1) {
        const seqStr = formatConsecutiveGroups(firstSeq);
        const repeatCount = pattern.length / seqLen;
        return `[${seqStr}] repeat ${repeatCount} times`;
      }
    }
  }
  
  return null;
}

function sequencesEqual(seq1: any[], seq2: any[]): boolean {
  if (seq1.length !== seq2.length) return false;
  for (let i = 0; i < seq1.length; i++) {
    if (seq1[i].stitch_type !== seq2[i].stitch_type) return false;
  }
  return true;
}

function formatConsecutiveGroups(pattern: any[]): string {
  if (pattern.length === 0) return '';
  
  const groups: string[] = [];
  let currentType = pattern[0].stitch_type;
  let count = 1;
  
  for (let i = 1; i < pattern.length; i++) {
    if (pattern[i].stitch_type === currentType) {
      count++;
    } else {
      groups.push(count > 1 ? `${count} ${currentType}` : currentType);
      currentType = pattern[i].stitch_type;
      count = 1;
    }
  }
  
  // Add final group
  groups.push(count > 1 ? `${count} ${currentType}` : currentType);
  
  return groups.join(', ');
}
