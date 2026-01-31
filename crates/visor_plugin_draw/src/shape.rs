use crate::draw_plugin::DrawId;

pub(crate) struct Shape {
    pub kind: ShapeKind,
    pub draw_id: DrawId,
    pub commands: Vec<ShapeCommand>,
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum ShapeKind {
    Ellipse,
    Rect,
    Quad,
    Polygon,
    Polyline,
    Spline,
    Path,
}

pub(crate) enum ShapeCommand {
    Xy {
        x: f32,
        y: f32,
    },
    Xyz {
        x: f32,
        y: f32,
        z: f32,
    },
    Wh {
        w: f32,
        h: f32,
    },
    Point {
        x: f32,
        y: f32,
    },
    QuadPoints {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
    },
    FillRgba {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },
    FillHsva {
        h: f32,
        s: f32,
        v: f32,
        a: f32,
    },
    NoFill,
    StrokeRgba {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },
    StrokeHsva {
        h: f32,
        s: f32,
        v: f32,
        a: f32,
    },
    StrokeWeight {
        w: f32,
    },
    Tension {
        t: f32,
    },
    Resolution {
        n: u32,
    },
}
