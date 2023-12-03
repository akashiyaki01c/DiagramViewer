#[test]
fn test() {
	use crate::train::get_train_lines;

	let root = diagram_testdata::testdata_kt();
	let is_reverse = false;
	
	let positions = crate::station::get_station_positions(&root.settings.diagram_display, &root, is_reverse);
	let times = &root.diagrams[0].trains.iter().map(|train| {
		get_train_lines(train, &root, is_reverse)
	}).collect::<Vec<_>>();
	for i in 0..positions.len() {
		let str = format!("{}{}", positions[i].0.name.clone(), (vec!['　'; 6-positions[i].0.name.chars().count()]).iter().collect::<String>());
		print!("{:>6.2} {}", positions[i].1, str);
		for train in times {
			print!("|{:^22}", train[i].to_time_string())
		}
		println!("");
	}
	
}

