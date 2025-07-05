use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcNode {
    pub endpoint: String,
    pub last_seen: std::time::SystemTime,
    pub response_time: Option<Duration>,
    pub is_active: bool,
}

impl RpcNode {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            last_seen: std::time::SystemTime::now(),
            response_time: None,
            is_active: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
} 