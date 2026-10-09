use crate::state::DreamSession;
use crate::tools::{call_tool, tool_definitions};
use serde_json::{Value, json};
use std::io::{BufRead, Write};

pub const PROTOCOL_VERSION: &str = "2024-11-05";

pub struct McpServer {
    pub session: DreamSession,
    pub initialized: bool,
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new(DreamSession::new())
    }
}

impl McpServer {
    pub fn new(session: DreamSession) -> Self {
        Self {
            session,
            initialized: false,
        }
    }

    pub async fn handle_line(&mut self, line: &str) -> Option<String> {
        let req: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => {
                return Some(
                    json!({
                        "jsonrpc": "2.0",
                        "id": null,
                        "error": { "code": -32700, "message": "Parse error" }
                    })
                    .to_string(),
                );
            }
        };

        let id = req.get("id").cloned().unwrap_or(Value::Null);
        let method = match req["method"].as_str() {
            Some(m) => m,
            None => return None, // Notification without method? Ignore
        };

        match method {
            "initialize" => {
                self.initialized = true;
                let res = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": PROTOCOL_VERSION,
                        "capabilities": {
                            "tools": { "listChanged": false }
                        },
                        "serverInfo": {
                            "name": "dreamcraft-mcp",
                            "version": "0.1.0",
                            "description": "Unified AI-native creative & productivity suite (Word, Grid, Deck, Film, Photo, Vector, CAD, PDF, AI)"
                        }
                    }
                });
                Some(res.to_string())
            }

            "notifications/initialized" => None,

            "tools/list" => {
                let res = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": tool_definitions()
                    }
                });
                Some(res.to_string())
            }

            "tools/call" => {
                let params = &req["params"];
                let name = params["name"].as_str().unwrap_or_default();
                let args = &params["arguments"];

                let tool_res = call_tool(&self.session, name, args).await;
                let res = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": tool_res.to_value()
                });
                Some(res.to_string())
            }

            _ => Some(
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32601, "message": format!("Method not found: {}", method) }
                })
                .to_string(),
            ),
        }
    }

    pub async fn run_stdio<R: BufRead, W: Write>(&mut self, mut reader: R, mut writer: W) {
        let mut line = String::new();
        while let Ok(bytes) = reader.read_line(&mut line) {
            if bytes == 0 {
                break;
            }
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if let Some(resp) = self.handle_line(trimmed).await {
                    let _ = writeln!(writer, "{}", resp);
                    let _ = writer.flush();
                }
            }
            line.clear();
        }
    }
}
