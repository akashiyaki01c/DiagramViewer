use diagram_types::{root::RootFile, train::{Train, StationTime, Direction}, station::Station, settings::DiagramDisplaySettings, route::RouteId, line::{Line, RouteRefDirection}};

use crate::station::get_station;

pub fn get_station_with_route<'a>(settings: &'a DiagramDisplaySettings, root: &'a RootFile, is_reverse: bool) -> Vec<(&'a Station, Vec<RouteId>)> {
	let mut result: Vec<(&Station, Vec<RouteId>)> = vec![];
	let routes = if is_reverse {
		let mut refs = settings.route_refs.clone();
		refs.reverse();
		refs
	} else {
		settings.route_refs.clone()
	};
	for route in routes {
		let (route, direction) = (root.routes.get().get(&route.0).unwrap(), &route.1);
		let stations = get_station(&route, direction.clone(), &root, is_reverse);
		let is_first_link = if result.is_empty() {
			false
		} else {
			if result.first().is_none() {
				false
			} else if result.last().unwrap().0.id() == stations.first().unwrap().id() {
				true
			} else {
				false
			}
		};
		if is_first_link {
			let last = result.len() - 1;
			result[last].1.push(route.id());
		} else {
			let sta = stations[0];
			result.push((sta, vec![route.id()]));
		}
		for i in 1..stations.len() {
			let sta = stations[i];
			result.push((sta, vec![route.id()]));
		}
	}
	result
}

pub fn get_station_with_line<'a>(line: &'a Line, root: &'a RootFile, is_reverse: bool) -> Vec<(&'a Station, Vec<RouteId>)> {
	let mut result: Vec<(&Station, Vec<RouteId>)> = vec![];
	let routes = if is_reverse {
		let mut refs = Vec::from(line.route_refs());
		refs.reverse();
		refs
	} else {
		Vec::from(line.route_refs())
	};
	for route in routes {
		let (route, direction) = (root.routes.get().get(&route.0).unwrap(), &route.1);

		let stations = get_station(&route, direction.clone(), &root, is_reverse);
		let is_first_link = if result.is_empty() {
			false
		} else {
			if result.first().is_none() {
				false
			} else if result.last().unwrap().0.id() == stations.first().unwrap().id() {
				true
			} else {
				false
			}
		};
		if is_first_link {
			let last = result.len() - 1;
			result[last].1.push(route.id());
		} else {
			let sta = stations[0];
			result.push((sta, vec![route.id()]));
		}
		for i in 1..stations.len() {
			let sta = stations[i];
			result.push((sta, vec![route.id()]));
		}
	}
	result
}

pub fn get_station_with_all_line<'a>(line: &'a Line, root: &'a RootFile) -> Vec<(&'a Station, Vec<RouteId>)> {
	let mut result: Vec<(&Station, Vec<RouteId>)> = vec![];
	let routes = Vec::from(line.route_refs());
	for route in routes {
		let (route, direction) = (root.routes.get().get(&route.0).unwrap(), &route.1);

		let stations = get_station(&route, direction.clone(), &root, false);
		let is_first_link = if result.is_empty() {
			false
		} else {
			if result.first().is_none() {
				false
			} else if result.last().unwrap().0.id() == stations.first().unwrap().id() {
				true
			} else {
				false
			}
		};
		if is_first_link {
			let last = result.len() - 1;
			result[last].1.push(route.id());
		} else {
			let sta = stations[0];
			result.push((sta, vec![route.id()]));
		}
		for i in 1..stations.len() {
			let sta = stations[i];
			result.push((sta, vec![route.id()]));
		}
	}
	result
}

/// 列車の時刻を時刻表路線群の時刻形式に変換する関数
pub fn get_train_lines(train: &Train, root: &RootFile, is_reverse: bool) -> Vec<StationTime> {
	let mut display_stations = get_station_with_route(&root.settings.diagram_display, &root, is_reverse);
	let line = root.lines.get().get(&train.line_id()).unwrap();
	let line_stations = get_station_with_line(&line, &root, is_reverse);
	let to_remove_lines = line.route_refs().iter().filter(|route| {
		let is_delete = if route.1 == RouteRefDirection::Normal {
			if is_reverse { true } else { false }
		} else if route.1 == RouteRefDirection::Reverse {
			if is_reverse { false } else { true }
		} else { todo!() };
		if train.direction == Direction::Up {
			!is_delete
		} else {
			is_delete
		}
	}).collect::<Vec<_>>();
	for i in 0..display_stations.len() {
		for j in 0..to_remove_lines.len() {
			if let Some(index) = display_stations[i].1.iter().position(|v| *v == to_remove_lines[j].0) {
				display_stations[i].1.remove(index);
			}
		}
	}
	let mut times: Vec<StationTime> = vec![StationTime::default(); display_stations.len()];
	
	for i in 0..line_stations.len() {
		let sta = &line_stations[i];
		let get_index = || -> Vec<usize> {
			// sta と一致する駅のインデックスを取得する
			let mut ret: Vec<usize> = vec![];
			for i in 0..display_stations.len() {
				if display_stations[i].0.id() == sta.0.id() 
				&& display_stations[i].1.iter().any(|v| {sta.1.iter().any(|id| id == v)}) {
					ret.push(i);
				}
			} 
			ret
		};
		for j in get_index() {
			let i = if is_reverse {
				line_stations.len() - i - 1
			} else { i };
			let i = if train.direction == Direction::Up {
				line_stations.len() - i - 1
			} else { i };
			times[j] = train.times()[i].clone();
		}
	}
	times
}

pub fn get_train_all_lines(train: &Train, root: &RootFile) -> Vec<StationTime> {
	let mut display_stations = get_station_with_route(&root.settings.diagram_display, &root, false);
	let line = root.lines.get().get(&train.line_id()).unwrap();
	let line_stations = get_station_with_all_line(&line, &root);
	let to_remove_lines = line.route_refs();
	for i in 0..display_stations.len() {
		for j in 0..to_remove_lines.len() {
			if let Some(index) = display_stations[i].1.iter().position(|v| *v == to_remove_lines[j].0) {
				display_stations[i].1.remove(index);
			}
		}
	}
	let mut times: Vec<StationTime> = vec![StationTime::default(); display_stations.len()];
	
	for i in 0..line_stations.len() {
		let sta = &line_stations[i];
		let get_index = || -> Vec<usize> {
			// sta と一致する駅のインデックスを取得する
			let mut ret: Vec<usize> = vec![];
			for i in 0..display_stations.len() {
				if display_stations[i].0.id() == sta.0.id() 
				&& display_stations[i].1.iter().any(|v| {sta.1.iter().any(|id| id == v)}) {
					ret.push(i);
				}
			} 
			ret
		};
		for j in get_index() {
			let i = i;
			let i = if train.direction == Direction::Up {
				line_stations.len() - i - 1
			} else { i };
			times[j] = train.times()[i].clone();
		}
	}
	times
}