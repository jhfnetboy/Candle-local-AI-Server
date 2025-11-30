use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tracing::{info, Level};

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ServerInfo {
    name: String,
    version: String,
    status: String,
    framework: String,
}

/// GET / - Server info
async fn get_server_info() -> Json<ApiResponse<ServerInfo>> {
    Json(ApiResponse {
        success: true,
        data: Some(ServerInfo {
            name: "TTS Server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            status: "running".to_string(),
            framework: "Candle".to_string(),
        }),
        error: None,
    })
}

/// GET /health - Health check
async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("healthy".to_string()),
        error: None,
    })
}

#[derive(Debug, Deserialize)]
struct SynthesizeRequest {
    text: String,
    #[serde(default = "default_format")]
    format: String,
}

fn default_format() -> String {
    "wav".to_string()
}

/// POST /synthesize - TTS synthesis
async fn synthesize(
    Json(payload): Json<SynthesizeRequest>
) -> impl IntoResponse {
    info!("🎵 TTS 合成请求: \"{}\"", payload.text);

    // TODO: 实际的 Candle TTS 推理
    // 当前返回模拟响应

    let response = ApiResponse {
        success: true,
        data: Some(format!(
            "Candle TTS 服务已接收: {} (format: {})\n实际音频生成待实现",
            payload.text,
            payload.format
        )),
        error: None,
    };

    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 启动 TTS 服务器 (Candle Framework)...");

    // 创建路由
    let app = Router::new()
        .route("/", get(get_server_info))
        .route("/health", get(health_check))
        .route("/synthesize", post(synthesize))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    // 绑定地址 - 端口 9527
    let addr = SocketAddr::from(([0, 0, 0, 0], 9527));
    info!("🎯 服务器监听地址: {}", addr);
    info!("📡 健康检查: http://localhost:9527/health");
    info!("🎵 TTS 端点: POST http://localhost:9527/synthesize");

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
