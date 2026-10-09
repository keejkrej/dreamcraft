use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeEvent {
    DocumentUpdated { id: String, title: String },
    SheetUpdated { id: String, cells_changed: usize },
    SlideUpdated { id: String, slide_count: usize },
    TimelineUpdated { id: String, duration_seconds: f64 },
    CanvasUpdated { id: String, width: u32, height: u32 },
    AiAssetGenerated { kind: String, uri: String, prompt: String },
}

pub type EventCallback = Arc<dyn Fn(ChangeEvent) + Send + Sync + 'static>;

#[derive(Clone, Default)]
pub struct EventBus {
    listeners: Arc<Mutex<Vec<EventCallback>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(ChangeEvent) + Send + Sync + 'static,
    {
        let mut list = self.listeners.lock().unwrap();
        list.push(Arc::new(callback));
    }

    pub fn emit(&self, event: ChangeEvent) {
        let list = self.listeners.lock().unwrap();
        for cb in list.iter() {
            cb(event.clone());
        }
    }
}
