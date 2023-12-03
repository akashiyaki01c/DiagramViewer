use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Line {
    pub stations: Vec<Station>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Station {
    pub name: String,
    pub next_station_distance: f32,
    pub is_main: bool,
}
