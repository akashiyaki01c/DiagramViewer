use serde::{Deserialize, Serialize};

use super::{diagram::Diagram, line::Line, traintype::TrainType};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DiaFile {
    pub version: String,
    pub railway: Railway,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Railway {
    pub lines: Vec<Line>,
    pub train_types: Vec<TrainType>,
    pub diagrams: Vec<Diagram>,
}
