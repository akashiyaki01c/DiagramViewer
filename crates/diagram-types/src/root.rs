use ron::{ser::PrettyConfig, extensions::Extensions};
use serde::{Deserialize, Serialize};

use crate::{
    diagram::Diagram, line::LineMap, route::RouteMap, station::StationMap, train_type::TrainTypeMap, settings::Settings,
};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RootFile {
    /// バージョン
    pub version: FileVersion,
    /// ファイルに含まれる駅一覧
    pub stations: StationMap,
    /// ファイルに含まれる実路線一覧
    pub routes: RouteMap,
    /// ファイルに含まれる運転系統一覧
    pub lines: LineMap,
    /// ファイルに含まれる列車種別一覧
    pub train_types: TrainTypeMap,
    /// ダイヤグラム一覧
    pub diagrams: Vec<Diagram>,
    /// 各種設定
    pub settings: Settings,
}

impl RootFile {
    pub fn serialize(&self) -> String {
        let config = PrettyConfig::new()
            .new_line(String::from("\n"))
            .struct_names(true)
            .extensions(Extensions::all());
        ron::ser::to_string_pretty(&self, config).expect("Serialize Error")
    }

    pub fn deserialize(str: &str) -> Self {
        ron::de::from_str(str).unwrap()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileVersion(pub u8, pub u8);

impl FileVersion {
    pub const VERSION_0_01: Self = Self(0, 1);

    pub fn new(major: u8, minor: u8) -> Self{
        Self(major, minor)
    }
}

impl Default for FileVersion {
    fn default() -> Self {
        Self::VERSION_0_01
    }
}