use std::collections::HashMap;

use diagram_types::{station::{Station, StationMap}, route::{RouteMap, Route}, line::{LineMap, Line, RouteRefDirection}, settings::{Settings, DiagramDisplaySettings}, root::RootFile, train::{Direction, Train, StationTime, StopType}, Time, diagram::Diagram, train_type::{TrainType, TrainTypeMap}};



fn stations() -> StationMap {
    let mut stations: StationMap = StationMap::new(HashMap::new());
    stations.insert(&Station::new(String::from("大阪難波")));
	stations.insert(&Station::new(String::from("大阪上本町")));
	stations.insert(&Station::new(String::from("鶴橋")));
	stations.insert(&Station::new(String::from("布施")));
	stations.insert(&Station::new(String::from("大和高田")));
	stations.insert(&Station::new(String::from("大和八木")));
	stations.insert(&Station::new(String::from("榛原")));
	stations.insert(&Station::new(String::from("名張")));
	stations.insert(&Station::new(String::from("桔梗が丘")));
	stations.insert(&Station::new(String::from("伊賀神戸")));
	stations.insert(&Station::new(String::from("榊原温泉口")));
	stations.insert(&Station::new(String::from("伊勢中川")));

	stations.insert(&Station::new(String::from("久居")));
	stations.insert(&Station::new(String::from("津")));
	stations.insert(&Station::new(String::from("白子")));
	stations.insert(&Station::new(String::from("四日市")));
	stations.insert(&Station::new(String::from("桑名")));
	stations.insert(&Station::new(String::from("名古屋")));

	stations.insert(&Station::new(String::from("松坂")));
	stations.insert(&Station::new(String::from("伊勢市")));
	stations.insert(&Station::new(String::from("宇治山田")));
	stations.insert(&Station::new(String::from("五十鈴川")));
	stations.insert(&Station::new(String::from("鳥羽")));
	stations.insert(&Station::new(String::from("志摩磯部")));
	stations.insert(&Station::new(String::from("鵜方")));
	stations.insert(&Station::new(String::from("賢島")));

	stations.insert(&Station::new(String::from("生駒")));
	stations.insert(&Station::new(String::from("学園前")));
	stations.insert(&Station::new(String::from("大和西大寺")));
	stations.insert(&Station::new(String::from("奈良")));

	stations.insert(&Station::new(String::from("京都")));
	stations.insert(&Station::new(String::from("丹波橋")));
	stations.insert(&Station::new(String::from("高の原")));
	stations.insert(&Station::new(String::from("西の京")));
	stations.insert(&Station::new(String::from("橿原神宮前")));

    stations
}

fn routes(sta: &StationMap) -> RouteMap {
    let mut routes = RouteMap::default();
	routes.insert(&Route::new(
        String::from("難波線"),
        &[
			sta.search_from_name("大阪難波").unwrap().id(),
			sta.search_from_name("大阪上本町").unwrap().id(),
        ],
        &[
            0.0,
            2.0
        ]
    ));

    routes.insert(&Route::new(
        String::from("大阪線(西)"),
        &[
			sta.search_from_name("大阪上本町").unwrap().id(),
			sta.search_from_name("鶴橋").unwrap().id(),
			sta.search_from_name("布施").unwrap().id(),
			sta.search_from_name("大和高田").unwrap().id(),
			sta.search_from_name("大和八木").unwrap().id(),
        ],
        &[
            0.0,
            1.1,
            4.1,
			29.9,
			34.8,
        ]
    ));

	routes.insert(&Route::new(
        String::from("大阪線(東)"),
        &[
			sta.search_from_name("大和八木").unwrap().id(),
			sta.search_from_name("榛原").unwrap().id(),
			sta.search_from_name("名張").unwrap().id(),
			sta.search_from_name("桔梗が丘").unwrap().id(),
			sta.search_from_name("伊賀神戸").unwrap().id(),
			sta.search_from_name("榊原温泉口").unwrap().id(),
			sta.search_from_name("伊勢中川").unwrap().id(),
        ],
        &[
			34.8,
			50.1,
			67.2,
			70.0,
			75.5,
			95.4,
			108.9
        ]
    ));

	routes.insert(&Route::new(
        String::from("名古屋線"),
        &[
			sta.search_from_name("伊勢中川").unwrap().id(),
			sta.search_from_name("久居").unwrap().id(),
			sta.search_from_name("津").unwrap().id(),
			sta.search_from_name("白子").unwrap().id(),
			sta.search_from_name("四日市").unwrap().id(),
			sta.search_from_name("桑名").unwrap().id(),
			sta.search_from_name("名古屋").unwrap().id(),
        ],
        &[
            0.0,
			4.8,
			12.3,
			25.9,
			41.9,
			55.1,
			78.8
        ]
    ));

	routes.insert(&Route::new(
        String::from("山田線"),
        &[
			sta.search_from_name("伊勢中川").unwrap().id(),
			sta.search_from_name("松坂").unwrap().id(),
			sta.search_from_name("伊勢市").unwrap().id(),
			sta.search_from_name("宇治山田").unwrap().id(),
        ],
        &[
            0.0,
			8.4,
			27.7,
			28.3
        ]
    ));

	routes.insert(&Route::new(
        String::from("鳥羽線"),
        &[
			sta.search_from_name("宇治山田").unwrap().id(),
			sta.search_from_name("五十鈴川").unwrap().id(),
			sta.search_from_name("鳥羽").unwrap().id(),
        ],
        &[
            0.0,
			1.9,
			13.2
        ]
    ));

	routes.insert(&Route::new(
        String::from("志摩線"),
        &[
			sta.search_from_name("鳥羽").unwrap().id(),
			sta.search_from_name("志摩磯部").unwrap().id(),
			sta.search_from_name("鵜方").unwrap().id(),
			sta.search_from_name("賢島").unwrap().id(),
        ],
        &[
            0.0,
			16.0,
			21.3,
			24.5,
        ]
    ));

	routes.insert(&Route::new(
        String::from("奈良線(西)"),
        &[
			sta.search_from_name("大阪上本町").unwrap().id(),
			sta.search_from_name("鶴橋").unwrap().id(),
			sta.search_from_name("生駒").unwrap().id(),
			sta.search_from_name("学園前").unwrap().id(),
			sta.search_from_name("大和西大寺").unwrap().id(),
        ],
        &[
            -4.1,
			-3.0,
			14.2,
			19.1,
			22.3
        ]
    ));

	routes.insert(&Route::new(
        String::from("奈良線(東)"),
        &[
			sta.search_from_name("大和西大寺").unwrap().id(),
			sta.search_from_name("奈良").unwrap().id(),
        ],
        &[
            22.3,
			26.7
        ]
    ));

	routes.insert(&Route::new(
        String::from("京都線"),
        &[
			sta.search_from_name("京都").unwrap().id(),
			sta.search_from_name("丹波橋").unwrap().id(),
			sta.search_from_name("高の原").unwrap().id(),
			sta.search_from_name("大和西大寺").unwrap().id(),
        ],
        &[
            0.0,
			6.0,
			30.8,
			34.6,
        ]
    ));

	routes.insert(&Route::new(
        String::from("橿原線(北)"),
        &[
			sta.search_from_name("大和西大寺").unwrap().id(),
			sta.search_from_name("西の京").unwrap().id(),
			sta.search_from_name("大和八木").unwrap().id(),
        ],
        &[
            0.0,
			2.8,
			20.5,
        ]
    ));

	routes.insert(&Route::new(
        String::from("橿原線(南)"),
        &[
			sta.search_from_name("大和八木").unwrap().id(),
			sta.search_from_name("橿原神宮前").unwrap().id(),
        ],
        &[
            20.5,
			23.8,
        ]
    ));
    
    routes
}

fn lines(routes: &RouteMap) -> LineMap {
    let mut lines = LineMap::default();
    lines.insert(&Line::new(
        String::from("名阪特急"),
        &[
			(routes.search_from_name("難波線").unwrap().id(), RouteRefDirection::Normal,),
            (routes.search_from_name("大阪線(西)").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("大阪線(東)").unwrap().id(), RouteRefDirection::Normal,),
            (routes.search_from_name("名古屋線").unwrap().id(), RouteRefDirection::Normal,),
        ],
    ));
	lines.insert(&Line::new(
        String::from("阪伊特急"),
        &[
			(routes.search_from_name("難波線").unwrap().id(), RouteRefDirection::Normal,),
            (routes.search_from_name("大阪線(西)").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("大阪線(東)").unwrap().id(), RouteRefDirection::Normal,),
            (routes.search_from_name("山田線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("鳥羽線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("志摩線").unwrap().id(), RouteRefDirection::Normal,),
        ],
    ));
	lines.insert(&Line::new(
        String::from("名伊特急"),
        &[
			(routes.search_from_name("名古屋線").unwrap().id(), RouteRefDirection::Reverse,),
            (routes.search_from_name("山田線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("鳥羽線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("志摩線").unwrap().id(), RouteRefDirection::Normal,),
        ],
    ));
	lines.insert(&Line::new(
        String::from("京伊特急"),
        &[
            (routes.search_from_name("京都線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("橿原線(北)").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("大阪線(東)").unwrap().id(), RouteRefDirection::Normal,),
            (routes.search_from_name("山田線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("鳥羽線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("志摩線").unwrap().id(), RouteRefDirection::Normal,),
        ],
    ));
	lines.insert(&Line::new(
        String::from("京奈特急"),
        &[
            (routes.search_from_name("京都線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("奈良線(東)").unwrap().id(), RouteRefDirection::Normal,),
        ],
    ));
	lines.insert(&Line::new(
        String::from("京橿特急"),
        &[
            (routes.search_from_name("京都線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("橿原線(北)").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("橿原線(南)").unwrap().id(), RouteRefDirection::Normal,),
        ],
    ));
	lines.insert(&Line::new(
        String::from("阪奈特急"),
        &[
            (routes.search_from_name("難波線").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("奈良線(西)").unwrap().id(), RouteRefDirection::Normal,),
			(routes.search_from_name("奈良線(東)").unwrap().id(), RouteRefDirection::Normal,),
        ],
    ));
    lines
}

fn settings(routes: &RouteMap) -> Settings {
    Settings { diagram_display: DiagramDisplaySettings {
        route_refs: vec![
            (routes.search_from_name("京都線").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("難波線").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("奈良線(西)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("奈良線(東)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("橿原線(北)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("橿原線(南)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("大阪線(西)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("大阪線(東)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("名古屋線").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("山田線").unwrap().id(), RouteRefDirection::Normal),
			(routes.search_from_name("鳥羽線").unwrap().id(), RouteRefDirection::Normal),
			(routes.search_from_name("志摩線").unwrap().id(), RouteRefDirection::Normal),
        ]
    } }
}

fn train_type() -> TrainTypeMap {
    let mut train_types = TrainTypeMap::default();
    train_types.insert(&TrainType::new("特急"));

    train_types
}

pub fn testdata_kt() -> RootFile {
    let stations = stations();
    let routes = routes(&stations);
    let lines = lines(&routes);
    let train_types = train_type();

	let trains: Vec<Train> = vec![
		Train::new(
			lines.search_from_name("名阪特急").unwrap().id(),
			Direction::Down,
			&lines,
			train_types.search_from_name("特急").unwrap().id(),
			&[
				StationTime::new(StopType::Stop, None, Some(Time::new(6, 30, 0))), // 難波
				StationTime::new(StopType::Stop, None, Some(Time::new(6, 33, 0))), // 上本町
				StationTime::new(StopType::Stop, None, Some(Time::new(6, 36, 0))), // 鶴橋
				StationTime::new(StopType::Pass, None, None), // 布施
				StationTime::new(StopType::Pass, None, None), // 高田
				StationTime::new(StopType::Stop, Some(Time::new(6, 59, 0)), Some(Time::new(7, 00, 0))), // 八木
				StationTime::new(StopType::Pass, None, None), // 榛原
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 22, 0))), // 名張
				StationTime::new(StopType::Pass, None, None), // 桔梗が丘
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 28, 0))), // 神戸
				StationTime::new(StopType::Pass, None, None), // 榊原温泉口
				StationTime::new(StopType::Pass, None, None), // 伊勢中川
				StationTime::new(StopType::Pass, None, None), // 久居
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 1, 0))), // 津
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 10, 0))), // 白子
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 23, 0))), // 四日市
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 35, 0))), // 桑名
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 54, 0))), // 名古屋
			]
		),
		Train::new(
			lines.search_from_name("名阪特急").unwrap().id(),
			Direction::Up,
			&lines,
			train_types.search_from_name("特急").unwrap().id(),
			&[
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 00, 0))), // 名古屋
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 45, 0))), // 津
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None), // 中川
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Stop, Some(Time::new(8, 37, 0)), Some(Time::new(8, 38, 0))), // 八木
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Stop, None, Some(Time::new(9, 3, 0))), // 鶴橋
				StationTime::new(StopType::Stop, None, Some(Time::new(9, 5, 0))), // 上本町
				StationTime::new(StopType::Stop, Some(Time::new(9, 9, 0)), None), // 難波
			]
		),
		Train::new(
			lines.search_from_name("阪伊特急").unwrap().id(),
			Direction::Down,
			&lines,
			train_types.search_from_name("特急").unwrap().id(),
			&[
				StationTime::new(StopType::Stop, None, Some(Time::new(6, 5, 0))), // 難波
				StationTime::new(StopType::Stop, None, Some(Time::new(6, 8, 0))), // 上本町
				StationTime::new(StopType::Stop, None, Some(Time::new(6, 11, 0))), // 鶴橋
				StationTime::new(StopType::Pass, None, None), // 布施
				StationTime::new(StopType::Stop, None, Some(Time::new(6, 32, 0))), // 高田
				StationTime::new(StopType::Stop, Some(Time::new(6, 36, 0)), Some(Time::new(6, 37, 0))), // 八木
				StationTime::new(StopType::Stop, None, Some(Time::new(6, 48, 0))), // 榛原
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 02, 0))), // 名張
				StationTime::new(StopType::Pass, None, None), // 桔梗が丘
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 08, 0))), // 神戸
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 21, 0))), // 榊原温泉口
				StationTime::new(StopType::Stop, Some(Time::new(7, 30, 0)), Some(Time::new(7, 32, 0))), // 伊勢中川
				StationTime::new(StopType::Stop, Some(Time::new(7, 38, 0)), Some(Time::new(7, 41, 0))), // 松坂
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 53, 0))), // 伊勢市
				StationTime::new(StopType::Stop, Some(Time::new(7, 55, 0)), Some(Time::new(7, 59, 0))), // 宇治山田
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 1, 0))), // 五十鈴川
				StationTime::new(StopType::Stop, Some(Time::new(8, 10, 0)), Some(Time::new(8, 13, 0))), // 鳥羽
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 30, 0))), // 志摩磯部
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 36, 0))), // 鵜方
				StationTime::new(StopType::Stop, Some(Time::new(8, 42, 0)), None), // 賢島
			]
		),
		Train::new(
			lines.search_from_name("名伊特急").unwrap().id(),
			Direction::Down,
			&lines,
			train_types.search_from_name("特急").unwrap().id(),
			&[
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 00, 0))), // 名古屋
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 45, 0))), // 津
				StationTime::new(StopType::Pass, None, None),
				StationTime::new(StopType::Stop, Some(Time::new(7, 30, 0)), Some(Time::new(7, 32, 0))), // 伊勢中川
				StationTime::new(StopType::Stop, Some(Time::new(7, 38, 0)), Some(Time::new(7, 41, 0))), // 松坂
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 53, 0))), // 伊勢市
				StationTime::new(StopType::Stop, Some(Time::new(7, 55, 0)), Some(Time::new(7, 59, 0))), // 宇治山田
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 1, 0))), // 五十鈴川
				StationTime::new(StopType::Stop, Some(Time::new(8, 10, 0)), Some(Time::new(8, 13, 0))), // 鳥羽
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 30, 0))), // 志摩磯部
				StationTime::new(StopType::Stop, None, Some(Time::new(8, 36, 0))), // 鵜方
				StationTime::new(StopType::Stop, Some(Time::new(8, 42, 0)), None), // 賢島
			]
		),
		Train::new(
			lines.search_from_name("京奈特急").unwrap().id(),
			Direction::Down,
			&lines,
			train_types.search_from_name("特急").unwrap().id(),
			&[
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 50, 0))), // 京都
				StationTime::new(StopType::Stop, None, Some(Time::new(7, 57, 0))), // 丹波橋
				StationTime::new(StopType::None, None, None), // 高の原
				StationTime::new(StopType::Stop, Some(Time::new(8, 20, 0)), Some(Time::new(8, 20, 0))), // 西大寺
				StationTime::new(StopType::Stop, Some(Time::new(8, 25, 0)), None), // 奈良
			]
		)
	];

    let root = RootFile {
        stations: stations.clone(),
        routes: routes.clone(),
        lines: lines.clone(),
        train_types: train_types.clone(),
        diagrams: vec![Diagram::new("平日", &trains)],
        settings: settings(&routes),
        ..Default::default()
    };
    root
}