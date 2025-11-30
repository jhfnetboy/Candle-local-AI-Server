/// WAV Audio Encoder
///
/// 将 f32 音频样本编码为 WAV 格式

use anyhow::Result;
use hound::{WavWriter, WavSpec, SampleFormat};
use std::io::Cursor;

/// 编码 WAV 音频到内存
/// 使用 16-bit PCM 格式 (浏览器兼容)
pub fn encode_wav(audio: &[f32], sample_rate: u32) -> Result<Vec<u8>> {
    // WAV 规格 - 使用 16-bit PCM (浏览器标准格式)
    let spec = WavSpec {
        channels: 1,                    // 单声道
        sample_rate,                    // 24000 Hz
        bits_per_sample: 16,            // 16-bit PCM
        sample_format: SampleFormat::Int,
    };

    // 写入内存缓冲区
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = WavWriter::new(&mut cursor, spec)?;

        // 写入所有样本 (f32 → i16 转换)
        for &sample in audio {
            // 限制范围到 [-1.0, 1.0]
            let clamped = sample.max(-1.0).min(1.0);
            // 转换为 16-bit PCM: [-1.0, 1.0] → [-32768, 32767]
            let pcm_sample = (clamped * 32767.0) as i16;
            writer.write_sample(pcm_sample)?;
        }

        writer.finalize()?;
    }

    Ok(cursor.into_inner())
}

/// 编码 WAV 音频到文件
pub fn encode_wav_file(audio: &[f32], sample_rate: u32, path: &str) -> Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create(path, spec)?;

    for &sample in audio {
        writer.write_sample(sample)?;
    }

    writer.finalize()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_silence() {
        let sample_rate = 24000;
        let duration = 1.0; // 1 秒
        let sample_count = (sample_rate as f32 * duration) as usize;

        // 生成静音
        let silence = vec![0.0_f32; sample_count];

        // 编码
        let wav_bytes = encode_wav(&silence, sample_rate).unwrap();

        // 验证 WAV 文件头 (至少 44 字节)
        assert!(wav_bytes.len() > 44);

        // 验证 RIFF 头
        assert_eq!(&wav_bytes[0..4], b"RIFF");
        assert_eq!(&wav_bytes[8..12], b"WAVE");
    }

    #[test]
    fn test_encode_tone() {
        let sample_rate = 24000;
        let duration = 0.1; // 100ms
        let frequency = 440.0; // A4 音符

        // 生成正弦波
        let sample_count = (sample_rate as f32 * duration) as usize;
        let mut tone = Vec::with_capacity(sample_count);

        for i in 0..sample_count {
            let t = i as f32 / sample_rate as f32;
            let sample = (2.0 * std::f32::consts::PI * frequency * t).sin();
            tone.push(sample);
        }

        // 编码
        let wav_bytes = encode_wav(&tone, sample_rate).unwrap();

        // 验证非空
        assert!(wav_bytes.len() > 44);
    }
}
