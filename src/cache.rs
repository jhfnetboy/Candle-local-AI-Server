/// 音频缓存模块
///
/// 功能:
/// - SHA256 文本哈希 → 文件 ID
/// - 缓存 WAV 文件到磁盘
/// - 自动过期清理 (1 小时 TTL)
/// - 线程安全访问

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

/// 音频缓存管理器
pub struct AudioCache {
    cache_dir: PathBuf,
    ttl_seconds: u64,
}

impl AudioCache {
    /// 创建新的缓存实例
    ///
    /// # Arguments
    /// * `cache_dir` - 缓存目录路径
    /// * `ttl_seconds` - 缓存过期时间 (秒)
    pub fn new<P: AsRef<Path>>(cache_dir: P, ttl_seconds: u64) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();

        // 确保缓存目录存在
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .with_context(|| format!("无法创建缓存目录: {:?}", cache_dir))?;
            info!("📁 创建缓存目录: {:?}", cache_dir);
        }

        Ok(Self {
            cache_dir,
            ttl_seconds,
        })
    }

    /// 根据文本和声音生成唯一的文件 ID (SHA256 哈希)
    ///
    /// # Arguments
    /// * `text` - 要合成的文本
    /// * `voice` - 可选的声音名称
    ///
    /// # Returns
    /// 16 字符的十六进制哈希 (SHA256 前 64 位)
    pub fn get_file_id(&self, text: &str, voice: Option<&str>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());

        // 如果有声音参数,也包含在哈希中
        if let Some(v) = voice {
            hasher.update(b"|voice:");
            hasher.update(v.as_bytes());
        }

        let result = hasher.finalize();

        // 取前 8 字节 (64 位) 转为 16 字符十六进制
        format!("{:x}", &result[..8].iter().fold(0u64, |acc, &b| (acc << 8) | b as u64))
    }

    /// 获取缓存文件的完整路径
    ///
    /// # Arguments
    /// * `file_id` - 文件 ID
    fn get_file_path(&self, file_id: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.wav", file_id))
    }

    /// 检查缓存是否存在且未过期
    ///
    /// # Arguments
    /// * `text` - 要检查的文本
    /// * `voice` - 可选的声音名称
    ///
    /// # Returns
    /// `Some(file_id)` 如果缓存命中, `None` 如果未命中或已过期
    pub fn exists(&self, text: &str, voice: Option<&str>) -> Option<String> {
        let file_id = self.get_file_id(text, voice);
        let file_path = self.get_file_path(&file_id);

        if !file_path.exists() {
            debug!("❌ 缓存未命中: {} (文件不存在)", file_id);
            return None;
        }

        // 检查文件修改时间
        match fs::metadata(&file_path) {
            Ok(metadata) => {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = SystemTime::now().duration_since(modified) {
                        if duration.as_secs() < self.ttl_seconds {
                            debug!("✅ 缓存命中: {} ({}秒前)", file_id, duration.as_secs());
                            return Some(file_id);
                        } else {
                            debug!("⏰ 缓存过期: {} ({}秒前)", file_id, duration.as_secs());
                            // 删除过期文件
                            let _ = fs::remove_file(&file_path);
                        }
                    }
                }
            }
            Err(e) => {
                warn!("⚠️  无法读取缓存文件元数据: {}", e);
            }
        }

        None
    }

    /// 保存音频数据到缓存
    ///
    /// # Arguments
    /// * `text` - 原始文本
    /// * `voice` - 可选的声音名称
    /// * `audio_data` - WAV 音频数据 (字节)
    ///
    /// # Returns
    /// 文件 ID
    pub fn save(&self, text: &str, voice: Option<&str>, audio_data: &[u8]) -> Result<String> {
        let file_id = self.get_file_id(text, voice);
        let file_path = self.get_file_path(&file_id);

        let mut file = File::create(&file_path)
            .with_context(|| format!("无法创建缓存文件: {:?}", file_path))?;

        file.write_all(audio_data)
            .with_context(|| "写入缓存文件失败")?;

        info!("💾 保存到缓存: {} ({} 字节)", file_id, audio_data.len());
        Ok(file_id)
    }

    /// 清理所有过期的缓存文件
    ///
    /// # Returns
    /// 删除的文件数量
    pub fn cleanup(&self) -> Result<usize> {
        let mut deleted = 0;
        let now = SystemTime::now();

        let entries = fs::read_dir(&self.cache_dir)
            .with_context(|| format!("无法读取缓存目录: {:?}", self.cache_dir))?;

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // 只处理 .wav 文件
                if path.extension().and_then(|s| s.to_str()) != Some("wav") {
                    continue;
                }

                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(duration) = now.duration_since(modified) {
                            if duration.as_secs() >= self.ttl_seconds {
                                match fs::remove_file(&path) {
                                    Ok(_) => {
                                        debug!("🗑️  删除过期缓存: {:?}", path.file_name());
                                        deleted += 1;
                                    }
                                    Err(e) => {
                                        warn!("⚠️  删除缓存文件失败 {:?}: {}", path, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if deleted > 0 {
            info!("🧹 清理完成: 删除 {} 个过期缓存文件", deleted);
        }

        Ok(deleted)
    }

    /// 获取缓存目录路径
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_file_id_generation() {
        let cache = AudioCache::new("/tmp/test_cache", 3600).unwrap();

        let id1 = cache.get_file_id("Hello, world!");
        let id2 = cache.get_file_id("Hello, world!");
        let id3 = cache.get_file_id("Different text");

        // 相同文本生成相同 ID
        assert_eq!(id1, id2);

        // 不同文本生成不同 ID
        assert_ne!(id1, id3);

        // ID 长度为 16 字符 (64 位十六进制)
        assert_eq!(id1.len(), 16);
    }

    #[test]
    fn test_cache_save_and_exists() {
        let cache = AudioCache::new("/tmp/test_cache", 3600).unwrap();

        let text = "Test audio content";
        let audio_data = vec![0u8; 1024]; // 模拟音频数据

        // 保存到缓存
        let file_id = cache.save(text, &audio_data).unwrap();

        // 检查缓存存在
        assert_eq!(cache.exists(text), Some(file_id));
    }

    #[test]
    fn test_cache_expiration() {
        let cache = AudioCache::new("/tmp/test_cache_ttl", 1).unwrap(); // 1 秒过期

        let text = "Expiring content";
        let audio_data = vec![0u8; 512];

        cache.save(text, &audio_data).unwrap();

        // 立即检查 - 应该存在
        assert!(cache.exists(text).is_some());

        // 等待 2 秒
        thread::sleep(Duration::from_secs(2));

        // 检查 - 应该已过期
        assert!(cache.exists(text).is_none());
    }
}
