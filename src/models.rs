use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataA {
    pub version: String,
    pub topic: Option<String>,
    pub content: String,
    pub time_recived: i64,
}