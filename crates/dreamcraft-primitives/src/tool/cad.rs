use dreamcraft_core::{Color, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CadEntity {
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Circle { cx: f64, cy: f64, radius: f64 },
    Arc { cx: f64, cy: f64, radius: f64, start_angle: f64, end_angle: f64 },
    Dimension { x1: f64, y1: f64, x2: f64, y2: f64, text: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CadLayer {
    pub name: String,
    pub color: Color,
    pub visible: bool,
    pub entities: Vec<CadEntity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CadDrawing {
    pub id: Id,
    pub name: String,
    pub units: String, // "mm", "inches"
    pub layers: Vec<CadLayer>,
}

impl CadDrawing {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            units: "mm".into(),
            layers: vec![CadLayer {
                name: "0".into(),
                color: Color::WHITE,
                visible: true,
                entities: Vec::new(),
            }],
        }
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

    /// Export basic DXF format string for CAD software.
    pub fn to_dxf(&self) -> String {
        let mut dxf = String::from("0\nSECTION\n2\nENTITIES\n");
        for layer in &self.layers {
            for entity in &layer.entities {
                match entity {
                    CadEntity::Line { x1, y1, x2, y2 } => {
                        dxf.push_str(&format!(
                            "0\nLINE\n8\n{}\n10\n{}\n20\n{}\n11\n{}\n21\n{}\n",
                            layer.name, x1, y1, x2, y2
                        ));
                    }
                    CadEntity::Circle { cx, cy, radius } => {
                        dxf.push_str(&format!(
                            "0\nCIRCLE\n8\n{}\n10\n{}\n20\n{}\n40\n{}\n",
                            layer.name, cx, cy, radius
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
