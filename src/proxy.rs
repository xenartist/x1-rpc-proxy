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
use tokio::sync::Semaphore;
use tracing::{error, info, warn, debug};
use tower_http::cors::{CorsLayer, Any};

use crate::node_cache::NodeCache;
use crate::rpc_client::forward_rpc_request;
use crate::types::{RpcRequest, RpcResponse, RpcError};

pub struct ProxyServer {
    node_cache: Arc<NodeCache>,
    rpc_timeout: u64,
    rpc_semaphore: Arc<Semaphore>,
    max_concurrent: usize,
    max_queue_wait_time: u64,
}

impl ProxyServer {
    pub fn new(
        node_cache: Arc<NodeCache>, 
        rpc_timeout: u64, 
        max_concurrent_rpc: usize,
        max_queue_wait_time: u64
    ) -> Self {
        info!("🚦 Setting up RPC request queue with max {} concurrent requests, queue wait time: {}s", 
              max_concurrent_rpc, max_queue_wait_time);
        Self {
            node_cache,
            rpc_timeout,
            rpc_semaphore: Arc::new(Semaphore::new(max_concurrent_rpc)),
            max_concurrent: max_concurrent_rpc,
            max_queue_wait_time,
        }
    }
    
    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/", post(rpc_handler))
            .route("/health", axum::routing::get(health_handler))
            .route("/stats", axum::routing::get(stats_handler))
            .route("/performance", axum::routing::get(performance_handler))
            .route("/queue", axum::routing::get(queue_stats_handler))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any)
            )
            .with_state(AppState {
                node_cache: Arc::clone(&self.node_cache),
                rpc_timeout: self.rpc_timeout,
                rpc_semaphore: Arc::clone(&self.rpc_semaphore),
                max_concurrent: self.max_concurrent,
                max_queue_wait_time: self.max_queue_wait_time,
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
    rpc_semaphore: Arc<Semaphore>,
    max_concurrent: usize,
    max_queue_wait_time: u64,
}

fn format_rpc_request_info(request: &RpcRequest) -> String {
    let method = &request.method;
    let id = &request.id;
    
    let params_preview = match &request.params {
        Some(params) => {
            let params_str = serde_json::to_string(params).unwrap_or_default();
            if params_str.len() > 100 {
                format!("{}...", &params_str[..97])
            } else {
                params_str
            }
        },
        None => "null".to_string(),
    };
    
    match method.as_str() {
        "getTokenAccountsByOwner" => {
            if let Some(params) = &request.params {
                if let Some(params_array) = params.as_array() {
                    if let Some(owner) = params_array.get(0) {
                        if let Some(owner_str) = owner.as_str() {
                            return format!("getTokenAccountsByOwner(owner={}..., id={})", 
                                         &owner_str[..8.min(owner_str.len())], id);
                        }
                    }
                }
            }
            format!("getTokenAccountsByOwner(id={})", id)
        },
        "getAccountInfo" => {
            if let Some(params) = &request.params {
                if let Some(params_array) = params.as_array() {
                    if let Some(account) = params_array.get(0) {
                        if let Some(account_str) = account.as_str() {
                            return format!("getAccountInfo(account={}..., id={})", 
                                         &account_str[..8.min(account_str.len())], id);
                        }
                    }
                }
            }
            format!("getAccountInfo(id={})", id)
        },
        "sendTransaction" => format!("sendTransaction(id={})", id),
        "simulateTransaction" => format!("simulateTransaction(id={})", id),
        "getBalance" => format!("getBalance(id={})", id),
        "getRecentBlockhash" => format!("getRecentBlockhash(id={})", id),
        "getSlot" => format!("getSlot(id={})", id),
        "getBlockHeight" => format!("getBlockHeight(id={})", id),
        "getHealth" => format!("getHealth(id={})", id),
        "getVersion" => format!("getVersion(id={})", id),
        "getEpochInfo" => format!("getEpochInfo(id={})", id),
        "getLatestBlockhash" => format!("getLatestBlockhash(id={})", id),
        _ => format!("{}(params={}, id={})", method, params_preview, id),
    }
}

async fn rpc_handler(
    State(state): State<AppState>,
    Json(request): Json<RpcRequest>,
) -> Result<Json<RpcResponse>, (StatusCode, Json<RpcResponse>)> {
    let start_time = std::time::Instant::now();
    let request_info = format_rpc_request_info(&request);
    let available_permits = state.rpc_semaphore.available_permits();
    
    info!("📨 Incoming RPC request: {} (queue: {}/{})", 
          request_info, state.max_concurrent - available_permits, state.max_concurrent);
    
    // get permit to implement queue mechanism
    let _permit = match state.rpc_semaphore.try_acquire() {
        Ok(permit) => {
            debug!("✅ Acquired RPC permit immediately for [{}]", request.method);
            permit
        },
        Err(_) => {
            info!("⏳ RPC request [{}] entering queue, waiting for available slot...", request.method);
            
            // use configurable wait time
            match tokio::time::timeout(
                std::time::Duration::from_secs(state.max_queue_wait_time), 
                state.rpc_semaphore.acquire()
            ).await {
                Ok(Ok(permit)) => {
                    let wait_time = start_time.elapsed();
                    info!("✅ RPC request [{}] acquired permit after waiting {:?}", 
                          request.method, wait_time);
                    permit
                },
                Ok(Err(_)) => {
                    error!("💀 RPC semaphore was closed for [{}]", request.method);
                    let error_response = RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(RpcError {
                            code: -32000,
                            message: "Server shutting down".to_string(),
                            data: None,
                        }),
                    };
                    return Err((StatusCode::SERVICE_UNAVAILABLE, Json(error_response)));
                },
                Err(_) => {
                    let wait_time = start_time.elapsed();
                    warn!("⏰ RPC request [{}] queue timeout after {:?}, server overloaded", 
                          request.method, wait_time);
                    let error_response = RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(RpcError {
                            code: -32000,
                            message: "Server overloaded, request queue full".to_string(),
                            data: Some(json!({
                                "queue_wait_time_ms": wait_time.as_millis(),
                                "max_queue_wait_ms": state.max_queue_wait_time * 1000
                            })),
                        }),
                    };
                    return Err((StatusCode::SERVICE_UNAVAILABLE, Json(error_response)));
                }
            }
        }
    };

    // add small delay for single-core CPU to allow system to handle other tasks
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;

    // process request
    let processing_start = std::time::Instant::now();
    
    match state.node_cache.get_random_fast_node().await {
        Some(node) => {
            let response_time_info = node.response_time
                .map(|t| format!(" (avg response: {:?})", t))
                .unwrap_or_else(|| " (no timing data)".to_string());
            
            info!("🚀 Processing RPC request [{}] to node: {}{}", 
                  request.method, node.endpoint, response_time_info);
            
            match forward_rpc_request(&node.endpoint, &request, state.rpc_timeout).await {
                Ok(response) => {
                    let processing_time = processing_start.elapsed();
                    let total_time = start_time.elapsed();
                    info!("✅ RPC request [{}] completed - processing: {:?}, total: {:?}", 
                          request.method, processing_time, total_time);
                    Ok(Json(response))
                },
                Err(e) => {
                    let processing_time = processing_start.elapsed();
                    let total_time = start_time.elapsed();
                    error!("❌ RPC request [{}] failed - processing: {:?}, total: {:?}, error: {}", 
                           request.method, processing_time, total_time, e);
                    let error_response = RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(RpcError {
                            code: -32603,
                            message: "Internal error".to_string(),
                            data: Some(json!({
                                "details": e.to_string(),
                                "processing_time_ms": processing_time.as_millis(),
                                "total_time_ms": total_time.as_millis()
                            })),
                        }),
                    };
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                }
            }
        }
        None => {
            let total_time = start_time.elapsed();
            warn!("💥 No available RPC nodes for [{}] after {:?}", request.method, total_time);
            let error_response = RpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(RpcError {
                    code: -32000,
                    message: "No available RPC nodes".to_string(),
                    data: Some(json!({
                        "total_time_ms": total_time.as_millis()
                    })),
                }),
            };
            Err((StatusCode::SERVICE_UNAVAILABLE, Json(error_response)))
        }
    }
    // _permit is automatically released here
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

// add queue status monitoring endpoint
async fn queue_stats_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let available_permits = state.rpc_semaphore.available_permits();
    let active_requests = state.max_concurrent - available_permits;
    
    Json(json!({
        "queue_status": {
            "max_concurrent_requests": state.max_concurrent,
            "active_requests": active_requests,
            "available_slots": available_permits,
            "queue_full": available_permits == 0
        }
    }))
} 