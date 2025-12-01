use axum::{
    extract::Path,
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

mod cache;
mod tts_engine;
mod vocab;
mod wav_encoder;

use cache::AudioCache;
use tts_engine::TTSEngine;
use wav_encoder::encode_wav;

// 全局 TTS 引擎 (单例模式)
static TTS_ENGINE: OnceLock<std::sync::Mutex<TTSEngine>> = OnceLock::new();

// 全局音频缓存 (单例模式)
static AUDIO_CACHE: OnceLock<AudioCache> = OnceLock::new();

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
    #[allow(dead_code)]
    format: String,  // 保留用于未来扩展 (mp3, ogg等)
    voice: Option<String>,  // 可选的声音参数
}

fn default_format() -> String {
    "wav".to_string()
}

#[derive(Debug, Serialize)]
struct SynthesizeResponse {
    file_id: String,
    url: String,
    cached: bool,
}

/// POST /synthesize - TTS synthesis (使用文件缓存)
async fn synthesize(
    Json(payload): Json<SynthesizeRequest>
) -> impl IntoResponse {
    info!("🎵 TTS 合成请求: \"{}\"", &payload.text[..payload.text.len().min(50)]);

    // 获取或初始化缓存
    let cache = AUDIO_CACHE.get_or_init(|| {
        info!("🔧 初始化音频缓存...");
        AudioCache::new("cache/audio", 3600).expect("无法初始化缓存")
    });

    // 检查缓存 (包含声音参数)
    if let Some(file_id) = cache.exists(&payload.text, payload.voice.as_deref()) {
        info!("✅ 缓存命中: {}", file_id);

        let response = SynthesizeResponse {
            file_id: file_id.clone(),
            url: format!("http://localhost:9527/audio/{}.wav", file_id),
            cached: true,
        };

        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            serde_json::to_string(&response).unwrap()
        );
    }

    info!("❌ 缓存未命中，开始合成...");

    // 获取或初始化 TTS 引擎
    let engine_mutex = TTS_ENGINE.get_or_init(|| {
        info!("🔧 首次初始化 TTS 引擎...");

        match TTSEngine::new("checkpoints/kokoro-v1.0.onnx") {
            Ok(engine) => {
                info!("✅ TTS 引擎初始化成功");
                std::sync::Mutex::new(engine)
            },
            Err(e) => {
                error!("❌ TTS 引擎初始化失败: {}", e);
                panic!("无法加载 TTS 模型: {}", e);
            }
        }
    });

    let mut engine = engine_mutex.lock().unwrap();

    // 合成音频 (传递 voice 参数)
    match engine.synthesize(&payload.text, payload.voice.as_deref()) {
        Ok(audio_samples) => {
            info!("✅ 音频合成成功 ({} 样本)", audio_samples.len());

            // 编码为 WAV
            match encode_wav(&audio_samples, engine.sample_rate()) {
                Ok(wav_bytes) => {
                    info!("✅ WAV 编码完成 ({} 字节)", wav_bytes.len());

                    // 保存到缓存 (包含声音参数)
                    match cache.save(&payload.text, payload.voice.as_deref(), &wav_bytes) {
                        Ok(file_id) => {
                            let response = SynthesizeResponse {
                                file_id: file_id.clone(),
                                url: format!("http://localhost:9527/audio/{}.wav", file_id),
                                cached: false,
                            };

                            (
                                StatusCode::OK,
                                [(header::CONTENT_TYPE, "application/json")],
                                serde_json::to_string(&response).unwrap()
                            )
                        },
                        Err(e) => {
                            error!("❌ 缓存保存失败: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                [(header::CONTENT_TYPE, "application/json")],
                                format!(r#"{{"error": "缓存保存失败: {}"}}"#, e)
                            )
                        }
                    }
                },
                Err(e) => {
                    error!("❌ WAV 编码失败: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        [(header::CONTENT_TYPE, "application/json")],
                        format!(r#"{{"error": "WAV 编码失败: {}"}}"#, e)
                    )
                }
            }
        },
        Err(e) => {
            error!("❌ 音频合成失败: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "application/json")],
                format!(r#"{{"error": "音频合成失败: {}"}}"#, e)
            )
        }
    }
}

/// GET /audio/:filename - 静态音频文件服务
async fn serve_audio(Path(filename): Path<String>) -> impl IntoResponse {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    info!("📁 请求音频文件: {}", filename);

    // 安全检查: 只允许 .wav 文件
    if !filename.ends_with(".wav") {
        error!("❌ 非法文件扩展名: {}", filename);
        return (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "text/plain")],
            Vec::new()
        );
    }

    // 构建文件路径
    let file_path = format!("cache/audio/{}", filename);

    // 读取文件
    match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            match file.read_to_end(&mut contents).await {
                Ok(_) => {
                    info!("✅ 读取音频文件: {} ({} 字节)", filename, contents.len());
                    (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "audio/wav")],
                        contents
                    )
                },
                Err(e) => {
                    error!("❌ 读取文件失败: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        [(header::CONTENT_TYPE, "text/plain")],
                        Vec::new()
                    )
                }
            }
        },
        Err(_) => {
            error!("❌ 文件不存在: {}", filename);
            (
                StatusCode::NOT_FOUND,
                [(header::CONTENT_TYPE, "text/plain")],
                Vec::new()
            )
        }
    }
}

/// 检查并下载模型文件
async fn ensure_models_downloaded() -> anyhow::Result<()> {
    use tokio::process::Command;
    use std::path::Path;

    let model_path = Path::new("checkpoints/kokoro-v1.0.onnx");

    if !model_path.exists() {
        info!("📥 模型文件不存在，开始自动下载...");
        info!("⏳ 这可能需要几分钟时间 (模型约 310MB)...");

        // 运行下载脚本
        let output = Command::new("bash")
            .arg("download_models.sh")
            .output()
            .await?;

        if output.status.success() {
            info!("✅ 模型下载完成");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("❌ 模型下载失败: {}", stderr);
            return Err(anyhow::anyhow!("模型下载失败"));
        }
    } else {
        info!("✅ 模型文件已存在，跳过下载");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 启动 TTS 服务器 (Candle Framework)...");

    // 确保模型已下载
    ensure_models_downloaded().await?;

    // 创建路由
    let app = Router::new()
        .route("/", get(get_server_info))
        .route("/health", get(health_check))
        .route("/synthesize", post(synthesize))
        .route("/audio/:filename", get(serve_audio))
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
