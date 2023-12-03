use diagram_types::{settings::DiagramDisplaySettings, root::RootFile, line::RouteRefDirection, route::Route, station::Station};

pub fn get_station<'a>(route: &'a Route, direction: RouteRefDirection, root: &'a RootFile, is_reverse: bool) -> Vec<&'a Station> {
	match direction {
		RouteRefDirection::Normal => {
			let mut result = route.station_refs().iter().map(|id| {
				root.stations.get().get(id).unwrap()
			}).collect::<Vec<&Station>>();
			if is_reverse {
				result.reverse();
			}
			result
		}
		RouteRefDirection::Reverse => {
			let mut result = route.station_refs().iter().rev().map(|id| {
				root.stations.get().get(id).unwrap()
			}).collect::<Vec<&Station>>();
			if is_reverse {
				result.reverse();
			}
			result
		}
	}
}

fn get_station_distance<'a>(route: &'a Route, direction: RouteRefDirection) -> Vec<f32> {
	match direction {
		RouteRefDirection::Normal => {
			let mut now: f32 = *route.station_distances().first().unwrap_or(&0.0);
			let mut result = vec![];
			for distance in route.station_distances().iter() {
				result.push(distance - now);
				now = *distance;
			}
			result
		}
		RouteRefDirection::Reverse => {
			let mut now: f32 = *route.station_distances().last().unwrap_or(&0.0);
			let mut result = vec![];
			for distance in route.station_distances().iter().rev() {
				result.push(distance - now);
				now = *distance;
			}
			result
		}
	}
}

pub fn get_station_positions<'a>(settings: &'a DiagramDisplaySettings, root: &'a RootFile, is_reverse: bool) -> Vec<(&'a Station, f32)> {
	let mut positions: Vec<(&Station, f32)> = vec![];
	let mut now_pos: f32 = -10.0;
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
		let distances = get_station_distance(route, direction.clone());
		let is_first_link = if positions.is_empty() {
			false
		} else {
			if stations.first().is_none() {
				false
			} else if positions.last().unwrap().0.id() == stations.first().unwrap().id() {
				true
			} else {
				false
			}
		};
		if is_first_link {
		} else {
			now_pos += 10.0;
			let sta = stations[0];
			let distance = distances[0];
			now_pos += distance.abs();
			positions.push((sta, now_pos));
		}
		for i in 1..stations.len() {
			let sta = stations[i];
			let distance = distances[i];
			now_pos += distance.abs();
			positions.push((sta, now_pos));
		}
	}
	positions
}