pub mod storage;
pub mod format;
pub mod graph;
pub mod cli;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum ResourceType {
    Texture,
    Model3D,
    Audio,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ResourceMetadata {
    pub hash: String,
    pub resource_type: ResourceType,
    pub size: u64,
    pub create_at: i64,
    pub dependencies: Vec<String>,
}