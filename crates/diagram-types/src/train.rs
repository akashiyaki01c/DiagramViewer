use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    line::{LineId, LineMap, RouteRefDirection},
    root::RootFile,
    station::Station,
    train_type::TrainTypeId,
    Time,
};

/// 一つの運転系統を表す構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Train {
    /// 固有の運転系統ID
    id: Uuid,
    /// 列車番号
    pub number: String,
    /// 列車名
    pub nickname: String,
    /// 列車種別ID
    train_type_id: TrainTypeId,
    /// 運転系統ID
    line_id: LineId,
    /// 運転方向
    pub direction: Direction,
    /// 駅時刻
    times: Vec<StationTime>,
}

impl Train {
    pub fn new(
        line_id: LineId,
        direction: Direction,
        lines: &LineMap,
        train_type_id: TrainTypeId,
        times: &[StationTime],
    ) -> Self {
        if let Some(_line) = lines.get().get(&line_id) {
            // let line_len = line.route_refs().len();
            // let times: Vec<StationTime> = vec![StationTime::default(); line_len];
            Self {
                id: Uuid::new_v4(),
                line_id,
                direction,
                times: Vec::from(times),
                train_type_id,
                ..Self::default()
            }
        } else {
            panic!("指定のLineIDが存在しません。");
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn line_id(&self) -> LineId {
        self.line_id
    }
    pub fn train_type_id(&self) -> TrainTypeId {
        self.train_type_id
    }

    pub fn times(&self) -> &[StationTime] {
        self.times.as_ref()
    }
    pub fn times_mut(&mut self) -> &mut [StationTime] {
        self.times.as_mut()
    }

    pub fn get_stations<'a>(&'a self, root: &'a RootFile) -> Vec<&Station> {
        let mut stations_id: Vec<Station> = vec![];
        for line in root.lines.get().get(&self.line_id).unwrap().route_refs() {
            match line.1 {
                RouteRefDirection::Normal => {
                    if let Some(route) = root.routes.get().get(&line.0) {
                        for sta in route.station_refs() {
                            if stations_id.is_empty() || stations_id.last().unwrap().id().0 != sta.0
                            {
                                stations_id.push(root.stations.get().get(sta).unwrap().clone());
                            }
                        }
                    }
                }
                RouteRefDirection::Reverse => {
                    if let Some(route) = root.routes.get().get(&line.0) {
                        for sta in route.station_refs().iter().rev() {
                            if stations_id.is_empty() || stations_id.last().unwrap().id().0 != sta.0
                            {
                                stations_id.push(root.stations.get().get(sta).unwrap().clone());
                            }
                        }
                    }
                }
            };
        }
        stations_id
            .iter()
            .map(|sta| root.stations.get().get(&sta.id()).unwrap())
            .collect::<Vec<&Station>>()
    }
}

/// 一つの駅時刻を表す構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct StationTime {
    /// 着時刻
    pub arrive: Option<Time>,
    /// 発時刻(通時刻)
    pub departure: Option<Time>,
    /// 停車種別
    pub stop_type: StopType,
}

impl StationTime {
    pub fn new(stop_type: StopType, arrive: Option<Time>, departure: Option<Time>) -> Self {
        Self {
            stop_type,
            arrive,
            departure,
        }
    }

    pub fn to_time_string(&self) -> String {
        match self.stop_type {
            StopType::None => format!(" {: ^20} ", "…"),
            StopType::Pass => {
                let dep = if self.departure.is_some() { self.departure.unwrap().to_string() } else { String::from("---") };
                format!(" {: ^8} /({: ^8})", "ﾚ", dep)
            }
            StopType::Stop => {
                let arr = if self.arrive.is_some() { self.arrive.unwrap().to_string() } else { String::from("---") };
                let dep = if self.departure.is_some() { self.departure.unwrap().to_string() } else { String::from("---") };
                format!(" {: ^8} / {: ^8} ", arr, dep)
            }
        }
    }
}

/// 停車種別を表す列挙体
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum StopType {
    None,
    Stop,
    Pass,
}

impl Default for StopType {
    fn default() -> Self {
        Self::None
    }
}

/// 運転方向を表す列挙体
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Direction {
    Down,
    Up,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Down
    }
}