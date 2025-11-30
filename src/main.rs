use axum::{
    routing::{get, post},
    Router,
    Json,
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::OnceLock;
use tower_http::cors::{CorsLayer, Any};
use tracing::{info, error, Level};

mod tts_engine;
mod wav_encoder;

use tts_engine::TTSEngine;
use wav_encoder::encode_wav;

// 全局 TTS 引擎 (单例模式)
static TTS_ENGINE: OnceLock<TTSEngine> = OnceLock::new();

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

    // 获取或初始化 TTS 引擎
    let engine = TTS_ENGINE.get_or_init(|| {
        info!("🔧 首次初始化 TTS 引擎...");

        match TTSEngine::new("checkpoints/kokoro-v1.0.onnx") {
            Ok(engine) => {
                info!("✅ TTS 引擎初始化成功");
                engine
            },
            Err(e) => {
                error!("❌ TTS 引擎初始化失败: {}", e);
                // 返回 mock 引擎作为降级
                panic!("无法加载 TTS 模型: {}", e);
            }
        }
    });

    // 合成音频 (当前使用 Mock 实现)
    match engine.synthesize(&payload.text) {
        Ok(audio_samples) => {
            info!("✅ 音频合成成功 ({} 样本)", audio_samples.len());

            // 编码为 WAV
            match encode_wav(&audio_samples, engine.sample_rate()) {
                Ok(wav_bytes) => {
                    info!("✅ WAV 编码完成 ({} 字节)", wav_bytes.len());

                    // 返回 WAV 音频
                    (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "audio/wav")],
                        wav_bytes
                    )
                },
                Err(e) => {
                    error!("❌ WAV 编码失败: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        [(header::CONTENT_TYPE, "audio/wav")],
                        Vec::new()
                    )
                }
            }
        },
        Err(e) => {
            error!("❌ 音频合成失败: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "audio/wav")],
                Vec::new()
            )
        }
    }
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
