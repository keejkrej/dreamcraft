use crate::Id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetKind {
    Image,
    Video,
    Audio,
    Vector,
    Dataset,
    Document,
    Model3D,
    Font,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_seconds: Option<f64>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u16>,
    pub format: String,
    pub custom: HashMap<String, String>,
}

impl Default for AssetMetadata {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            duration_seconds: None,
            sample_rate: None,
            channels: None,
            format: "unknown".into(),
            custom: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetEntry {
    pub id: Id,
    pub name: String,
    pub kind: AssetKind,
    pub uri_or_path: String,
    pub size_bytes: u64,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub metadata: AssetMetadata,
}

impl AssetEntry {
    pub fn new(name: impl Into<String>, kind: AssetKind, uri_or_path: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            kind,
            uri_or_path: uri_or_path.into(),
            size_bytes: 0,
            tags: Vec::new(),
            created_at: Utc::now(),
            metadata: AssetMetadata::default(),
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AssetCatalog {
    pub assets: HashMap<Id, AssetEntry>,
}

impl AssetCatalog {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn register(&mut self, asset: AssetEntry) -> Id {
        let id = asset.id;
        self.assets.insert(id, asset);
        id
    }

    pub fn get(&self, id: &Id) -> Option<&AssetEntry> {
        self.assets.get(id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&AssetEntry> {
        self.assets.values().find(|a| a.name.eq_ignore_ascii_case(name))
    }

    pub fn list_by_kind(&self, kind: AssetKind) -> Vec<&AssetEntry> {
        self.assets.values().filter(|a| a.kind == kind).collect()
    }

    pub fn list_by_tag(&self, tag: &str) -> Vec<&AssetEntry> {
        self.assets.values().filter(|a| a.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))).collect()
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }
}
