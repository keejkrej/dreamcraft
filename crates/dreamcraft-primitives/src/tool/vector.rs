use dreamcraft_core::{Color, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VectorNode {
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    QuadBezierTo { cx: f64, cy: f64, x: f64, y: f64 },
    CubicBezierTo { cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64 },
    ArcTo { rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool, x: f64, y: f64 },
    Close,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorPath {
    pub nodes: Vec<VectorNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StrokeCap {
    #[default]
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StrokeJoin {
    #[default]
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrokeStyle {
    pub color: Color,
    pub width: f64,
    pub cap: StrokeCap,
    pub join: StrokeJoin,
    pub miter_limit: f64,
    pub dash_array: Vec<f64>,
    pub dash_offset: f64,
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            width: 1.0,
            cap: StrokeCap::Butt,
            join: StrokeJoin::Miter,
            miter_limit: 4.0,
            dash_array: Vec::new(),
            dash_offset: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GradientStop {
    pub offset: f32, // 0.0 .. 1.0
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VectorPaint {
    None,
    Solid(Color),
    LinearGradient {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        stops: Vec<GradientStop>,
    },
    RadialGradient {
        cx: f64,
        cy: f64,
        r: f64,
        stops: Vec<GradientStop>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VectorShapeKind {
    Path(VectorPath),
    Rect { x: f64, y: f64, w: f64, h: f64, rx: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
    Ellipse { cx: f64, cy: f64, rx: f64, ry: f64 },
    Polygon { points: Vec<(f64, f64)> },
    Text { x: f64, y: f64, text: String, font_family: String, font_size: f64 },
    Group(Vec<VectorElement>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorElement {
    pub id: Id,
    pub name: String,
    pub kind: VectorShapeKind,
    pub fill: Option<Color>, // Backwards-compatibility
    pub paint: VectorPaint,
    pub stroke: Option<Color>,
    pub stroke_width: f64,
    pub stroke_style: StrokeStyle,
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: Id,
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub elements: Vec<VectorElement>,
}

impl VectorDocument {
    pub fn new(name: impl Into<String>, width: f64, height: f64) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            width,
            height,
            elements: Vec::new(),
        }
    }

    pub fn add_rect(&mut self, name: impl Into<String>, x: f64, y: f64, w: f64, h: f64, fill: Option<Color>, stroke: Option<Color>) -> Id {
        let el = VectorElement {
            id: Id::new(),
            name: name.into(),
            kind: VectorShapeKind::Rect { x, y, w, h, rx: 0.0 },
            fill,
            paint: fill.map(VectorPaint::Solid).unwrap_or(VectorPaint::None),
            stroke,
            stroke_width: 2.0,
            stroke_style: StrokeStyle {
                color: stroke.unwrap_or(Color::BLACK),
                width: 2.0,
                ..Default::default()
            },
            opacity: 1.0,
        };
        let id = el.id;
        self.elements.push(el);
        id
    }

    pub fn add_circle(&mut self, name: impl Into<String>, cx: f64, cy: f64, r: f64, fill: Option<Color>, stroke: Option<Color>) -> Id {
        let el = VectorElement {
            id: Id::new(),
            name: name.into(),
            kind: VectorShapeKind::Circle { cx, cy, r },
            fill,
            paint: fill.map(VectorPaint::Solid).unwrap_or(VectorPaint::None),
            stroke,
            stroke_width: 2.0,
            stroke_style: StrokeStyle {
                color: stroke.unwrap_or(Color::BLACK),
                width: 2.0,
                ..Default::default()
            },
            opacity: 1.0,
        };
        let id = el.id;
        self.elements.push(el);
        id
    }

    pub fn add_path(&mut self, name: impl Into<String>, nodes: Vec<VectorNode>, fill: Option<Color>, stroke: Option<Color>) -> Id {
        let el = VectorElement {
            id: Id::new(),
            name: name.into(),
            kind: VectorShapeKind::Path(VectorPath { nodes }),
            fill,
            paint: fill.map(VectorPaint::Solid).unwrap_or(VectorPaint::None),
            stroke,
            stroke_width: 2.0,
            stroke_style: StrokeStyle {
                color: stroke.unwrap_or(Color::BLACK),
                width: 2.0,
                ..Default::default()
            },
            opacity: 1.0,
        };
        let id = el.id;
        self.elements.push(el);
        id
    }

    /// Export elements as SVG markup with clean path data.
    pub fn to_svg(&self) -> String {
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n",
            self.width, self.height, self.width, self.height
        );

        for el in &self.elements {
            let fill_attr = match el.fill {
                Some(c) => format!("fill=\"{}\"", c.to_hex()),
                None => "fill=\"none\"".into(),
            };
            let stroke_attr = match el.stroke {
                Some(c) => format!("stroke=\"{}\" stroke-width=\"{}\"", c.to_hex(), el.stroke_width),
                None => "stroke=\"none\"".into(),
            };

            match &el.kind {
                VectorShapeKind::Rect { x, y, w, h, rx } => {
                    svg.push_str(&format!(
                        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" {} {} />\n",
                        x, y, w, h, rx, fill_attr, stroke_attr
                    ));
                }
                VectorShapeKind::Circle { cx, cy, r } => {
                    svg.push_str(&format!(
                        "  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" {} {} />\n",
                        cx, cy, r, fill_attr, stroke_attr
                    ));
                }
                VectorShapeKind::Ellipse { cx, cy, rx, ry } => {
                    svg.push_str(&format!(
                        "  <ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" {} {} />\n",
                        cx, cy, rx, ry, fill_attr, stroke_attr
                    ));
                }
                VectorShapeKind::Polygon { points } => {
                    let pts: String = points.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<_>>().join(" ");
                    svg.push_str(&format!(
                        "  <polygon points=\"{}\" {} {} />\n",
                        pts, fill_attr, stroke_attr
                    ));
                }
                VectorShapeKind::Text { x, y, text, font_family, font_size } => {
                    svg.push_str(&format!(
                        "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" {}>{}</text>\n",
                        x, y, font_family, font_size, fill_attr, text
                    ));
                }
                VectorShapeKind::Path(p) => {
                    let mut d = String::new();
                    for node in &p.nodes {
                        match node {
                            VectorNode::MoveTo { x, y } => d.push_str(&format!("M {} {} ", x, y)),
                            VectorNode::LineTo { x, y } => d.push_str(&format!("L {} {} ", x, y)),
                            VectorNode::QuadBezierTo { cx, cy, x, y } => {
                                d.push_str(&format!("Q {} {}, {} {} ", cx, cy, x, y));
                            }
                            VectorNode::CubicBezierTo { cp1x, cp1y, cp2x, cp2y, x, y } => {
                                d.push_str(&format!("C {} {}, {} {}, {} {} ", cp1x, cp1y, cp2x, cp2y, x, y));
                            }
                            VectorNode::ArcTo { rx, ry, x_axis_rotation, large_arc, sweep, x, y } => {
                                d.push_str(&format!(
                                    "A {} {} {} {} {} {} {} ",
                                    rx, ry, x_axis_rotation,
                                    if *large_arc { 1 } else { 0 },
                                    if *sweep { 1 } else { 0 },
                                    x, y
                                ));
                            }
                            VectorNode::Close => d.push_str("Z "),
                        }
                    }
                    svg.push_str(&format!("  <path d=\"{}\" {} {} />\n", d.trim(), fill_attr, stroke_attr));
                }
                VectorShapeKind::Group(_) => {}
            }
        }

        svg.push_str("</svg>");
        svg
    }
}
