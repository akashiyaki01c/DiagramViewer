use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BasicStation {
    pub name: String,
    pub tracks: Vec<StationTrack>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StationTrack(pub usize, pub String);
