import { useState, useRef, useEffect } from 'react';
import { MousePointer, Plus, Trash2, Grid3x3 } from 'lucide-react';
import * as THREE from 'three';
import type { ProfileCurve, Point2D, SplineSegment, DrawingTool } from '../types';

interface Props {
  initialPoints: Point2D[] | null;
  onChange: (profile: ProfileCurve, points: Point2D[]) => void;
  onError: (error: string) => void;
}

export default function DrawingCanvas({ initialPoints, onChange, onError }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const preview3DRef = useRef<HTMLCanvasElement>(null);
  
  // Constants for consistent scaling
  const canvasWidth = 600;
  const canvasHeight = 500;
  const marginY = 50; // Offset from top/bottom
  
  // Use FIXED scale for canvas display - don't change when config.height changes!
  // This ensures drawing stays visually stable
  const displayHeight = 10; // cm - fixed display height on canvas
  const pxPerCm = (canvasHeight - marginY * 2) / displayHeight;

  // Helper: Convert Pixels to CM for state storage
  const toCm = (p: Point2D) => ({
    x: p.x / pxPerCm,
    y: (p.y - marginY) / pxPerCm
  });

  const getInitialPoints = (): Point2D[] => {
    // Use initialPoints from parent if available (preserves across tab switches)
    if (initialPoints && initialPoints.length > 0) {
      return initialPoints;
    }
    
    // Otherwise use default shape
    const centerY = canvasHeight / 2;
    const quarterY = canvasHeight / 4;
    const threeQuarterY = 3 * canvasHeight / 4;
    
    return [
      { x: 0, y: marginY },
      { x: 80, y: quarterY },
      { x: 100, y: centerY },
      { x: 80, y: threeQuarterY },
      { x: 0, y: canvasHeight - marginY },
    ];
  };
  
  const [points, setPoints] = useState<Point2D[]>(getInitialPoints());
  const [selectedPoint, setSelectedPoint] = useState<number | null>(null);
  const [tool, setTool] = useState<DrawingTool>('select');
  const [showGrid, setShowGrid] = useState(true);
  const [isDragging, setIsDragging] = useState(false);
  
  const lastSentProfile = useRef<ProfileCurve | null>(null);
  const dragStartPos = useRef<{ x: number; y: number } | null>(null);

  const isFixedPoint = (index: number): boolean => index === 0 || index === points.length - 1;

  // Spline logic using cubic B-splines for guaranteed smooth curves
  const getSmoothSegments = (pts: Point2D[]): SplineSegment[] => {
    if (pts.length < 2) return [];
    
    const segments: SplineSegment[] = [];

    // For cubic B-spline, we need at least 4 control points
    // Pad the endpoints if needed
    const paddedPts = [
      pts[0], // Duplicate first point
      ...pts,
      pts[pts.length - 1], // Duplicate last point
    ];

    // Generate cubic B-spline segments
    for (let i = 0; i < pts.length - 1; i++) {
      const p1 = paddedPts[i + 1];
      const p2 = paddedPts[i + 2];
      const p3 = paddedPts[i + 3];

      // Cubic B-spline to Bezier conversion
      let segStart: Point2D;
      let segEnd: Point2D;
      let segControl1: Point2D;
      let segControl2: Point2D;
      
      if (i === 0) {
        // First segment: force to start at p1 with horizontal tangent
        segStart = p1;
        segControl1 = {
          x: p1.x + (p2.x - p1.x) / 3,
          y: p1.y, // Horizontal tangent
        };
        segControl2 = {
          x: (p1.x + 2 * p2.x) / 3,
          y: (p1.y + 2 * p2.y) / 3,
        };
        segEnd = {
          x: (p1.x + 4 * p2.x + p3.x) / 6,
          y: (p1.y + 4 * p2.y + p3.y) / 6,
        };
      } else if (i === pts.length - 2) {
        // Last segment: force to end at p2 with horizontal tangent
        const p0 = paddedPts[i];
        segStart = {
          x: (p0.x + 4 * p1.x + p2.x) / 6,
          y: (p0.y + 4 * p1.y + p2.y) / 6,
        };
        segControl1 = {
          x: (2 * p1.x + p2.x) / 3,
          y: (2 * p1.y + p2.y) / 3,
        };
        segControl2 = {
          x: p2.x - (p2.x - p1.x) / 3,
          y: p2.y, // Horizontal tangent
        };
        segEnd = p2;
      } else {
        // Middle segments: standard B-spline
        const p0 = paddedPts[i];
        segStart = {
          x: (p0.x + 4 * p1.x + p2.x) / 6,
          y: (p0.y + 4 * p1.y + p2.y) / 6,
        };
        segEnd = {
          x: (p1.x + 4 * p2.x + p3.x) / 6,
          y: (p1.y + 4 * p2.y + p3.y) / 6,
        };
        segControl1 = {
          x: (2 * p1.x + p2.x) / 3,
          y: (2 * p1.y + p2.y) / 3,
        };
        segControl2 = {
          x: (p1.x + 2 * p2.x) / 3,
          y: (p1.y + 2 * p2.y) / 3,
        };
      }

      segments.push({
        start: toCm(segStart),
        control1: toCm(segControl1),
        control2: toCm(segControl2),
        end: toCm(segEnd),
      });
    }
    
    return segments;
  };

  // Sync state to parent - send both profile and control points
  useEffect(() => {
    if (points.length < 2) return;
    
    const segments = getSmoothSegments(points);
    const newProfile: ProfileCurve = {
      segments,
      start_radius: segments[0].start.x,
      end_radius: segments[segments.length - 1].end.x,
    };
    
    // Send both profile and the actual control points
    lastSentProfile.current = newProfile;
    onChange(newProfile, points);
  }, [points]);

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
      
      // Use B-spline for rendering (matches getSmoothSegments)
      const paddedPts = [points[0], ...points, points[points.length - 1]];
      
      for (let i = 0; i < points.length - 1; i++) {
        const p1 = paddedPts[i + 1];
        const p2 = paddedPts[i + 2];
        const p3 = paddedPts[i + 3];
        
        // B-spline control points
        let cp1 = {
          x: (2 * p1.x + p2.x) / 3,
          y: (2 * p1.y + p2.y) / 3,
        };
        
        let cp2 = {
          x: (p1.x + 2 * p2.x) / 3,
          y: (p1.y + 2 * p2.y) / 3,
        };
        
        // Endpoint for this segment
        const endPt = {
          x: (p1.x + 4 * p2.x + p3.x) / 6,
          y: (p1.y + 4 * p2.y + p3.y) / 6,
        };

        // Override for first and last segments
        if (i === 0) {
          cp1 = { x: p1.x + (p2.x - p1.x) / 3, y: p1.y };
        }
        
        if (i === points.length - 2) {
          cp2 = { x: p2.x - (p2.x - p1.x) / 3, y: p2.y };
          ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, p2.x, p2.y);
        } else {
          ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, endPt.x, endPt.y);
        }
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

  // Helper to evaluate Bezier curve
  const evaluateBezier = (seg: SplineSegment, t: number): Point2D => {
    const t2 = t * t;
    const t3 = t2 * t;
    const mt = 1 - t;
    const mt2 = mt * mt;
    const mt3 = mt2 * mt;
    
    return {
      x: mt3 * seg.start.x + 3 * mt2 * t * seg.control1.x + 3 * mt * t2 * seg.control2.x + t3 * seg.end.x,
      y: mt3 * seg.start.y + 3 * mt2 * t * seg.control1.y + 3 * mt * t2 * seg.control2.y + t3 * seg.end.y,
    };
  };

  // 3D Preview with Three.js
  useEffect(() => {
    const canvas = preview3DRef.current;
    if (!canvas || points.length < 2) return;

    // Setup scene
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0xf8f9fa);
    
    // Calculate center of the shape for proper framing
    const segments = getSmoothSegments(points);
    let minY = Infinity, maxY = -Infinity, maxRadius = 0;
    
    for (const seg of segments) {
      minY = Math.min(minY, seg.start.y, seg.end.y);
      maxY = Math.max(maxY, seg.start.y, seg.end.y);
      maxRadius = Math.max(maxRadius, seg.start.x, seg.end.x, seg.control1.x, seg.control2.x);
    }
    
    const centerY = (minY + maxY) / 2;
    const shapeHeight = maxY - minY;
    const shapeDiameter = maxRadius * 2;
    const maxDimension = Math.max(shapeHeight, shapeDiameter);
    
    // Position camera to frame the shape
    const cameraDistance = maxDimension * 2;
    
    const camera = new THREE.PerspectiveCamera(45, 1, 0.1, 1000);
    const initialCameraPos = { 
      x: cameraDistance * 0.7, 
      y: centerY + cameraDistance * 0.5, 
      z: cameraDistance * 0.7 
    };
    camera.position.set(initialCameraPos.x, initialCameraPos.y, initialCameraPos.z);
    camera.lookAt(0, centerY, 0);
    
    const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
    renderer.setSize(500, 500);
    
    // Lighting
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambientLight);
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(5, 10, 5);
    scene.add(directionalLight);

    // Generate surface of revolution from profile curve
    const radialSegments = 32;
    const heightSegments = 100;
    
    // Sample the curve
    const curvePoints: { x: number; y: number }[] = [];
    for (const segment of segments) {
      for (let t = 0; t <= 1; t += 1 / heightSegments * segments.length) {
        const ptCm = evaluateBezier(segment, t);
        curvePoints.push({ x: ptCm.x, y: ptCm.y });
      }
    }
    
    // Create geometry for surface of revolution
    const geometry = new THREE.BufferGeometry();
    const vertices: number[] = [];
    const indices: number[] = [];
    
    // Generate vertices by rotating profile around Y axis
    for (let i = 0; i < curvePoints.length; i++) {
      const radius = curvePoints[i].x; // Already in cm
      const y = curvePoints[i].y; // Already in cm
      
      for (let j = 0; j <= radialSegments; j++) {
        const theta = (j / radialSegments) * Math.PI * 2;
        const x = radius * Math.cos(theta);
        const z = radius * Math.sin(theta);
        vertices.push(x, y, z);
      }
    }
    
    // Generate indices for triangles
    for (let i = 0; i < curvePoints.length - 1; i++) {
      for (let j = 0; j < radialSegments; j++) {
        const a = i * (radialSegments + 1) + j;
        const b = a + radialSegments + 1;
        const c = a + 1;
        const d = b + 1;
        
        indices.push(a, b, c);
        indices.push(b, d, c);
      }
    }
    
    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
    geometry.setIndex(indices);
    geometry.computeVertexNormals();
    
    // Create mesh with nice material
    const material = new THREE.MeshStandardMaterial({
      color: 0xC8603F,
      roughness: 0.5,
      metalness: 0.1,
    });
    const mesh = new THREE.Mesh(geometry, material);
    scene.add(mesh);
    
    // Add wireframe for better visibility
    const wireframe = new THREE.WireframeGeometry(geometry);
    const line = new THREE.LineSegments(wireframe);
    (line.material as THREE.LineBasicMaterial).color.set(0x8B5A3C);
    (line.material as THREE.LineBasicMaterial).opacity = 0.3;
    (line.material as THREE.LineBasicMaterial).transparent = true;
    scene.add(line);

    // Improved mouse controls with orbit camera, pan, and reset
    let isDragging3D = false;
    let isPanning = false;
    let previousMousePosition = { x: 0, y: 0 };
    
    // Use spherical coordinates for proper orbit control
    let spherical = {
      radius: Math.sqrt(initialCameraPos.x ** 2 + (initialCameraPos.y - centerY) ** 2 + initialCameraPos.z ** 2),
      theta: Math.atan2(initialCameraPos.z, initialCameraPos.x),
      phi: Math.acos((initialCameraPos.y - centerY) / Math.sqrt(initialCameraPos.x ** 2 + (initialCameraPos.y - centerY) ** 2 + initialCameraPos.z ** 2)),
    };
    let panOffset = { x: 0, y: centerY, z: 0 };
    
    const updateCamera = () => {
      const x = spherical.radius * Math.sin(spherical.phi) * Math.cos(spherical.theta);
      const y = spherical.radius * Math.cos(spherical.phi);
      const z = spherical.radius * Math.sin(spherical.phi) * Math.sin(spherical.theta);
      
      camera.position.set(x + panOffset.x, y + panOffset.y, z + panOffset.z);
      camera.lookAt(panOffset.x, panOffset.y, panOffset.z);
    };
    
    const resetCamera = () => {
      spherical.radius = Math.sqrt(initialCameraPos.x ** 2 + (initialCameraPos.y - centerY) ** 2 + initialCameraPos.z ** 2);
      spherical.theta = Math.atan2(initialCameraPos.z, initialCameraPos.x);
      spherical.phi = Math.acos((initialCameraPos.y - centerY) / Math.sqrt(initialCameraPos.x ** 2 + (initialCameraPos.y - centerY) ** 2 + initialCameraPos.z ** 2));
      panOffset = { x: 0, y: centerY, z: 0 };
      updateCamera();
    };
    
    const onMouseDown = (e: MouseEvent) => {
      if (e.button === 0) {
        isDragging3D = true;
      } else if (e.button === 2) {
        isPanning = true;
        e.preventDefault();
      }
      previousMousePosition = { x: e.offsetX, y: e.offsetY };
    };
    
    const onMouseMove = (e: MouseEvent) => {
      if (isDragging3D) {
        const deltaX = e.offsetX - previousMousePosition.x;
        const deltaY = e.offsetY - previousMousePosition.y;
        
        spherical.theta -= deltaX * 0.01;
        spherical.phi = Math.max(0.1, Math.min(Math.PI - 0.1, spherical.phi + deltaY * 0.01));
        
        updateCamera();
        previousMousePosition = { x: e.offsetX, y: e.offsetY };
      } else if (isPanning) {
        const deltaX = e.offsetX - previousMousePosition.x;
        const deltaY = e.offsetY - previousMousePosition.y;
        
        const right = new THREE.Vector3(1, 0, 0).applyQuaternion(camera.quaternion);
        const up = new THREE.Vector3(0, 1, 0).applyQuaternion(camera.quaternion);
        
        panOffset.x -= right.x * deltaX * 0.02;
        panOffset.y += up.y * deltaY * 0.02;
        panOffset.z -= right.z * deltaX * 0.02;
        
        updateCamera();
        previousMousePosition = { x: e.offsetX, y: e.offsetY };
      }
    };
    
    const onMouseUp = () => {
      isDragging3D = false;
      isPanning = false;
    };
    
    const onWheel = (e: WheelEvent) => {
      e.preventDefault();
      const minDist = maxDimension * 0.8;
      const maxDist = maxDimension * 5;
      spherical.radius = Math.max(minDist, Math.min(maxDist, spherical.radius * (1 + e.deltaY * 0.001)));
      updateCamera();
    };
    
    const onContextMenu = (e: Event) => {
      e.preventDefault();
    };
    
    const onDoubleClick = () => {
      resetCamera();
    };
    
    canvas.addEventListener('mousedown', onMouseDown);
    canvas.addEventListener('mousemove', onMouseMove);
    canvas.addEventListener('mouseup', onMouseUp);
    canvas.addEventListener('wheel', onWheel, { passive: false });
    canvas.addEventListener('contextmenu', onContextMenu);
    canvas.addEventListener('dblclick', onDoubleClick);
    
    // Animation loop
    let animationId: number;
    const animate = () => {
      animationId = requestAnimationFrame(animate);
      renderer.render(scene, camera);
    };
    animate();
    
    // Cleanup
    return () => {
      cancelAnimationFrame(animationId);
      canvas.removeEventListener('mousedown', onMouseDown);
      canvas.removeEventListener('mousemove', onMouseMove);
      canvas.removeEventListener('mouseup', onMouseUp);
      canvas.removeEventListener('wheel', onWheel);
      canvas.removeEventListener('contextmenu', onContextMenu);
      canvas.removeEventListener('dblclick', onDoubleClick);
      geometry.dispose();
      material.dispose();
      renderer.dispose();
    };
  }, [points, pxPerCm]);

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      {/* Left panel: Drawing canvas */}
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
        <p className="text-xs text-slate-500 mt-3">
          Draw the profile curve (one side only). The shape will be rotated 360° around the vertical axis.
          <br />
          <strong>Canvas scale:</strong> Fixed at 10cm height for easy drawing. The height config controls row count, not canvas display.
        </p>
      </div>

      {/* Right panel: 3D preview */}
      <div className="card p-8">
        <h2 className="text-xl font-semibold text-slate-900 mb-6">3D Preview</h2>
        <div className="border-2 border-slate-300 rounded-xl overflow-hidden bg-slate-50">
          <canvas ref={preview3DRef} width={500} height={500} className="w-full" />
        </div>
        <div className="text-xs text-slate-600 mt-3 space-y-1">
          <p><strong>Rotate:</strong> Left-click + drag</p>
          <p><strong>Pan:</strong> Right-click + drag</p>
          <p><strong>Zoom:</strong> Scroll wheel</p>
          <p><strong>Reset view:</strong> Double-click</p>
        </div>
      </div>
    </div>
  );
}
