use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::station::StationId;

/// 一つの実路線IDを表す構造体
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct RouteId(pub Uuid);

impl RouteId {
    pub fn get(self, map: &RouteMap) -> Option<&Route> {
        map.0.get(&self)
    }
}

/// 一つの物理的な実路線を表す構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Route {
    /// 固有の路線ID
    id: RouteId,
    /// 実路線名
    pub name: String,
    /// 実路線に含まれる駅のID集合
    station_refs: Vec<StationId>,
    /// 駅間の距離 (累計)
    station_distances: Vec<f32>,
}

impl Route {
    pub fn id(&self) -> RouteId {
        self.id
    }

    pub fn station_refs(&self) -> &[StationId] {
        &self.station_refs
    }
    pub fn station_refs_mut(&mut self) -> &mut [StationId] {
        &mut self.station_refs
    }
    pub fn station_refs_vec(&self) -> &Vec<StationId> {
        &self.station_refs
    }

    pub fn station_distances(&self) -> &[f32] {
        &self.station_distances
    }
    pub fn station_distances_mut(&mut self) -> &mut [f32] {
        &mut self.station_distances
    }
    pub fn station_distances_vec(&self) -> &Vec<f32> {
        &self.station_distances
    }

    pub fn new(name: String, station_refs: &[StationId], distances: &[f32]) -> Self {
        assert_eq!(station_refs.len(), distances.len());
        Self {
            id: RouteId(Uuid::new_v4()),
            name,
            station_refs: Vec::from(station_refs),
            station_distances: Vec::from(distances),
        }
    }
}

/// 実路線を管理する構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RouteMap(HashMap<RouteId, Route>);

impl RouteMap {
    pub fn get(&self) -> &HashMap<RouteId, Route> {
        &self.0
    }
    pub unsafe fn get_mut(&mut self) -> &HashMap<RouteId, Route> {
        &mut self.0
    }

    pub fn insert(&mut self, route: &Route) -> Option<Route> {
        self.0.insert(route.id, route.clone())
    }

    /// 実路線名から実路線を探索する関数
    pub fn search_from_name(&self, name: &str) -> Option<&Route> {
        for value in &self.0 {
            if &value.1.name == name {
                return Some(value.1);
            }
        }
        None
    }
}
