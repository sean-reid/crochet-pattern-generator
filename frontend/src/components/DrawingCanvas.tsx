import { useState, useRef, useEffect } from 'react';
import { MousePointer, Plus, Trash2, Grid3x3 } from 'lucide-react';
import type { ProfileCurve, Point2D, SplineSegment, DrawingTool, AmigurumiConfig } from '../types';

interface Props {
  profile: ProfileCurve | null;
  config: AmigurumiConfig;
  onChange: (profile: ProfileCurve) => void;
  onError: (error: string) => void;
}

export default function DrawingCanvas({ profile, config, onChange, onError }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  
  // Constants for consistent scaling
  const canvasWidth = 600;
  const canvasHeight = 500;
  const marginY = 50; // Offset from top/bottom
  // Standard scale to fit the configured height into the canvas workspace
  const pxPerCm = (canvasHeight - marginY * 2) / config.total_height_cm;

  // Helper: Convert CM to Pixels for UI display
  const toPx = (p: Point2D) => ({
    x: p.x * pxPerCm,
    y: p.y * pxPerCm + marginY
  });

  // Helper: Convert Pixels to CM for state storage
  const toCm = (p: Point2D) => ({
    x: p.x / pxPerCm,
    y: (p.y - marginY) / pxPerCm
  });

  const getInitialPoints = (): Point2D[] => {
    if (profile && profile.segments.length > 0) {
      // De-normalize: Scale stored CM values back to pixels for the canvas
      const pts: Point2D[] = [toPx(profile.segments[0].start)];
      profile.segments.forEach(seg => pts.push(toPx(seg.end)));
      return pts;
    }
    return [
      { x: 0, y: marginY },    // Top magic circle
      { x: 100, y: 250 },
      { x: 0, y: canvasHeight - marginY }, // Bottom magic circle
    ];
  };
  
  const [points, setPoints] = useState<Point2D[]>(getInitialPoints());
  const [selectedPoint, setSelectedPoint] = useState<number | null>(null);
  const [tool, setTool] = useState<DrawingTool>('select');
  const [showGrid, setShowGrid] = useState(true);
  const [isDragging, setIsDragging] = useState(false);
  
  const isUpdatingFromProfile = useRef(false);
  const dragStartPos = useRef<{ x: number; y: number } | null>(null);

  const isFixedPoint = (index: number): boolean => index === 0 || index === points.length - 1;

  // Spline logic with endpoint tangent constraints
  const getSmoothSegments = (pts: Point2D[]): SplineSegment[] => {
    const segments: SplineSegment[] = [];
    const tension = 0.33;

    for (let i = 0; i < pts.length - 1; i++) {
      const p0 = pts[i - 1] || pts[i];
      const p1 = pts[i];
      const p2 = pts[i + 1];
      const p3 = pts[i + 2] || pts[i + 1];

      let cp1Raw = {
        x: p1.x + (p2.x - p0.x) * tension,
        y: p1.y + (p2.y - p0.y) * tension
      };
      let cp2Raw = {
        x: p2.x - (p3.x - p1.x) * tension,
        y: p2.y - (p3.y - p1.y) * tension
      };

      // Enforce tangents normal (perpendicular) to rotation axis at magic circles
      if (i === 0) cp1Raw.y = p1.y; 
      if (i === pts.length - 2) cp2Raw.y = p2.y;

      segments.push({
        start: toCm(p1),
        control1: toCm(cp1Raw),
        control2: toCm(cp2Raw),
        end: toCm(p2),
      });
    }
    return segments;
  };

  // Sync state to parent
  useEffect(() => {
    if (points.length < 2 || isUpdatingFromProfile.current) return;
    
    const segments = getSmoothSegments(points);
    const newProfile: ProfileCurve = {
      segments,
      start_radius: segments[0].start.x,
      end_radius: segments[segments.length - 1].end.x,
    };
    onChange(newProfile);
  }, [points, config.total_height_cm, onChange]);

  // Sync points if profile changes externally (Tab switching)
  useEffect(() => {
    if (!profile || profile.segments.length === 0) return;
    
    const ptsFromProfile: Point2D[] = [toPx(profile.segments[0].start)];
    profile.segments.forEach(seg => ptsFromProfile.push(toPx(seg.end)));
    
    const pointsChanged = points.length !== ptsFromProfile.length ||
      points.some((p, i) => Math.abs(p.x - ptsFromProfile[i].x) > 0.5 || Math.abs(p.y - ptsFromProfile[i].y) > 0.5);
    
    if (pointsChanged) {
      isUpdatingFromProfile.current = true;
      setPoints(ptsFromProfile);
      setTimeout(() => { isUpdatingFromProfile.current = false; }, 0);
    }
  }, [profile, config.total_height_cm]);

  // Canvas Drawing
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.fillStyle = '#FEFDFB';
    ctx.fillRect(0, 0, canvasWidth, canvasHeight);

    if (showGrid) {
      ctx.strokeStyle = '#E8E1D4';
      ctx.lineWidth = 1;
      for (let x = 0; x <= canvasWidth; x += 50) { ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, canvasHeight); ctx.stroke(); }
      for (let y = 0; y <= canvasHeight; y += 50) { ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(canvasWidth, y); ctx.stroke(); }
    }

    ctx.strokeStyle = '#64748B';
    ctx.lineWidth = 3;
    ctx.beginPath(); ctx.moveTo(0, 0); ctx.lineTo(0, canvasHeight); ctx.stroke();

    if (points.length >= 2) {
      ctx.strokeStyle = '#C8603F';
      ctx.lineWidth = 3;
      ctx.beginPath();
      ctx.moveTo(points[0].x, points[0].y);
      
      const tension = 0.33;
      for (let i = 0; i < points.length - 1; i++) {
        const p0 = points[i - 1] || points[i];
        const p1 = points[i];
        const p2 = points[i + 1];
        const p3 = points[i + 2] || points[i + 1];

        let cp1 = { x: p1.x + (p2.x - p0.x) * tension, y: p1.y + (p2.y - p0.y) * tension };
        let cp2 = { x: p2.x - (p3.x - p1.x) * tension, y: p2.y - (p3.y - p1.y) * tension };
        
        if (i === 0) cp1.y = p1.y;
        if (i === points.length - 2) cp2.y = p2.y;

        ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, p2.x, p2.y);
      }
      ctx.stroke();
    }

    points.forEach((point, index) => {
      const isSelected = index === selectedPoint;
      ctx.fillStyle = isSelected ? '#C8603F' : (isFixedPoint(index) ? '#8B5A3C' : '#FFFFFF');
      ctx.strokeStyle = '#C8603F';
      ctx.lineWidth = 2;
      if (isFixedPoint(index)) {
        const size = isSelected ? 10 : 8;
        ctx.fillRect(point.x - size/2, point.y - size/2, size, size);
        ctx.strokeRect(point.x - size/2, point.y - size/2, size, size);
      } else {
        ctx.beginPath(); ctx.arc(point.x, point.y, isSelected ? 8 : 6, 0, Math.PI * 2); ctx.fill(); ctx.stroke();
      }
    });
  }, [points, selectedPoint, showGrid]);

  // Input Handlers
  const handleCanvasClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (dragStartPos.current) {
      const rect = canvasRef.current!.getBoundingClientRect();
      if (Math.hypot(e.clientX - rect.left - dragStartPos.current.x, e.clientY - rect.top - dragStartPos.current.y) > 3) {
        dragStartPos.current = null; return;
      }
    }
    dragStartPos.current = null;
    const rect = canvasRef.current!.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (tool === 'add') {
      const newPoints = [...points];
      let insertIndex = newPoints.length;
      for (let i = 0; i < newPoints.length; i++) { if (y < newPoints[i].y) { insertIndex = i; break; } }
      newPoints.splice(insertIndex, 0, { x, y });
      setPoints(newPoints);
    } else if (tool === 'delete') {
      for (let i = 0; i < points.length; i++) {
        if (Math.hypot(points[i].x - x, points[i].y - y) < 15) {
          if (isFixedPoint(i)) { onError('Cannot delete magic circle points'); return; }
          if (points.length > 3) { setPoints(points.filter((_, idx) => idx !== i)); setSelectedPoint(null); }
          else { onError('Profile must have at least 3 points'); }
          return;
        }
      }
    }
  };

  const handleCanvasMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (tool !== 'select') return;
    const rect = canvasRef.current!.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    dragStartPos.current = { x, y };
    for (let i = 0; i < points.length; i++) {
      if (Math.hypot(points[i].x - x, points[i].y - y) < 15) { setSelectedPoint(i); setIsDragging(true); return; }
    }
    setSelectedPoint(null);
  };

  const handleCanvasMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isDragging || selectedPoint === null || tool !== 'select') return;
    const rect = canvasRef.current!.getBoundingClientRect();
    const x = isFixedPoint(selectedPoint) ? 0 : Math.max(0, Math.min(canvasWidth, e.clientX - rect.left));
    const y = Math.max(0, Math.min(canvasHeight, e.clientY - rect.top));
    const newPoints = [...points];
    newPoints[selectedPoint] = { x, y };
    setPoints(newPoints);
  };

  return (
    <div className="card p-8">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold text-slate-900">Draw Profile</h2>
        <div className="flex items-center gap-2">
          <button onClick={() => setTool('select')} className={`p-2 rounded-lg ${tool === 'select' ? 'bg-terracotta-500 text-white' : 'bg-slate-100'}`}><MousePointer size={20} /></button>
          <button onClick={() => setTool('add')} className={`p-2 rounded-lg ${tool === 'add' ? 'bg-terracotta-500 text-white' : 'bg-slate-100'}`}><Plus size={20} /></button>
          <button onClick={() => setTool('delete')} className={`p-2 rounded-lg ${tool === 'delete' ? 'bg-terracotta-500 text-white' : 'bg-slate-100'}`}><Trash2 size={20} /></button>
          <div className="w-px h-8 bg-slate-300 mx-2" />
          <button onClick={() => setShowGrid(!showGrid)} className={`p-2 rounded-lg ${showGrid ? 'bg-terracotta-500 text-white' : 'bg-slate-100'}`}><Grid3x3 size={20} /></button>
        </div>
      </div>
      <div className="border-2 border-slate-300 rounded-xl overflow-hidden">
        <canvas ref={canvasRef} width={canvasWidth} height={canvasHeight} onClick={handleCanvasClick} onMouseDown={handleCanvasMouseDown}
          onMouseMove={handleCanvasMouseMove} onMouseUp={() => setIsDragging(false)} onMouseLeave={() => setIsDragging(false)} className="cursor-crosshair" />
      </div>
    </div>
  );
}
