use dreamcraft_core::{Color, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VectorNode {
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    CubicBezierTo { cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64 },
    Close,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorPath {
    pub nodes: Vec<VectorNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VectorShapeKind {
    Path(VectorPath),
    Rect { x: f64, y: f64, w: f64, h: f64, rx: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
    Text { x: f64, y: f64, text: String, font_size: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorElement {
    pub id: Id,
    pub name: String,
    pub kind: VectorShapeKind,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: f64,
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
            stroke,
            stroke_width: 2.0,
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
            stroke,
            stroke_width: 2.0,
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
            stroke,
            stroke_width: 2.0,
        };
        let id = el.id;
        self.elements.push(el);
        id
    }

    /// Export elements as SVG markup.
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
                VectorShapeKind::Text { x, y, text, font_size } => {
                    svg.push_str(&format!(
                        "  <text x=\"{}\" y=\"{}\" font-size=\"{}\" {}>{}</text>\n",
                        x, y, font_size, fill_attr, text
                    ));
                }
                VectorShapeKind::Path(p) => {
                    let mut d = String::new();
                    for node in &p.nodes {
                        match node {
                            VectorNode::MoveTo { x, y } => d.push_str(&format!("M {} {} ", x, y)),
                            VectorNode::LineTo { x, y } => d.push_str(&format!("L {} {} ", x, y)),
                            VectorNode::CubicBezierTo { cp1x, cp1y, cp2x, cp2y, x, y } => {
                                d.push_str(&format!("C {} {}, {} {}, {} {} ", cp1x, cp1y, cp2x, cp2y, x, y));
                            }
                            VectorNode::Close => d.push_str("Z "),
                        }
                    }
                    svg.push_str(&format!("  <path d=\"{}\" {} {} />\n", d.trim(), fill_attr, stroke_attr));
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }
}
