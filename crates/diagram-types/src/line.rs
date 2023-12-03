use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::route::RouteId;

/// 一つの運転系統IDを表す構造体
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct LineId(pub Uuid);

impl LineId {
    pub fn get(self, map: &LineMap) -> Option<&Line> {
        map.0.get(&self)
    }
}

/// 一つの運転系統を表す構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Line {
    /// 固有の運転系統ID
    id: LineId,
    /// 運転系統名
    pub name: String,
    /// 運転系統に含まれる実路線のID集合
    route_refs: Vec<(RouteId, RouteRefDirection)>,
}

impl Line {
    pub fn id(&self) -> LineId {
        self.id
    }

    pub fn route_refs(&self) -> &[(RouteId, RouteRefDirection)] {
        &self.route_refs
    }

    pub fn route_refs_mut(&mut self) -> &mut [(RouteId, RouteRefDirection)] {
        &mut self.route_refs
    }

    pub unsafe fn route_refs_vec(&mut self) -> &mut Vec<(RouteId, RouteRefDirection)> {
        &mut self.route_refs
    }

    pub fn new(name: String, route_refs: &[(RouteId, RouteRefDirection)]) -> Self {
        Self {
            id: LineId(Uuid::new_v4()),
            name,
            route_refs: Vec::from(route_refs),
            ..Default::default()
        }
    }
}

/// 運転系統が運転する実路線の方向を定義
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum RouteRefDirection {
    Normal,
    Reverse,
}
impl Default for RouteRefDirection {
    fn default() -> Self {
        Self::Normal
    }
}

/// 運転系統を管理する構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LineMap(HashMap<LineId, Line>);

impl LineMap {
    pub fn get(&self) -> &HashMap<LineId, Line> {
        &self.0
    }
    pub unsafe fn get_mut(&mut self) -> &HashMap<LineId, Line> {
        &mut self.0
    }

    pub fn insert(&mut self, line: &Line) -> Option<Line> {
        self.0.insert(line.id, line.clone())
    }

    /// 運転系統名から運転系統を探索する関数
    pub fn search_from_name(&self, name: &str) -> Option<&Line> {
        for value in &self.0 {
            if &value.1.name == name {
                return Some(value.1);
            }
        }
        None
    }
}
