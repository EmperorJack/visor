type ShapeCommand = (id: number) => number;
type ShapeXYCommand = (id: number, x: number, y: number) => void;
type ShapeXYZCommand = (id: number, x: number, y: number, z: number) => void;
type ShapeWHCommand = (id: number, w: number, h: number) => void;
type ShapeRGBACommand = (
  id: number,
  r: number,
  g: number,
  b: number,
  a: number,
) => void;
type ShapeHSVACommand = (
  id: number,
  h: number,
  s: number,
  v: number,
  a: number,
) => void;
type ShapeNoFillCommand = (id: number) => void;
type ShapeStrokeWeightCommand = (id: number, w: number) => void;
type ShapeTensionCommand = (id: number, t: number) => void;
type ShapePointCommand = (id: number, x: number, y: number) => void;

declare namespace Deno {
  const core: {
    ops: {
      op_draw_background_rgb: (
        id: number,
        r: number,
        g: number,
        b: number,
      ) => void;
      op_draw_background_hsv: (
        id: number,
        h: number,
        s: number,
        v: number,
      ) => void;
      op_draw_ellipse: ShapeCommand;
      op_draw_ellipse_xy: ShapeXYCommand;
      op_draw_ellipse_xyz: ShapeXYZCommand;
      op_draw_ellipse_wh: ShapeWHCommand;
      op_draw_ellipse_fill_rgba: ShapeRGBACommand;
      op_draw_ellipse_fill_hsva: ShapeHSVACommand;
      op_draw_ellipse_no_fill: ShapeNoFillCommand;
      op_draw_ellipse_stroke_rgba: ShapeRGBACommand;
      op_draw_ellipse_stroke_hsva: ShapeHSVACommand;
      op_draw_ellipse_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_rect: ShapeCommand;
      op_draw_rect_xy: ShapeXYCommand;
      op_draw_rect_xyz: ShapeXYZCommand;
      op_draw_rect_wh: ShapeWHCommand;
      op_draw_rect_fill_rgba: ShapeRGBACommand;
      op_draw_rect_fill_hsva: ShapeHSVACommand;
      op_draw_rect_no_fill: ShapeNoFillCommand;
      op_draw_rect_stroke_rgba: ShapeRGBACommand;
      op_draw_rect_stroke_hsva: ShapeHSVACommand;
      op_draw_rect_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_quad: ShapeCommand;
      op_draw_quad_xy: ShapeXYCommand;
      op_draw_quad_xyz: ShapeXYZCommand;
      op_draw_quad_points: (
        id: number,
        x1: number,
        y1: number,
        x2: number,
        y2: number,
        x3: number,
        y3: number,
        x4: number,
        y4: number,
      ) => void;
      op_draw_quad_fill_rgba: ShapeRGBACommand;
      op_draw_quad_fill_hsva: ShapeHSVACommand;
      op_draw_quad_no_fill: ShapeNoFillCommand;
      op_draw_quad_stroke_rgba: ShapeRGBACommand;
      op_draw_quad_stroke_hsva: ShapeHSVACommand;
      op_draw_quad_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_polygon: ShapeCommand;
      op_draw_polygon_xy: ShapeXYCommand;
      op_draw_polygon_xyz: ShapeXYZCommand;
      op_draw_polygon_point: ShapePointCommand;
      op_draw_polygon_fill_rgba: ShapeRGBACommand;
      op_draw_polygon_fill_hsva: ShapeHSVACommand;
      op_draw_polygon_no_fill: ShapeNoFillCommand;
      op_draw_polygon_stroke_rgba: ShapeRGBACommand;
      op_draw_polygon_stroke_hsva: ShapeHSVACommand;
      op_draw_polygon_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_polyline: ShapeCommand;
      op_draw_polyline_xyz: ShapeXYZCommand;
      op_draw_polyline_point: ShapePointCommand;
      op_draw_polyline_stroke_rgba: ShapeRGBACommand;
      op_draw_polyline_stroke_hsva: ShapeHSVACommand;
      op_draw_polyline_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_spline: ShapeCommand;
      op_draw_spline_xyz: ShapeXYZCommand;
      op_draw_spline_point: ShapePointCommand;
      op_draw_spline_stroke_rgba: ShapeRGBACommand;
      op_draw_spline_stroke_hsva: ShapeHSVACommand;
      op_draw_spline_stroke_weight: ShapeStrokeWeightCommand;
      op_draw_spline_tension: ShapeTensionCommand;
      op_draw_spline_resolution: (id: number, n: number) => void;
      op_draw_translate: (id: number, x: number, y: number) => number;
      op_draw_rotate: (id: number, radians: number) => number;
      op_draw_scale: (id: number, s: number) => number;
      op_draw_noise: (x: number, y: number, z: number) => number;
      op_draw_width: () => number;
      op_draw_height: () => number;
    };
  };
}

export default Deno.core.ops;
