use serde::{Deserialize, Serialize};

use crate::{route::RouteId, line::RouteRefDirection};


/// 各種ファイルの設定
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Settings {
	pub diagram_display: DiagramDisplaySettings,
}

/// ダイヤグラムの表示設定
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct DiagramDisplaySettings {
	/// ダイヤグラムに表示する実路線順序
    pub route_refs: Vec<(RouteId, RouteRefDirection)>,
}