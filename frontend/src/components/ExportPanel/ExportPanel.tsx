import React, { useState } from 'react';
import { useApp } from '../../context/AppContext';
import { useExport } from '../../hooks/useExport';
import { Button } from '../common/Button';
import { Icon } from '../common/Icon';

const ExportPanel: React.FC = () => {
  const { pattern } = useApp();
  const { exportAsPDF, exportAsSVG, exportAsJSON, exportAsCSV } = useExport();
  const [exporting, setExporting] = useState<string | null>(null);

  const handleExportPDF = async () => {
    if (!pattern) return;
    setExporting('pdf');
    // FIX: Added 'await' because exportAsPDF is now an async function
    const result = await exportAsPDF(pattern);
    if (!result.success) {
      alert(result.error);
    }
    setExporting(null);
  };

  const handleExportSVG = async () => {
    if (!pattern) return;
    setExporting('svg');
    const result = exportAsSVG(pattern);
    if (!result.success) {
      alert(result.error);
    }
    setExporting(null);
  };

  const handleExportJSON = async () => {
    if (!pattern) return;
    setExporting('json');
    const result = exportAsJSON(pattern);
    if (!result.success) {
      alert(result.error);
    }
    setExporting(null);
  };

  const handleExportCSV = async () => {
    if (!pattern) return;
    setExporting('csv');
    const result = exportAsCSV(pattern);
    if (!result.success) {
      alert(result.error);
    }
    setExporting(null);
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-12)' }}>
      <Button 
        variant="primary" 
        fullWidth 
        onClick={handleExportPDF} 
        disabled={!pattern}
        loading={exporting === 'pdf'}
      >
        <Icon name="FileText" size={16} />
        Export as PDF
      </Button>
      <Button 
        variant="secondary" 
        fullWidth 
        onClick={handleExportSVG} 
        disabled={!pattern}
        loading={exporting === 'svg'}
      >
        <Icon name="Image" size={16} />
        Export as SVG
      </Button>
      <Button 
        variant="secondary" 
        fullWidth 
        onClick={handleExportJSON} 
        disabled={!pattern}
        loading={exporting === 'json'}
      >
        <Icon name="Download" size={16} />
        Export as JSON
      </Button>
      <Button 
        variant="secondary" 
        fullWidth 
        onClick={handleExportCSV} 
        disabled={!pattern}
        loading={exporting === 'csv'}
      >
        <Icon name="FileSpreadsheet" size={16} />
        Export as CSV
      </Button>
    </div>
  );
};

export default ExportPanel;
