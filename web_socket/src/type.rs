use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub username: String,
    pub content: String,
    pub timestamp: String,
}

impl ChatMessage {
    pub fn new(username: String, content: String) -> Self {
        Self {
            username,
            content,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
