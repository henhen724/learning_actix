use mongodb::bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPacket {
    pub created: DateTime,
    pub topic: String,
    pub data: Document,
}
