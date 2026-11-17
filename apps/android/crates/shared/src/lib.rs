use serde::{Deserialize, Serialize};

/// A message returned by the Nexured message API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: i64,
    pub sender: String,
    pub body: String,
    pub created_at: String,
}

/// Payload accepted when creating a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessage {
    pub sender: String,
    pub body: String,
}
