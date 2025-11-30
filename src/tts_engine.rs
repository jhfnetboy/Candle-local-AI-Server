/// TTS Engine - Kokoro-82M ONNX 实现
///
/// 模型: Kokoro-82M (82M 参数 TTS 模型)
/// 运行时: ONNX Runtime 2.0-rc
///
/// 输入:
/// - tokens: i64 数组 [batch, seq_len]  (音素 token IDs)
/// - style: f32 数组 [1, 256]           (说话人风格向量)
/// - speed: f32 数组 [1]                (语速控制)
///
/// 输出:
/// - audio: f32 数组 [batch, audio_len] (24kHz 音频波形)

use anyhow::{Context, Result};
use ort::session::{builder::GraphOptimizationLevel, Session};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct TTSEngine {
    session: Session,
    sample_rate: u32,
    style_vectors: Vec<Vec<f32>>, // [512 tokens, 256 dims]
}

impl TTSEngine {
    /// 初始化 TTS 引擎
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        info!("🔧 TTS 引擎初始化");

        let model_path = model_path.as_ref();
        info!("📂 加载模型: {:?}", model_path);

        // 创建 ONNX Session
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(model_path)
            .with_context(|| format!("无法加载 ONNX 模型: {:?}", model_path))?;

        info!("✅ ONNX 模型加载成功");

        // 打印模型输入/输出信息
        info!("📋 ONNX 模型输入:");
        for input in session.inputs.iter() {
            info!("  - 名称: {}, 类型: {:?}", input.name, input.input_type);
        }

        info!("📋 ONNX 模型输出:");
        for output in session.outputs.iter() {
            info!("  - 名称: {}, 类型: {:?}", output.name, output.output_type);
        }

        // 加载 style vectors (使用简化的二进制格式)
        info!("📂 加载 style vectors...");
        let style_vectors = Self::load_style_vectors("data/voices_simple.bin")?;
        info!("✅ 加载 {} 个 style vectors", style_vectors.len());

        Ok(Self {
            session,
            sample_rate: 24000,
            style_vectors,
        })
    }

    /// 加载 style vectors from binary file
    /// 格式: [510 tokens, 256 dims] f32 数组
    fn load_style_vectors<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<f32>>> {
        let mut file = File::open(path.as_ref())
            .with_context(|| format!("无法打开 style vectors 文件: {:?}", path.as_ref()))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // 解析为 f32 数组
        let floats: Vec<f32> = buffer
            .chunks_exact(4)
            .map(|bytes| f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            .collect();

        // 重组为 [510, 256] 结构 (Kokoro 实际使用 510 个 token styles)
        const TOKEN_LIMIT: usize = 510;
        const STYLE_DIM: usize = 256;

        let mut vectors = Vec::with_capacity(TOKEN_LIMIT);
        for i in 0..TOKEN_LIMIT {
            let start = i * STYLE_DIM;
            let end = start + STYLE_DIM;
            if end <= floats.len() {
                vectors.push(floats[start..end].to_vec());
            }
        }

        Ok(vectors)
    }

    /// 文本转语音 - ONNX 推理
    pub fn synthesize(&mut self, text: &str) -> Result<Vec<f32>> {
        info!("🎵 合成文本: \"{}\"", &text[..text.len().min(50)]);

        // 1. 文本 → 音素 (简化版: 直接使用文本)
        let phonemes = self.simple_phonemize(text);
        info!("📝 音素: {}", &phonemes[..phonemes.len().min(50)]);

        // 2. 音素 → tokens
        let tokens = crate::vocab::tokenize(&phonemes);
        info!("🔢 Tokens: {} 个", tokens.len());

        if tokens.is_empty() {
            return Ok(vec![0.0; 24000]); // 1秒静音
        }

        // 3. 获取 style vector (使用第一个 style,不依赖 token 长度)
        // 参考 Kokoros: 使用固定的 voice style
        let style_vector = if !self.style_vectors.is_empty() {
            self.style_vectors[0].clone()  // 使用第一个 style (默认声音)
        } else {
            vec![0.0f32; 256]  // 降级: 零向量
        };

        info!("🎨 使用 style vector: index=0, dims={}", style_vector.len());

        // 4. ONNX 推理
        let audio = self.run_inference(&tokens, &style_vector)?;

        info!("✅ ONNX 推理完成 ({} 样本)", audio.len());
        Ok(audio)
    }

    /// espeak-ng 音素化
    fn simple_phonemize(&self, text: &str) -> String {
        match self.phonemize_with_espeak(text) {
            Ok(phonemes) => {
                info!("✅ espeak-ng 音素化成功");
                phonemes
            }
            Err(e) => {
                info!("⚠️ espeak-ng 失败: {}, 使用降级方案", e);
                // 降级: 简单处理
                text.chars()
                    .filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace())
                    .collect::<String>()
                    .to_lowercase()
            }
        }
    }

    /// 使用 espeak-ng 进行音素化
    fn phonemize_with_espeak(&self, text: &str) -> Result<String> {
        info!("🔊 调用 espeak-ng: {}", text);
        let output = Command::new("espeak-ng")
            .args(&["-v", "en-us", "-q", "--ipa", text])
            .output()
            .context("espeak-ng 未安装或无法执行")?;

        info!("📋 espeak-ng 返回状态: {}", output.status);

        if !output.status.success() {
            return Err(anyhow::anyhow!("espeak-ng 执行失败"));
        }

        let mut phonemes = String::from_utf8(output.stdout)?
            .trim()
            .to_string();

        // Kokoro-specific 替换
        phonemes = phonemes
            .replace("kəkˈoːɹoʊ", "kˈoʊkəɹoʊ")
            .replace("kəkˈɔːɹəʊ", "kˈəʊkəɹəʊ")
            .replace("ʲ", "j")
            .replace("r", "ɹ")
            .replace("x", "k")
            .replace("ɬ", "l");

        // 过滤词汇表外的字符
        phonemes = phonemes
            .chars()
            .filter(|&c| crate::vocab::VOCAB.contains_key(&c))
            .collect();

        Ok(phonemes)
    }

    /// ONNX 推理 (真实数据)
    fn run_inference(&mut self, tokens: &[i64], style_vector: &[f32]) -> Result<Vec<f32>> {
        use ort::value::Tensor;

        // 添加 padding tokens (0 = pad token '$')
        let mut padded_tokens = vec![0i64]; // 开始 pad
        padded_tokens.extend_from_slice(tokens);
        padded_tokens.push(0); // 结束 pad

        // 创建 tokens tensor [1, seq_len]
        let tokens_2d = vec![padded_tokens.clone()];
        let shape = [tokens_2d.len(), tokens_2d[0].len()];
        let tokens_flat: Vec<i64> = tokens_2d.into_iter().flatten().collect();

        info!("🔢 Token输入: shape={:?}, first_5={:?}", shape, &padded_tokens[..padded_tokens.len().min(5)]);

        let tokens_tensor = Tensor::from_array((shape, tokens_flat))?;

        // 创建 style tensor [1, 256]
        let style_2d = vec![style_vector.to_vec()];
        let shape_style = [style_2d.len(), style_2d[0].len()];
        let style_flat: Vec<f32> = style_2d.into_iter().flatten().collect();
        let style_tensor = Tensor::from_array((shape_style, style_flat))?;

        // speed: 默认速度 1.0
        let speed_tensor = Tensor::from_array(([1], vec![1.0f32]))?;

        info!("🔧 ONNX 输入准备完成");

        // 执行推理 (参考 Kokoros 实现)
        let outputs = self.session.run(ort::inputs![
            "input_ids" => tokens_tensor,  // Kokoro v1.0-timestamped 使用 "input_ids"
            "style" => style_tensor,
            "speed" => speed_tensor,
        ])?;

        info!("✅ ONNX 推理成功");

        // 提取音频输出 (尝试 "waveform" 或 "audio")
        let (shape, data) = outputs["waveform"]
            .try_extract_tensor::<f32>()
            .or_else(|_| outputs["audio"].try_extract_tensor::<f32>())
            .context("无法提取音频输出")?;

        info!("🎵 音频形状: {:?}", shape);

        // 转换为 Vec<f32>
        let mut audio: Vec<f32> = data.to_vec();

        // 归一化音频 (防止削波)
        let max_abs = audio.iter()
            .map(|&x| x.abs())
            .fold(0.0f32, |max, x| max.max(x));

        info!("📊 音频幅度范围: max={:.4}", max_abs);

        if max_abs > 1.0 {
            info!("⚠️ 音频幅度过大,进行归一化");
            let scale = 0.95 / max_abs; // 归一化到 95% 避免削波
            for sample in audio.iter_mut() {
                *sample *= scale;
            }
            info!("✅ 音频已归一化 (缩放: {:.4})", scale);
        } else if max_abs > 0.0 {
            // 即使在范围内,也放大到接近最大值以获得更好的音量
            let scale = 0.95 / max_abs;
            if scale > 1.0 {
                for sample in audio.iter_mut() {
                    *sample *= scale;
                }
                info!("📈 音频增益: {:.4}x", scale);
            }
        }

        Ok(audio)
    }

    /// 获取采样率
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}
