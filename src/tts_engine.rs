/// TTS Engine - Kokoro-82M (Simplified)
///
/// 当前实现: Mock TTS (生成静音或测试音调)
/// TODO: 等 ort 2.0 正式版发布后集成 ONNX Runtime

use anyhow::Result;
use std::path::Path;
use tracing::info;

pub struct TTSEngine {
    sample_rate: u32,
}

impl TTSEngine {
    /// 初始化 TTS 引擎
    pub fn new<P: AsRef<Path>>(_model_path: P) -> Result<Self> {
        info!("🔧 TTS 引擎初始化 (Mock 模式)");
        info!("⚠️  等待 ort 2.0 正式版发布后集成 ONNX Runtime");

        Ok(Self {
            sample_rate: 24000,  // Kokoro 使用 24kHz
        })
    }

    /// 文本转语音
    ///
    /// 当前生成测试音调用于验证音频管道
    pub fn synthesize(&self, text: &str) -> Result<Vec<f32>> {
        info!("🎵 合成文本: \"{}\"", &text[..text.len().min(50)]);

        // 生成 1 秒测试音调 (440Hz A4 音符)
        let duration = 1.0;
        let frequency = 440.0;
        let sample_count = (self.sample_rate as f32 * duration) as usize;

        let mut audio = Vec::with_capacity(sample_count);
        for i in 0..sample_count {
            let t = i as f32 / self.sample_rate as f32;
            let sample = (2.0 * std::f32::consts::PI * frequency * t).sin() * 0.3; // 30% 音量
            audio.push(sample);
        }

        info!("✅ 生成测试音调 ({} 样本, {}Hz)", audio.len(), frequency);
        Ok(audio)
    }

    /// 获取采样率
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}
