use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Diagram {
    pub down_trains: Vec<Train>,
    pub up_trains: Vec<Train>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Train {
    pub train_type_index: usize,
    pub station_times: Vec<StationTime>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StationTime {
    pub line_index: usize,
    pub station_index: usize,
    pub arrive: Option<Time>,
    pub departure: Option<Time>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct Time {
    pub hour: i8,
    pub minute: i8,
    pub second: i8,
}

impl Time {
    pub fn get_total_minute(&self) -> usize {
        (self.hour as usize) * 60 + (self.minute as usize)
    }
}
