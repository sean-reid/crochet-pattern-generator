import { useState, useRef, useEffect } from 'react';
import { Cursor, Plus, Trash2, Grid3x3 } from 'lucide-react';
import type { ProfileCurve, Point2D, SplineSegment, DrawingTool } from '../types';

interface Props {
  profile: ProfileCurve | null;
  onChange: (profile: ProfileCurve) => void;
  onError: (error: string) => void;
}

export default function DrawingCanvas({ profile, onChange, onError }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [points, setPoints] = useState<Point2D[]>([
    { x: 50, y: 50 },
    { x: 50, y: 450 },
  ]);
  const [selectedPoint, setSelectedPoint] = useState<number | null>(null);
  const [tool, setTool] = useState<DrawingTool>('select');
  const [showGrid, setShowGrid] = useState(true);
  const [isDragging, setIsDragging] = useState(false);

  const canvasWidth = 600;
  const canvasHeight = 500;

  // Update profile when points change
  useEffect(() => {
    if (points.length < 2) return;

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
  }, [points]);

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

    // Draw axis line
    ctx.strokeStyle = '#94A3B8';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(0, 0);
    ctx.lineTo(0, canvasHeight);
    ctx.stroke();

    // Draw curve
    if (points.length >= 2) {
      ctx.strokeStyle = '#C8603F';
      ctx.lineWidth = 3;
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
      
      ctx.fillStyle = isSelected ? '#C8603F' : '#FFFFFF';
      ctx.strokeStyle = '#C8603F';
      ctx.lineWidth = 2;
      
      ctx.beginPath();
      ctx.arc(point.x, point.y, isSelected ? 8 : 6, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
    });
  }, [points, selectedPoint, showGrid]);

  const handleCanvasClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
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
          if (points.length > 2) {
            const newPoints = points.filter((_, idx) => idx !== i);
            setPoints(newPoints);
            setSelectedPoint(null);
          } else {
            onError('Profile must have at least 2 points');
          }
          return;
        }
      }
    } else if (tool === 'select') {
      // Select point
      const clickRadius = 15;
      let found = false;
      
      for (let i = 0; i < points.length; i++) {
        const dx = points[i].x - x;
        const dy = points[i].y - y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        
        if (dist < clickRadius) {
          setSelectedPoint(i);
          setIsDragging(true);
          found = true;
          return;
        }
      }
      
      if (!found) {
        setSelectedPoint(null);
      }
    }
  };

  const handleCanvasMouseMove = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isDragging || selectedPoint === null || tool !== 'select') return;

    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = Math.max(0, Math.min(canvasWidth, e.clientX - rect.left));
    const y = Math.max(0, Math.min(canvasHeight, e.clientY - rect.top));

    const newPoints = [...points];
    newPoints[selectedPoint] = { x, y };
    setPoints(newPoints);
  };

  const handleCanvasMouseUp = () => {
    setIsDragging(false);
  };

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
            <Cursor size={20} />
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
        Draw the profile of your amigurumi. The left edge is the axis of rotation.
        Click to add points, drag to move them.
      </p>

      <div className="border-2 border-slate-300 rounded-xl overflow-hidden">
        <canvas
          ref={canvasRef}
          width={canvasWidth}
          height={canvasHeight}
          onClick={handleCanvasClick}
          onMouseMove={handleCanvasMouseMove}
          onMouseUp={handleCanvasMouseUp}
          onMouseLeave={handleCanvasMouseUp}
          className="cursor-crosshair"
        />
      </div>

      <div className="mt-4 text-sm text-slate-600">
        <p>Points: {points.length}</p>
        <p>Selected tool: {tool}</p>
      </div>
    </div>
  );
}
