use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct BasicStation {
	pub name: String,
	pub tracks: Vec<StationTrack>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct StationTrack(pub usize, pub String);