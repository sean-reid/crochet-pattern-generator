import { useCallback } from 'react';
import type { CrochetPattern } from '../types/pattern';

export const useExport = () => {
  const exportAsJSON = useCallback((pattern: CrochetPattern, filename: string = 'pattern.json') => {
    try {
      const json = JSON.stringify(pattern, null, 2);
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      
      return { success: true };
    } catch (error) {
      console.error('JSON export error:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to export JSON',
      };
    }
  }, []);

  const exportAsPDF = useCallback((pattern: CrochetPattern, filename: string = 'pattern.pdf') => {
    // This would use jsPDF to generate a PDF
    // Implementation would include:
    // - Title page with pattern metadata
    // - Stitch diagram (from pattern.diagram.svg)
    // - Row-by-row instructions (from pattern.instructions)
    // - Materials list
    // - Abbreviations guide
    
    console.error('PDF export not implemented - requires jsPDF integration');
    console.log('Would export pattern:', pattern.metadata.stitchCount, 'stitches to', filename);
    return {
      success: false,
      error: 'PDF export requires jsPDF library integration. Please implement in ExportPanel component.',
    };
  }, []);

  const exportAsSVG = useCallback((pattern: CrochetPattern, filename: string = 'pattern.svg') => {
    try {
      // Use the SVG from the pattern diagram
      if (!pattern.diagram.svg) {
        throw new Error('No diagram available in pattern');
      }

      const blob = new Blob([pattern.diagram.svg], { type: 'image/svg+xml' });
      const url = URL.createObjectURL(blob);
      
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      
      return { success: true };
    } catch (error) {
      console.error('SVG export error:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to export SVG',
      };
    }
  }, []);

  const exportAsCSV = useCallback((pattern: CrochetPattern, filename: string = 'pattern.csv') => {
    try {
      // Export stitch data as CSV
      const headers = ['ID', 'Type', 'Row', 'Position X', 'Position Y', 'Position Z', 'Connections'];
      const rows = pattern.stitches.map(stitch => [
        stitch.id,
        stitch.type,
        stitch.row,
        stitch.position3D.x,
        stitch.position3D.y,
        stitch.position3D.z,
        stitch.connections.join(';'),
      ]);

      const csv = [
        headers.join(','),
        ...rows.map(row => row.join(',')),
      ].join('\n');

      const blob = new Blob([csv], { type: 'text/csv' });
      const url = URL.createObjectURL(blob);
      
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      
      return { success: true };
    } catch (error) {
      console.error('CSV export error:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to export CSV',
      };
    }
  }, []);

  return {
    exportAsJSON,
    exportAsPDF,
    exportAsSVG,
    exportAsCSV,
  };
};
