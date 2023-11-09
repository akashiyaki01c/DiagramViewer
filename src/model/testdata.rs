#![allow(unused)]

use crate::io::file::deserialize;

use super::dia::{diafile::{DiaFile, Railway}, line::{Line, Station}, traintype::TrainType, diagram::{self, Train, StationTime, Time, Diagram}};

fn shiftjis_to_utf8(bytes: &[u8]) -> String {
	String::from(encoding_rs::SHIFT_JIS.decode(bytes).0)
}

fn get_weekday_east_csv() -> String { shiftjis_to_utf8(include_bytes!("./testdata/open_seishin_w_east_r050818.csv")) }
fn get_weekday_west_csv() -> String { shiftjis_to_utf8(include_bytes!("./testdata/open_seishin_w_west_r050818.csv")) }
fn get_holiday_east_csv() -> String { shiftjis_to_utf8(include_bytes!("./testdata/open_seishin_h_east_r050818.csv")) }
fn get_holiday_west_csv() -> String { shiftjis_to_utf8(include_bytes!("./testdata/open_seishin_h_west_r050818.csv")) }

fn str_to_train(str: &str, is_east: bool) -> Vec<Train> {
	let line: Vec<&str> = str.split("\r\n").collect();
	let mut data: Vec<Vec<&str>> = line.iter().map(|v| -> Vec<&str> { v.split(',').collect::<Vec<&str>>() }).collect::<Vec<Vec<&str>>>();
	let mut data = {
		let mut result: Vec<Vec<&str>> = vec![];
		let col_count = data.len();
		let row_count = data.first().unwrap().len();
		for r in 0..row_count {
			result.push(vec![]);
			for d in data.iter().take(col_count) {
				result[r].push(d[r]);
			}
		}
		result.remove(0);
		for d in &mut result {
			d.remove(0);
			d.remove(0);
		}

		result
	};
	let trains: Vec<Train> = {
		let mut result: Vec<Train> = data.iter().map(|v| -> Train {
			let mut train = Train::default();
			let mut times: Vec<StationTime> = vec![];
			'T: for (i, str) in v.iter().enumerate() {
				if str.is_empty() {
					continue 'T;
				}
				let hour_minute = str.split(':').collect::<Vec<&str>>();
				let hour: i8 = hour_minute[0].parse().unwrap();
				let minute: i8 = hour_minute[1].parse().unwrap();
				let station_index = if is_east { 16 - i } else { i };
				times.push(StationTime { 
					line_index: 0, 
					station_index, 
					arrive: Some(Time { hour, minute, second: 0 }), 
					departure: Some(Time { hour, minute, second: 0 })
				});
			}
			train.station_times = times;
			train
		}).collect();
		result
	};
	trains
}

pub fn testdata() -> DiaFile {
	
	DiaFile { 
		version: String::from("0.01"),
		railway: Railway {
			lines: vec![
				Line {
					stations: vec![
						Station { name: String::from("谷上"), next_station_distance: 7.5, is_main: true, },
						Station { name: String::from("新神戸"), next_station_distance: 1.3, is_main: true, },
						Station { name: String::from("三宮"), next_station_distance: 0.9, is_main: false, },
						Station { name: String::from("県庁前"), next_station_distance: 1.1, is_main: false, },
						Station { name: String::from("大倉山"), next_station_distance: 1.0, is_main: true, },
						Station { name: String::from("湊川公園"), next_station_distance: 1.0, is_main: false, },
						Station { name: String::from("上沢"), next_station_distance: 0.8, is_main: false, },
						Station { name: String::from("長田"), next_station_distance: 1.5, is_main: false, },
						Station { name: String::from("新長田"), next_station_distance: 1.2, is_main: true, },
						Station { name: String::from("板宿"), next_station_distance: 2.9, is_main: false, },
						Station { name: String::from("妙法寺"), next_station_distance: 1.6, is_main: false, },
						Station { name: String::from("名谷"), next_station_distance: 1.8, is_main: true, },
						Station { name: String::from("運動公園"), next_station_distance: 1.7, is_main: false, },
						Station { name: String::from("学園都市"), next_station_distance: 1.6, is_main: true, },
						Station { name: String::from("伊川谷"), next_station_distance: 1.7, is_main: false, },
						Station { name: String::from("西神南"), next_station_distance: 2.3, is_main: false, },
						Station { name: String::from("西神中央"), next_station_distance: 0.0, is_main: true, },
					]
				}
			],
			train_types: vec![
				TrainType { name: String::from("普通") }
			],
			diagrams: vec![
				Diagram {
					down_trains: str_to_train(&get_weekday_west_csv(), false),
					up_trains: str_to_train(&get_weekday_east_csv(), true),
				},
				Diagram {
					down_trains: str_to_train(&get_holiday_west_csv(), false),
					up_trains: str_to_train(&get_holiday_east_csv(), true),
				}
			]
		} 
	}
}

/* pub fn testdata() -> DiaFile {
	deserialize(include_str!("../../test.dia")).expect("ParseError")
} */