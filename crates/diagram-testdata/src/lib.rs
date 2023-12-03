mod data_kintetsu;

pub use data_kintetsu::testdata_kt;

use std::collections::HashMap;

use diagram_types::{
    diagram::Diagram,
    line::{Line, LineMap, RouteRefDirection},
    root::RootFile,
    route::{Route, RouteMap},
    station::{Station, StationMap},
    train::{StationTime, StopType, Train, Direction},
    train_type::{TrainType, TrainTypeMap},
    Time, settings::{Settings, DiagramDisplaySettings},
};

fn stations() -> StationMap {
    let mut stations: StationMap = StationMap::new(HashMap::new());
    stations.insert(&Station::new(String::from("新開地")));
    stations.insert(&Station::new(String::from("高速神戸")));
    stations.insert(&Station::new(String::from("花隈")));
    stations.insert(&Station::new(String::from("神戸三宮")));
    stations.insert(&Station::new(String::from("春日野道")));
    stations.insert(&Station::new(String::from("王子公園")));
    stations.insert(&Station::new(String::from("六甲")));
    stations.insert(&Station::new(String::from("御影")));
    stations.insert(&Station::new(String::from("岡本")));
    stations.insert(&Station::new(String::from("芦屋川")));
    stations.insert(&Station::new(String::from("夙川")));
    stations.insert(&Station::new(String::from("西宮北口")));
    stations.insert(&Station::new(String::from("武庫之荘")));
    stations.insert(&Station::new(String::from("塚口")));
    stations.insert(&Station::new(String::from("園田")));
    stations.insert(&Station::new(String::from("神崎川")));
    stations.insert(&Station::new(String::from("十三")));
    stations.insert(&Station::new(String::from("中津")));
    stations.insert(&Station::new(String::from("大阪梅田")));

    stations.insert(&Station::new(String::from("稲野")));
    stations.insert(&Station::new(String::from("新伊丹")));
    stations.insert(&Station::new(String::from("伊丹")));

    stations.insert(&Station::new(String::from("今津")));
    stations.insert(&Station::new(String::from("阪神国道")));

    stations.insert(&Station::new(String::from("門戸厄神")));
    stations.insert(&Station::new(String::from("甲東園")));
    stations.insert(&Station::new(String::from("仁川")));
    stations.insert(&Station::new(String::from("小林")));
    stations.insert(&Station::new(String::from("逆瀬川")));
    stations.insert(&Station::new(String::from("宝塚南口")));
    stations.insert(&Station::new(String::from("宝塚")));

    stations.insert(&Station::new(String::from("苦楽園口")));
    stations.insert(&Station::new(String::from("甲陽園")));
    stations
}

fn routes(sta: &StationMap) -> RouteMap {
    let mut routes = RouteMap::default();
    routes.insert(&Route::new(
        String::from("神戸線 (梅田-十三)"),
        &[
            sta.search_from_name("大阪梅田").unwrap().id(),
            sta.search_from_name("中津").unwrap().id(),
            sta.search_from_name("十三").unwrap().id(),
        ],
        &[
            0.0,
            0.9,
            2.4
        ]
    ));
    routes.insert(&Route::new(
        String::from("神戸線 (十三-塚口)"),
        &[
            sta.search_from_name("十三").unwrap().id(),
            sta.search_from_name("神崎川").unwrap().id(),
            sta.search_from_name("園田").unwrap().id(),
            sta.search_from_name("塚口").unwrap().id(),
        ],
        &[
            2.4,
            4.1,
            7.2,
            10.2
        ]
    ));
    routes.insert(&Route::new(
        String::from("神戸線 (塚口-西宮北口)"),
        &[
            sta.search_from_name("塚口").unwrap().id(),
            sta.search_from_name("武庫之荘").unwrap().id(),
            sta.search_from_name("西宮北口").unwrap().id(),
        ],
        &[
            10.2,
            12.3,
            15.6
        ]
    ));
    routes.insert(&Route::new(
        String::from("神戸線 (西宮北口-夙川)"),
        &[
            sta.search_from_name("西宮北口").unwrap().id(),
            sta.search_from_name("夙川").unwrap().id(),
        ],
        &[
            15.6,
            18.3
        ]
    ));
    routes.insert(&Route::new(
        String::from("神戸線 (夙川-神戸三宮)"),
        &[
            sta.search_from_name("夙川").unwrap().id(),
            sta.search_from_name("芦屋川").unwrap().id(),
            sta.search_from_name("岡本").unwrap().id(),
            sta.search_from_name("御影").unwrap().id(),
            sta.search_from_name("六甲").unwrap().id(),
            sta.search_from_name("王子公園").unwrap().id(),
            sta.search_from_name("春日野道").unwrap().id(),
            sta.search_from_name("神戸三宮").unwrap().id(),
        ],
        &[
            18.3,
            21.0,
            23.4,
            25.6,
            27.4,
            29.2,
            30.7,
            32.3
        ]
    ));
    routes.insert(&Route::new(
        String::from("神戸線 (神戸三宮-新開地)"),
        &[
            sta.search_from_name("神戸三宮").unwrap().id(),
            sta.search_from_name("花隈").unwrap().id(),
            sta.search_from_name("高速神戸").unwrap().id(),
            sta.search_from_name("新開地").unwrap().id(),
        ],
        &[
            32.3,
            33.6,
            34.5,
            35.1
        ]
    ));
    routes.insert(&Route::new(
        String::from("伊丹線"),
        &[
            sta.search_from_name("塚口").unwrap().id(),
            sta.search_from_name("稲野").unwrap().id(),
            sta.search_from_name("新伊丹").unwrap().id(),
            sta.search_from_name("伊丹").unwrap().id(),
        ],
        &[
            0.0,
            1.4,
            2.2,
            3.1
        ]
    ));
    routes.insert(&Route::new(
        String::from("今津南線"),
        &[
            sta.search_from_name("西宮北口").unwrap().id(),
            sta.search_from_name("阪神国道").unwrap().id(),
            sta.search_from_name("今津").unwrap().id(),
        ],
        &[
            7.7,
            8.6,
            9.3
        ]
    ));
    routes.insert(&Route::new(
        String::from("今津北線"),
        &[
            sta.search_from_name("西宮北口").unwrap().id(),
            sta.search_from_name("門戸厄神").unwrap().id(),
            sta.search_from_name("甲東園").unwrap().id(),
            sta.search_from_name("仁川").unwrap().id(),
            sta.search_from_name("小林").unwrap().id(),
            sta.search_from_name("逆瀬川").unwrap().id(),
            sta.search_from_name("宝塚南口").unwrap().id(),
            sta.search_from_name("宝塚").unwrap().id(),
        ],
        &[
            7.7,
            6.4,
            5.4,
            4.5,
            2.8,
            1.8,
            0.9,
            0.0
        ]
    ));
    routes.insert(&Route::new(
        String::from("甲陽線"),
        &[
            sta.search_from_name("夙川").unwrap().id(),
            sta.search_from_name("苦楽園口").unwrap().id(),
            sta.search_from_name("甲陽園").unwrap().id(),
        ],
        &[
            0.0,
            0.9,
            2.2
        ]
    ));
    routes
}

fn lines(routes: &RouteMap) -> LineMap {
    let mut lines = LineMap::default();
    lines.insert(&Line::new(
        String::from("神戸本線"),
        &[
            (
                routes.search_from_name("神戸線 (梅田-十三)").unwrap().id(),
                RouteRefDirection::Normal,
            ),
            (
                routes.search_from_name("神戸線 (十三-塚口)").unwrap().id(),
                RouteRefDirection::Normal,
            ),
            (
                routes
                    .search_from_name("神戸線 (塚口-西宮北口)")
                    .unwrap()
                    .id(),
                RouteRefDirection::Normal,
            ),
            (
                routes
                    .search_from_name("神戸線 (西宮北口-夙川)")
                    .unwrap()
                    .id(),
                RouteRefDirection::Normal,
            ),
            (
                routes
                    .search_from_name("神戸線 (夙川-神戸三宮)")
                    .unwrap()
                    .id(),
                RouteRefDirection::Normal,
            ),
            (
                routes
                    .search_from_name("神戸線 (神戸三宮-新開地)")
                    .unwrap()
                    .id(),
                RouteRefDirection::Normal,
            ),
        ],
    ));
    lines.insert(&Line::new(
        String::from("(西宮北口-)伊丹線"),
        &[
            (
                routes
                    .search_from_name("神戸線 (塚口-西宮北口)")
                    .unwrap()
                    .id(),
                RouteRefDirection::Reverse,
            ),
            (
                routes.search_from_name("伊丹線").unwrap().id(),
                RouteRefDirection::Normal,
            ),
        ],
    ));
    lines.insert(&Line::new(
        String::from("(大阪梅田-)今津北線"),
        &[
            (
                routes.search_from_name("神戸線 (梅田-十三)").unwrap().id(),
                RouteRefDirection::Normal,
            ),
            (
                routes.search_from_name("神戸線 (十三-塚口)").unwrap().id(),
                RouteRefDirection::Normal,
            ),
            (
                routes
                    .search_from_name("神戸線 (塚口-西宮北口)")
                    .unwrap()
                    .id(),
                RouteRefDirection::Normal,
            ),
            (
                routes.search_from_name("今津北線").unwrap().id(),
                RouteRefDirection::Normal,
            ),
        ],
    ));
    lines.insert(&Line::new(
        String::from("今津南線"),
        &[(
            routes.search_from_name("今津南線").unwrap().id(),
            RouteRefDirection::Normal,
        )],
    ));
    lines.insert(&Line::new(
        String::from("(西宮北口-)甲陽線"),
        &[
            (
                routes
                    .search_from_name("神戸線 (西宮北口-夙川)")
                    .unwrap()
                    .id(),
                RouteRefDirection::Normal,
            ),
            (
                routes.search_from_name("甲陽線").unwrap().id(),
                RouteRefDirection::Normal,
            ),
        ],
    ));
    lines
}

fn train_type() -> TrainTypeMap {
    let mut train_types = TrainTypeMap::default();
    train_types.insert(&TrainType::new("普通"));
    train_types.insert(&TrainType::new("区間普通"));
    train_types.insert(&TrainType::new("通勤急行"));
    train_types.insert(&TrainType::new("急行"));
    train_types.insert(&TrainType::new("臨時急行"));
    train_types.insert(&TrainType::new("区間急行"));
    train_types.insert(&TrainType::new("準特急"));
    train_types.insert(&TrainType::new("通勤特急"));
    train_types.insert(&TrainType::new("特急"));
    train_types.insert(&TrainType::new("直通特急"));

    train_types
}

fn settings(routes: &RouteMap) -> Settings {
    Settings { diagram_display: DiagramDisplaySettings {
        route_refs: vec![
            (routes.search_from_name("神戸線 (梅田-十三)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("神戸線 (十三-塚口)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("神戸線 (塚口-西宮北口)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("神戸線 (西宮北口-夙川)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("神戸線 (夙川-神戸三宮)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("神戸線 (神戸三宮-新開地)").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("伊丹線").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("今津南線").unwrap().id(), RouteRefDirection::Reverse),
            (routes.search_from_name("今津北線").unwrap().id(), RouteRefDirection::Normal),
            (routes.search_from_name("甲陽線").unwrap().id(), RouteRefDirection::Normal),
        ]
    } }
}

pub fn testdata_hk() -> RootFile {
    let stations = stations();
    let routes = routes(&stations);
    let lines = lines(&routes);
    let train_types = train_type();

    let train1 = Train::new(
        lines.search_from_name("神戸本線").unwrap().id(),
        Direction::Down,
        &lines,
        train_types.search_from_name("直通特急").unwrap().id(),
        &[
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 0, 0))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 1, 30))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 3, 10)), Some(Time::new(13, 3, 45)),
            ),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 5, 5))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 6, 55))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 8, 45))), // 塚口
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 9, 45))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 12, 30)), Some(Time::new(13, 13, 10))), // 西北
            StationTime::new(StopType::Stop, Some(Time::new(13, 15, 40)), Some(Time::new(13, 16, 10))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 18, 15))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 20, 00)), Some(Time::new(13, 20,20 ))), // 岡本
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 22, 20))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 23, 30))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 24, 35))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 25, 40))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 27, 20)), Some(Time::new(13, 28, 0))), // 三宮
            StationTime::new(StopType::Stop, Some(Time::new(13, 29, 50)), Some(Time::new(13, 30, 10))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 32, 0)), Some(Time::new(13, 32, 30))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 34, 0)), None),
        ],
    );
    let train2 = Train::new(
        lines.search_from_name("(西宮北口-)伊丹線").unwrap().id(),
        Direction::Down,
        &lines,
        train_types.search_from_name("区間普通").unwrap().id(),
        &[
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 1, 0))), // 西北
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 2, 0))),
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 3, 0))), // 塚口
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 4, 0))),
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 5, 0))),
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 6, 0))), // 伊丹
        ],
    );
    let train3 = Train::new(
        lines.search_from_name("神戸本線").unwrap().id(),
        Direction::Up,
        &lines,
        train_types.search_from_name("直通特急").unwrap().id(),
        &[
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 0, 0))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 1, 30))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 3, 10)), Some(Time::new(13, 3, 45)),
            ),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 5, 5))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 6, 55))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 8, 45))), // 塚口
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 9, 45))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 12, 30)), Some(Time::new(13, 13, 10))), // 西北
            StationTime::new(StopType::Stop, Some(Time::new(13, 15, 40)), Some(Time::new(13, 16, 10))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 18, 15))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 20, 00)), Some(Time::new(13, 20,20 ))), // 岡本
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 22, 20))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 23, 30))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 24, 35))),
            StationTime::new(StopType::Pass, None, Some(Time::new(13, 25, 40))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 27, 20)), Some(Time::new(13, 28, 0))), // 三宮
            StationTime::new(StopType::Stop, Some(Time::new(13, 29, 50)), Some(Time::new(13, 30, 10))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 32, 0)), Some(Time::new(13, 32, 30))),
            StationTime::new(StopType::Stop, Some(Time::new(13, 34, 0)), None),
        ],
    );
    let train4 = Train::new(
        lines.search_from_name("(西宮北口-)伊丹線").unwrap().id(),
        Direction::Up,
        &lines,
        train_types.search_from_name("区間普通").unwrap().id(),
        &[
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 1, 0))), // 西北
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 2, 0))),
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 3, 0))), // 塚口
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 4, 0))),
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 5, 0))),
            StationTime::new(StopType::Stop, None, Some(Time::new(13, 6, 0))), // 伊丹
        ],
    );

    let root = RootFile {
        stations: stations.clone(),
        routes: routes.clone(),
        lines: lines.clone(),
        train_types: train_types.clone(),
        diagrams: vec![Diagram::new("一号表", &[train1.clone(), train2.clone(), train3.clone(), train4.clone()])],
        settings: settings(&routes),
        ..Default::default()
    };
    root
}

#[test]
fn serialize_test() {
    let _ = std::fs::write("./test.ron", testdata_hk().serialize());
}

#[test]
fn deserialize_test() {
    let _ = RootFile::deserialize(&std::fs::read_to_string("./test.ron").unwrap());
}
