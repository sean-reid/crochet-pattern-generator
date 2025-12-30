import { useState, useRef, useEffect } from 'react';
import { MousePointer, Plus, Trash2, Grid3x3 } from 'lucide-react';
import type { ProfileCurve, Point2D, SplineSegment, DrawingTool } from '../types';

interface Props {
  profile: ProfileCurve | null;
  onChange: (profile: ProfileCurve) => void;
  onError: (error: string) => void;
}

export default function DrawingCanvas({ profile, onChange, onError }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  
  // Initialize points from profile if provided, otherwise use defaults
  const getInitialPoints = (): Point2D[] => {
    if (profile && profile.segments.length > 0) {
      const pointsFromProfile: Point2D[] = [profile.segments[0].start];
      profile.segments.forEach(seg => pointsFromProfile.push(seg.end));
      return pointsFromProfile;
    }
    return [
      { x: 0, y: 50 },    // Top magic circle point (fixed at axis)
      { x: 100, y: 250 },
      { x: 0, y: 450 },   // Bottom magic circle point (fixed at axis)
    ];
  };
  
  const [points, setPoints] = useState<Point2D[]>(getInitialPoints());
  const [selectedPoint, setSelectedPoint] = useState<number | null>(null);
  const [tool, setTool] = useState<DrawingTool>('select');
  const [showGrid, setShowGrid] = useState(true);
  const [isDragging, setIsDragging] = useState(false);
  
  // Track if we're updating from external profile to avoid circular updates
  const isUpdatingFromProfile = useRef(false);
  // Track mouse position to distinguish click from drag
  const dragStartPos = useRef<{ x: number; y: number } | null>(null);

  const canvasWidth = 600;
  const canvasHeight = 500;
  
  // Magic circle points are first and last
  const isFixedPoint = (index: number): boolean => {
    return index === 0 || index === points.length - 1;
  };

  // Update profile when points change (but not when syncing from profile prop)
  useEffect(() => {
    if (points.length < 2 || isUpdatingFromProfile.current) return;

    const segments: SplineSegment[] = [];
    
    for (let i = 0; i < points.length - 1; i++) {
      const start = points[i];
      const end = points[i + 1];
      
      // Create smooth Bézier curve between points
      const dx = end.x - start.x;
      const dy = end.y - start.y;
      
      segments.push({
        start,
        control1: {
          x: start.x + dx * 0.33,
          y: start.y + dy * 0.33,
        },
        control2: {
          x: start.x + dx * 0.67,
          y: start.y + dy * 0.67,
        },
        end,
      });
    }

    const newProfile: ProfileCurve = {
      segments,
      start_radius: points[0].x / 10,
      end_radius: points[points.length - 1].x / 10,
    };

    onChange(newProfile);
  }, [points, onChange]);

  // Sync points with profile prop changes (when returning to draw tab)
  // Only if profile has actually changed externally
  useEffect(() => {
    if (!profile || profile.segments.length === 0) return;
    
    // Extract points from profile
    const pointsFromProfile: Point2D[] = [profile.segments[0].start];
    profile.segments.forEach(seg => pointsFromProfile.push(seg.end));
    
    // Check if points are actually different to avoid unnecessary updates
    const pointsChanged = 
      points.length !== pointsFromProfile.length ||
      points.some((p, i) => 
        Math.abs(p.x - pointsFromProfile[i].x) > 0.1 ||
        Math.abs(p.y - pointsFromProfile[i].y) > 0.1
      );
    
    if (pointsChanged) {
      isUpdatingFromProfile.current = true;
      setPoints(pointsFromProfile);
      // Reset flag after state update completes
      setTimeout(() => {
        isUpdatingFromProfile.current = false;
      }, 0);
    }
  }, [profile]); // Deliberately not including points to avoid circular dependency

  // Draw canvas
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = '#FEFDFB';
    ctx.fillRect(0, 0, canvasWidth, canvasHeight);

    // Draw grid
    if (showGrid) {
      ctx.strokeStyle = '#E8E1D4';
      ctx.lineWidth = 1;
      
      for (let x = 0; x <= canvasWidth; x += 50) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, canvasHeight);
        ctx.stroke();
      }
      
      for (let y = 0; y <= canvasHeight; y += 50) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(canvasWidth, y);
        ctx.stroke();
      }
    }

    // Draw axis line (more prominent)
    ctx.strokeStyle = '#64748B';
    ctx.lineWidth = 3;
    ctx.setLineDash([]);
    ctx.beginPath();
    ctx.moveTo(0, 0);
    ctx.lineTo(0, canvasHeight);
    ctx.stroke();
    
    // Add axis label
    ctx.fillStyle = '#64748B';
    ctx.font = 'bold 12px Inter, sans-serif';
    ctx.fillText('Axis of Rotation', 10, 20);

    // Draw curve
    if (points.length >= 2) {
      ctx.strokeStyle = '#C8603F';
      ctx.lineWidth = 3;
      ctx.setLineDash([]);
      ctx.beginPath();
      ctx.moveTo(points[0].x, points[0].y);
      
      for (let i = 0; i < points.length - 1; i++) {
        const start = points[i];
        const end = points[i + 1];
        const cp1x = start.x + (end.x - start.x) * 0.33;
        const cp1y = start.y + (end.y - start.y) * 0.33;
        const cp2x = start.x + (end.x - start.x) * 0.67;
        const cp2y = start.y + (end.y - start.y) * 0.67;
        
        ctx.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, end.x, end.y);
      }
      
      ctx.stroke();
    }

    // Draw points
    points.forEach((point, index) => {
      const isSelected = index === selectedPoint;
      const isFixed = isFixedPoint(index);
      
      // Fixed points (magic circles) are drawn as squares
      if (isFixed) {
        ctx.fillStyle = isSelected ? '#C8603F' : '#8B5A3C';
        ctx.strokeStyle = '#C8603F';
        ctx.lineWidth = 2;
        
        const size = isSelected ? 10 : 8;
        ctx.fillRect(point.x - size/2, point.y - size/2, size, size);
        ctx.strokeRect(point.x - size/2, point.y - size/2, size, size);
      } else {
        // Regular points are circles
        ctx.fillStyle = isSelected ? '#C8603F' : '#FFFFFF';
        ctx.strokeStyle = '#C8603F';
        ctx.lineWidth = 2;
        
        ctx.beginPath();
        ctx.arc(point.x, point.y, isSelected ? 8 : 6, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
      }
    });
  }, [points, selectedPoint, showGrid]);

  const handleCanvasClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    // Don't process clicks that were actually drags
    // Check if mouse moved more than 3 pixels from start position
    if (dragStartPos.current) {
      const canvas = canvasRef.current;
      if (canvas) {
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        const dx = x - dragStartPos.current.x;
        const dy = y - dragStartPos.current.y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        
        // If moved more than 3 pixels, it was a drag, not a click
        if (dist > 3) {
          dragStartPos.current = null;
          return;
        }
      }
    }
    dragStartPos.current = null;
    
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (tool === 'add') {
      // Add new point
      const newPoints = [...points];
      
      // Find where to insert (sort by y)
      let insertIndex = newPoints.length;
      for (let i = 0; i < newPoints.length; i++) {
        if (y < newPoints[i].y) {
          insertIndex = i;
          break;
        }
      }
      
      newPoints.splice(insertIndex, 0, { x, y });
      setPoints(newPoints);
    } else if (tool === 'delete') {
      // Find and delete closest point
      const clickRadius = 15;
      for (let i = 0; i < points.length; i++) {
        const dx = points[i].x - x;
        const dy = points[i].y - y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        
        if (dist < clickRadius) {
          // Cannot delete magic circle points
          if (isFixedPoint(i)) {
            onError('Cannot delete magic circle points (top and bottom)');
            return;
          }
          
          if (points.length > 3) {  // Need at least 3 points: 2 magic circles + 1 middle
            const newPoints = points.filter((_, idx) => idx !== i);
            setPoints(newPoints);
            setSelectedPoint(null);
          } else {
            onError('Profile must have at least 3 points (including 2 magic circle points)');
          }
          return;
        }
      }
    }
  };

  const handleCanvasMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (tool !== 'select') return;
    
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    // Track where drag started
    dragStartPos.current = { x, y };

    // Select point for dragging
    const clickRadius = 15;
    
    for (let i = 0; i < points.length; i++) {
      const dx = points[i].x - x;
      const dy = points[i].y - y;
      const dist = Math.sqrt(dx * dx + dy * dy);
      
      if (dist < clickRadius) {
        setSelectedPoint(i);
        setIsDragging(true);
        return;
      }
    }
    
    // Clicked on empty space - deselect
    setSelectedPoint(null);
  };

  const handleCanvasMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isDragging || selectedPoint === null || tool !== 'select') return;

    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    let x = Math.max(0, Math.min(canvasWidth, e.clientX - rect.left));
    let y = Math.max(0, Math.min(canvasHeight, e.clientY - rect.top));
    
    // Fixed points (magic circles) can only move vertically and must stay on axis
    if (isFixedPoint(selectedPoint)) {
      x = 0;
    }

    const newPoints = [...points];
    newPoints[selectedPoint] = { x, y };
    setPoints(newPoints);
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };
  
  // Add global mouse up handler to catch when mouse is released outside canvas
  useEffect(() => {
    const handleGlobalMouseUp = () => {
      setIsDragging(false);
    };
    
    document.addEventListener('mouseup', handleGlobalMouseUp);
    return () => {
      document.removeEventListener('mouseup', handleGlobalMouseUp);
    };
  }, []);

  return (
    <div className="card p-8">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold text-slate-900">Draw Profile</h2>
        
        <div className="flex items-center gap-2">
          <button
            onClick={() => setTool('select')}
            className={`p-2 rounded-lg ${
              tool === 'select'
                ? 'bg-terracotta-500 text-white'
                : 'bg-slate-100 text-slate-700 hover:bg-slate-200'
            }`}
            title="Select and move points"
          >
            <MousePointer size={20} />
          </button>
          
          <button
            onClick={() => setTool('add')}
            className={`p-2 rounded-lg ${
              tool === 'add'
                ? 'bg-terracotta-500 text-white'
                : 'bg-slate-100 text-slate-700 hover:bg-slate-200'
            }`}
            title="Add new point"
          >
            <Plus size={20} />
          </button>
          
          <button
            onClick={() => setTool('delete')}
            className={`p-2 rounded-lg ${
              tool === 'delete'
                ? 'bg-terracotta-500 text-white'
                : 'bg-slate-100 text-slate-700 hover:bg-slate-200'
            }`}
            title="Delete point"
          >
            <Trash2 size={20} />
          </button>
          
          <div className="w-px h-8 bg-slate-300 mx-2" />
          
          <button
            onClick={() => setShowGrid(!showGrid)}
            className={`p-2 rounded-lg ${
              showGrid
                ? 'bg-terracotta-500 text-white'
                : 'bg-slate-100 text-slate-700 hover:bg-slate-200'
            }`}
            title="Toggle grid"
          >
            <Grid3x3 size={20} />
          </button>
        </div>
      </div>

      <p className="text-sm text-slate-600 mb-4">
        Draw the profile of your amigurumi. The <strong>left edge (axis of rotation)</strong> is where the magic circles will be.
        The <strong>top and bottom points</strong> (squares) are fixed at the axis and can only move vertically.
        Click to add points, drag to move them.
      </p>

      <div className="border-2 border-slate-300 rounded-xl overflow-hidden">
        <canvas
          ref={canvasRef}
          width={canvasWidth}
          height={canvasHeight}
          onClick={handleCanvasClick}
          onMouseDown={handleCanvasMouseDown}
          onMouseMove={handleCanvasMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          className="cursor-crosshair"
        />
      </div>

      <div className="mt-4 text-sm text-slate-600">
        <p>Points: {points.length} (including {isFixedPoint(0) && isFixedPoint(points.length - 1) ? '2' : '0'} fixed magic circle points)</p>
        <p>Selected tool: {tool}</p>
      </div>
    </div>
  );
}
