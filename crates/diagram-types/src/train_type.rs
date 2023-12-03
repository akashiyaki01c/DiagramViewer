use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 一つの列車種別IDを表す構造体
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TrainTypeId(pub Uuid);

impl TrainTypeId {
    pub fn get(self, map: &TrainTypeMap) -> Option<&TrainType> {
        map.0.get(&self)
    }
}

/// 一つの列車種別を表す構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TrainType {
    /// 列車種別固有のID
    id: TrainTypeId,
    /// 列車種別名
    pub name: String,
}

impl TrainType {
    pub fn id(&self) -> TrainTypeId {
        self.id
    }

    pub fn new(name: &str) -> Self {
        Self {
            id: TrainTypeId(Uuid::new_v4()),
            name: String::from(name),
            ..Default::default()
        }
    }
}

/// 列車種別を管理する構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TrainTypeMap(HashMap<TrainTypeId, TrainType>);

impl TrainTypeMap {
    pub fn get(&self) -> &HashMap<TrainTypeId, TrainType> {
        &self.0
    }
    pub unsafe fn get_mut(&mut self) -> &HashMap<TrainTypeId, TrainType> {
        &mut self.0
    }

    pub fn insert(&mut self, train_type: &TrainType) -> Option<TrainType> {
        self.0.insert(train_type.id, train_type.clone())
    }

    /// 種別名から種別を探索する関数
    pub fn search_from_name(&self, name: &str) -> Option<&TrainType> {
        for value in &self.0 {
            if &value.1.name == name {
                return Some(value.1);
            }
        }
        None
    }
}
