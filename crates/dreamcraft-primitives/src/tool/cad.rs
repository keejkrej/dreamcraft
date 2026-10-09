use dreamcraft_core::{Color, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CadLineType {
    #[default]
    Continuous,
    Dashed,
    Dotted,
    Center,
    Hidden,
    Phantom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolylineVertex {
    pub x: f64,
    pub y: f64,
    pub bulge: f64, // 0 = straight segment, tan(theta/4) for circular arc
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DimensionKind {
    LinearHorizontal,
    LinearVertical,
    Aligned,
    Radial,
    Diameter,
    Angular,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CadEntity {
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
    Polyline {
        vertices: Vec<PolylineVertex>,
        is_closed: bool,
    },
    Circle {
        cx: f64,
        cy: f64,
        radius: f64,
    },
    Arc {
        cx: f64,
        cy: f64,
        radius: f64,
        start_angle_deg: f64,
        end_angle_deg: f64,
    },
    Ellipse {
        cx: f64,
        cy: f64,
        major_axis_dx: f64,
        major_axis_dy: f64,
        minor_major_ratio: f64,
    },
    Hatch {
        pattern: String, // "SOLID", "ANSI31", "ANSI32"
        scale: f64,
        angle_deg: f64,
        boundary_vertices: Vec<(f64, f64)>,
    },
    Dimension {
        kind: DimensionKind,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        text_override: Option<String>,
        measured_value: f64,
    },
    Text {
        x: f64,
        y: f64,
        height: f64,
        rotation_deg: f64,
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CadLayer {
    pub name: String,
    pub color: Color,
    pub linetype: CadLineType,
    pub lineweight_mm: f64,
    pub visible: bool,
    pub locked: bool,
    pub frozen: bool,
    pub entities: Vec<CadEntity>,
}

impl CadLayer {
    pub fn new(name: impl Into<String>, color: Color) -> Self {
        Self {
            name: name.into(),
            color,
            linetype: CadLineType::Continuous,
            lineweight_mm: 0.25,
            visible: true,
            locked: false,
            frozen: false,
            entities: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CadDrawing {
    pub id: Id,
    pub name: String,
    pub units: String, // "mm", "inches", "meters"
    pub layers: Vec<CadLayer>,
}

impl CadDrawing {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            units: "mm".into(),
            layers: vec![
                CadLayer::new("0", Color::WHITE),
                CadLayer::new("Dimensions", Color::rgb(0, 180, 255)),
                CadLayer::new("Centerlines", Color::rgb(255, 60, 60)),
            ],
        }
    }

    pub fn add_layer(&mut self, name: impl Into<String>, color: Color) -> usize {
        let layer = CadLayer::new(name, color);
        self.layers.push(layer);
        self.layers.len() - 1
    }

    pub fn add_line(&mut self, layer_idx: usize, x1: f64, y1: f64, x2: f64, y2: f64) {
        if let Some(layer) = self.layers.get_mut(layer_idx) {
            layer.entities.push(CadEntity::Line { x1, y1, x2, y2 });
        }
    }

    pub fn add_circle(&mut self, layer_idx: usize, cx: f64, cy: f64, radius: f64) {
        if let Some(layer) = self.layers.get_mut(layer_idx) {
            layer.entities.push(CadEntity::Circle { cx, cy, radius });
        }
    }

    pub fn add_arc(&mut self, layer_idx: usize, cx: f64, cy: f64, radius: f64, start_deg: f64, end_deg: f64) {
        if let Some(layer) = self.layers.get_mut(layer_idx) {
            layer.entities.push(CadEntity::Arc {
                cx,
                cy,
                radius,
                start_angle_deg: start_deg,
                end_angle_deg: end_deg,
            });
        }
    }

    pub fn add_dimension(&mut self, layer_idx: usize, kind: DimensionKind, x1: f64, y1: f64, x2: f64, y2: f64) {
        let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        if let Some(layer) = self.layers.get_mut(layer_idx) {
            layer.entities.push(CadEntity::Dimension {
                kind,
                x1,
                y1,
                x2,
                y2,
                text_override: None,
                measured_value: dist,
            });
        }
    }

    /// Export standard AutoCAD DXF format string.
    pub fn to_dxf(&self) -> String {
        let mut dxf = String::from(
            "0\nSECTION\n2\nHEADER\n9\n$INSUNITS\n70\n4\n0\nENDSEC\n\
             0\nSECTION\n2\nTABLES\n0\nTABLE\n2\nLAYER\n70\n1\n"
        );

        for layer in &self.layers {
            dxf.push_str(&format!(
                "0\nLAYER\n2\n{}\n70\n0\n62\n7\n6\nCONTINUOUS\n",
                layer.name
            ));
        }
        dxf.push_str("0\nENDTAB\n0\nENDSEC\n");

        dxf.push_str("0\nSECTION\n2\nENTITIES\n");
        for layer in &self.layers {
            for entity in &layer.entities {
                match entity {
                    CadEntity::Line { x1, y1, x2, y2 } => {
                        dxf.push_str(&format!(
                            "0\nLINE\n8\n{}\n10\n{}\n20\n{}\n30\n0.0\n11\n{}\n21\n{}\n31\n0.0\n",
                            layer.name, x1, y1, x2, y2
                        ));
                    }
                    CadEntity::Circle { cx, cy, radius } => {
                        dxf.push_str(&format!(
                            "0\nCIRCLE\n8\n{}\n10\n{}\n20\n{}\n30\n0.0\n40\n{}\n",
                            layer.name, cx, cy, radius
                        ));
                    }
                    CadEntity::Arc { cx, cy, radius, start_angle_deg, end_angle_deg } => {
                        dxf.push_str(&format!(
                            "0\nARC\n8\n{}\n10\n{}\n20\n{}\n30\n0.0\n40\n{}\n50\n{}\n51\n{}\n",
                            layer.name, cx, cy, radius, start_angle_deg, end_angle_deg
                        ));
                    }
                    CadEntity::Text { x, y, height, rotation_deg, text } => {
                        dxf.push_str(&format!(
                            "0\nTEXT\n8\n{}\n10\n{}\n20\n{}\n30\n0.0\n40\n{}\n50\n{}\n1\n{}\n",
                            layer.name, x, y, height, rotation_deg, text
                        ));
                    }
                    CadEntity::Dimension { x1, y1, x2, y2, measured_value, .. } => {
                        dxf.push_str(&format!(
                            "0\nDIMENSION\n8\n{}\n10\n{}\n20\n{}\n11\n{}\n21\n{}\n42\n{}\n",
                            layer.name, x1, y1, x2, y2, measured_value
                        ));
                    }
                    _ => {}
                }
            }
        }
        dxf.push_str("0\nENDSEC\n0\nEOF\n");
        dxf
    }
}
