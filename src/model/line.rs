use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Line {
	pub stations: Vec<Station>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Station {
	pub name: String,
	pub next_station_distance: f32,
	pub is_main: bool,
}