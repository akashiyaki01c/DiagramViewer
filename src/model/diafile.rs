use serde::{Serialize, Deserialize};

use super::{line::Line, traintype::TrainType, diagram::Diagram};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct DiaFile {
	pub version: String,
	pub railway: Railway,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Railway {
	pub lines: Vec<Line>,
	pub train_types: Vec<TrainType>,
	pub diagrams: Vec<Diagram>,
}