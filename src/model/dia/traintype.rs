use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TrainType {
    pub name: String,
    pub line_color: [u8; 3],
    pub line_type: LineType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LineType {
    Solid,
    Bold(f32),
    Dashed(f32, f32, f32),
}

impl Default for LineType {
    fn default() -> Self {
        Self::Solid
    }
}
