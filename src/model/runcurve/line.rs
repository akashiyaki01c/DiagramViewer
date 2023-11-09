use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct RuncurveLine {
	up: Line,
	down: Line,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Line {
	gradients: Vec<Gradient>,
	cruves: Vec<Curve>,
	tunnels: Vec<Tunnel>,
	turnouts: Vec<Turnout>,
	speed_limits: Vec<SpeedLimit>,
	signals: Vec<Signal>,
	stop_positions: Vec<StopPosition>,
	break_chains: Vec<BreakChain>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Gradient {
	pub distance: Distance,
	pub gradient_permille: f32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone, Copy)]
pub struct Distance(f32);

#[allow(dead_code)]
impl Distance {
	pub fn new(kilo: f32) -> Self {
		Self(kilo)
	}

	pub fn meter(&self) -> f32 { self.0 * 1000.0 }
	pub fn kilo(&self) -> f32 { self.0}
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Curve {
	pub start_distance: Distance,
	pub end_distance: Distance,
	pub radius: Radius,
	pub limit_speed: f32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Radius {
	Right(f32),
	Left(f32),
}
impl Default for Radius {
	fn default() -> Self { Self::Right(0.0) }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Tunnel {
	pub start_distance: Distance,
	pub end_distance: Distance,
	pub name: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Turnout {
	pub start_distance: Distance,
	pub end_distance: Distance,
	pub name: String,
	pub normal_limit_speed: f32,
	pub reverse_limit_speed: f32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct SpeedLimit {
	pub start_distance: Distance,
	pub end_distance: Distance,
	pub limit_speed: f32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct SignalIndex(
	pub usize, 
	pub f32
);

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct Signal {
	pub distance: Distance,
	pub block_name: String,
	pub circuit_name: String,
	pub signal_index: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct StopPosition {
	pub distance: Distance,
	pub station_index: usize,
	pub track_index: usize,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
pub struct BreakChain {
	pub from_distance: Distance,
	pub to_distance: Distance,
	pub name: String,
}