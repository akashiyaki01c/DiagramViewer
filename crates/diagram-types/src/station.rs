use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 一つの駅IDを表す構造体
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct StationId(pub Uuid);

impl StationId {
    pub fn get(self, map: &StationMap) -> Option<&Station> {
        map.0.get(&self)
    }
}

/// 一つの駅を表す構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Station {
    /// 駅固有のID
    id: StationId,
    /// 駅名
    pub name: String,
}

impl Station {
    pub fn id(&self) -> StationId {
        self.id
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: StationId(Uuid::new_v4()),
            ..Default::default()
        }
    }
}

/// 駅集合を管理する構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct StationMap(HashMap<StationId, Station>);

impl StationMap {
    pub fn get(&self) -> &HashMap<StationId, Station> {
        &self.0
    }
    pub unsafe fn get_mut(&mut self) -> &HashMap<StationId, Station> {
        &mut self.0
    }

    pub fn insert(&mut self, station: &Station) -> Option<Station> {
        self.0.insert(station.id, station.clone())
    }

    pub fn new(hashmap: HashMap<StationId, Station>) -> Self {
        Self(hashmap)
    }

    /// 駅名から駅を探索する関数
    pub fn search_from_name(&self, name: &str) -> Option<&Station> {
        for value in &self.0 {
            if &value.1.name == name {
                return Some(value.1);
            }
        }
        None
    }
}
