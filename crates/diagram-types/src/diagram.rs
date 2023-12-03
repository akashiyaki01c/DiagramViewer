use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::train::Train;

/// 一つのダイヤグラムを表す構造体
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Diagram {
    /// 固有のダイヤグラムID
    id: Uuid,
    /// ダイヤグラム名
    pub name: String,
    /// 上下列車の集合
    pub trains: Vec<Train>,
}

impl Diagram {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn new(name: &str, trains: &[Train]) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from(name),
            trains: Vec::from(trains),
        }
    }
}
