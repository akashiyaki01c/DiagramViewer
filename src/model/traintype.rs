use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct TrainType {
	pub name: String,
}