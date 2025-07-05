use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
    serve,
};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info, warn};
use tower_http::cors::{CorsLayer, Any};

use crate::node_cache::NodeCache;
use crate::rpc_client::forward_rpc_request;
use crate::types::{RpcRequest, RpcResponse, RpcError};

pub struct ProxyServer {
    node_cache: Arc<NodeCache>,
    rpc_timeout: u64,
}

impl ProxyServer {
    pub fn new(node_cache: Arc<NodeCache>, rpc_timeout: u64) -> Self {
        Self {
            node_cache,
            rpc_timeout,
        }
    }
    
    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/", post(rpc_handler))
            .route("/health", axum::routing::get(health_handler))
            .route("/stats", axum::routing::get(stats_handler))
            .route("/performance", axum::routing::get(performance_handler))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
            )
            .with_state(AppState {
                node_cache: Arc::clone(&self.node_cache),
                rpc_timeout: self.rpc_timeout,
            });
        
        let addr = format!("0.0.0.0:{}", port);
        info!("RPC proxy server starting on: {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        serve(listener, app).await?;
        
        Ok(())
    }
}

#[derive(Clone)]
struct AppState {
    node_cache: Arc<NodeCache>,
    rpc_timeout: u64,
}

async fn rpc_handler(
    State(state): State<AppState>,
    Json(request): Json<RpcRequest>,
) -> Result<Json<RpcResponse>, (StatusCode, Json<RpcResponse>)> {
    // Get a random node from the top 100 fastest nodes
    match state.node_cache.get_random_fast_node().await {
        Some(node) => {
            let response_time_info = node.response_time
                .map(|t| format!(" (avg response: {:?})", t))
                .unwrap_or_else(|| " (no timing data)".to_string());
            
            info!("Forwarding RPC request to fast node: {}{}", node.endpoint, response_time_info);
            
            // Forward the request
            match forward_rpc_request(&node.endpoint, &request, state.rpc_timeout).await {
                Ok(response) => Ok(Json(response)),
                Err(e) => {
                    error!("RPC forwarding failed: {}", e);
                    let error_response = RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(RpcError {
                            code: -32603,
                            message: "Internal error".to_string(),
                            data: Some(json!({"details": e.to_string()})),
                        }),
                    };
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                }
            }
        }
        None => {
            warn!("No available RPC nodes");
            let error_response = RpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(RpcError {
                    code: -32000,
                    message: "No available RPC nodes".to_string(),
                    data: None,
                }),
            };
            Err((StatusCode::SERVICE_UNAVAILABLE, Json(error_response)))
        }
    }
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "x1-rpc-proxy"
    }))
}

async fn stats_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let (total, active) = state.node_cache.get_node_stats().await;
    
    Json(json!({
        "total_nodes": total,
        "active_nodes": active,
        "uptime": "running"
    }))
}

async fn performance_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let (total, active, min_response, max_response) = state.node_cache.get_performance_stats().await;
    
    Json(json!({
        "total_nodes": total,
        "active_nodes": active,
        "min_response_time_ms": min_response.map(|t| t.as_millis()),
        "max_response_time_ms": max_response.map(|t| t.as_millis()),
        "performance_optimization": "top_100_fastest_nodes"
    }))
} 