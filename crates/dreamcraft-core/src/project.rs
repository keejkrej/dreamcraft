use crate::asset::AssetCatalog;
use crate::command::CommandHistory;
use crate::Id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DreamProject {
    pub id: Id,
    pub name: String,
    pub author: String,
    pub asset_catalog: AssetCatalog,
    pub command_history: CommandHistory,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

impl Default for DreamProject {
    fn default() -> Self {
        Self::new("Untitled Project")
    }
}

impl DreamProject {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            name: name.into(),
            author: "AI Agent".into(),
            asset_catalog: AssetCatalog::new(),
            command_history: CommandHistory::new(500),
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
        self.touch();
    }
}
