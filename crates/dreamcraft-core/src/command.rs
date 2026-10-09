use crate::Id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandSpec {
    pub id: &'static str,
    pub domain: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub params_schema: &'static str,
    pub mutates: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandInvocation {
    pub command: String,
    pub params: Value,
}

impl CommandInvocation {
    pub fn new(command: impl Into<String>, params: Value) -> Self {
        Self {
            command: command.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandRecord {
    pub id: Id,
    pub command: String,
    pub params: Value,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CommandHistory {
    pub past: Vec<CommandRecord>,
    pub future: Vec<CommandRecord>,
    pub max_history: usize,
}

impl CommandHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            past: Vec::new(),
            future: Vec::new(),
            max_history,
        }
    }

    pub fn record_success(&mut self, command: impl Into<String>, params: Value, result: Value) {
        let record = CommandRecord {
            id: Id::new(),
            command: command.into(),
            params,
            timestamp: Utc::now(),
            success: true,
            result: Some(result),
            error: None,
        };
        self.past.push(record);
        if self.past.len() > self.max_history && self.max_history > 0 {
            self.past.remove(0);
        }
        self.future.clear();
    }

    pub fn record_failure(&mut self, command: impl Into<String>, params: Value, error: String) {
        let record = CommandRecord {
            id: Id::new(),
            command: command.into(),
            params,
            timestamp: Utc::now(),
            success: false,
            result: None,
            error: Some(error),
        };
        self.past.push(record);
    }

    pub fn last_command(&self) -> Option<&CommandRecord> {
        self.past.last()
    }
}

pub struct CommandRegistry {
    specs: Vec<CommandSpec>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self { specs: Vec::new() }
    }

    pub fn register(&mut self, spec: CommandSpec) {
        self.specs.push(spec);
    }

    pub fn get(&self, id: &str) -> Option<&CommandSpec> {
        self.specs.iter().find(|s| s.id == id)
    }

    pub fn list_by_domain(&self, domain: &str) -> Vec<&CommandSpec> {
        self.specs.iter().filter(|s| s.domain == domain).collect()
    }

    pub fn all(&self) -> &[CommandSpec] {
        &self.specs
    }

    pub fn describe_catalog(&self) -> Value {
        serde_json::to_value(&self.specs).unwrap_or(Value::Array(Vec::new()))
    }
}
