use thiserror::Error;

#[derive(Debug, Error)]
pub enum DreamError {
    #[error("Document error: {0}")]
    Document(String),

    #[error("Grid / Formula error: {0}")]
    Grid(String),

    #[error("Timeline edit error: {0}")]
    Timeline(String),

    #[error("Canvas / Graphic error: {0}")]
    Graphic(String),

    #[error("AI generation error: {0}")]
    Ai(String),

    #[error("MCP error: {0}")]
    Mcp(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Operation failed: {0}")]
    General(String),
}

pub type Result<T> = std::result::Result<T, DreamError>;
