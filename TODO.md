# TTS Server - TODO List

## 🚧 当前状态

**版本**: 0.1.0
**状态**: 基础架构完成，等待 ONNX Runtime 2.0 正式版

---

## ✅ 已完成

- [x] HTTP 服务器框架 (Axum + Tokio, 端口 9527)
- [x] 健康检查端点 (GET /health)
- [x] TTS 合成端点 (POST /synthesize)
- [x] WAV 音频编码 (24kHz, 32-bit float)
- [x] Mock TTS 实现 (生成测试音调)
- [x] CORS 配置
- [x] 下载脚本 (download_models.sh)
- [x] 自动发现机制 (扩展端自动检测服务器)

---

## 🔥 高优先级 (等待依赖)

### 1. ONNX Runtime 集成 ⏳

**阻塞原因**: `ort` crate 1.x 版本全部被 yanked，2.0 版本仍在 RC 阶段

**当前状态**:
```toml
# Cargo.toml
# ort = "2.0.0-rc.10"  # API 不稳定，暂时禁用
```

**待办事项**:
- [ ] 监控 `ort 2.0.0` 正式版发布
- [ ] 升级到正式版
- [ ] 实现 ONNX 模型加载
- [ ] 实现音频推理
- [ ] 测试 Kokoro-82M 模型

**预计时间**: 等待上游发布 (1-2 周？)

---

## 📝 中优先级 (可立即实现)

### 2. 文件缓存系统 🎯

**目标**: 避免重复生成相同文本的音频

**实现计划**:
```rust
// src/cache.rs
pub struct AudioCache {
    cache_dir: PathBuf,
    index: HashMap<String, CacheEntry>,
}

impl AudioCache {
    // 基于文本哈希生成文件名
    pub fn get_file_id(&self, text: &str) -> String {
        // SHA256(text)[..16]
    }

    // 检查缓存是否存在
    pub fn exists(&self, file_id: &str) -> bool

    // 保存音频到缓存
    pub fn save(&mut self, text: &str, audio: &[f32]) -> Result<String>

    // 定期清理过期文件
    pub fn cleanup(&mut self)
}
```

**待办事项**:
- [ ] 创建 `src/cache.rs` 模块
- [ ] 实现文本哈希 (SHA256)
- [ ] 实现文件保存逻辑
- [ ] 添加缓存过期机制 (1小时)
- [ ] 集成到 `/synthesize` 端点

**预计时间**: 1-2 小时

---

### 3. HTTP 文件服务端点 🎯

**目标**: 通过 HTTP 提供缓存的音频文件

**实现计划**:
```rust
// src/main.rs

// 新增端点
Router::new()
    .route("/audio/:filename", get(serve_audio))

async fn serve_audio(Path(filename): Path<String>) -> impl IntoResponse {
    let path = format!("cache/{}", filename);
    let bytes = tokio::fs::read(&path).await?;

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "audio/wav")],
        bytes
    )
}
```

**API 变更**:
```javascript
// 当前: 返回二进制
POST /synthesize → 返回 WAV 二进制 (100KB)

// 新版: 返回 URL
POST /synthesize → 返回 { "url": "http://localhost:9527/audio/abc123.wav" } (50B)
```

**待办事项**:
- [ ] 添加 `/audio/:filename` 端点
- [ ] 修改 `/synthesize` 返回格式
- [ ] 添加文件 MIME 类型检测
- [ ] 错误处理 (404 文件不存在)

**预计时间**: 30 分钟

---

### 4. 更新 Plugin 使用 URL 模式 🎯

**目标**: 扩展通过 URL 播放音频，而不是传输二进制数据

**实现计划**:
```javascript
// src/lib/tts-manager.js
async speakViaLocalServer(text) {
    const response = await fetch(`${this.localServerUrl}/synthesize`, {
        method: 'POST',
        body: JSON.stringify({ text })
    });

    const { url } = await response.json();

    // 通过 Offscreen Document 播放 URL
    chrome.runtime.sendMessage({
        action: 'playAudioFromUrl',
        audioUrl: url
    });
}

// src/offscreen/audio-player.js
chrome.runtime.onMessage.addListener((message) => {
    if (message.action === 'playAudioFromUrl') {
        const audio = new Audio(message.audioUrl);
        audio.play();
    }
});
```

**待办事项**:
- [ ] 修改 `TTSManager.speakViaLocalServer()`
- [ ] 添加 `playAudioFromUrl` 消息处理
- [ ] 更新 Offscreen Document 播放逻辑
- [ ] 测试端到端流程

**预计时间**: 30 分钟

---

## 🌟 低优先级 (未来增强)

### 5. 音素化 (Phonemizer)

**目标**: 文本 → 音素 → Token IDs

**选项**:
- **Option A**: 使用 espeak-ng (需要系统依赖)
- **Option B**: 纯 Rust 实现
- **Option C**: 调用 Python phonemizer (跨语言)

**待办事项**:
- [ ] 研究最佳方案
- [ ] 实现音素转换
- [ ] 集成到 TTS 引擎

**预计时间**: 4-6 小时

---

### 6. 说话人选择

**目标**: 支持多个语音风格

**Kokoro 支持的说话人**:
- `af_heart`: 女声
- `af_sarah`: 女声
- `af_nicole`: 女声
- `af_sky`: 女声
- 等等...

**待办事项**:
- [ ] 加载 `voices-v1.0.bin`
- [ ] 添加 API 参数: `{ "text": "...", "voice": "af_heart" }`
- [ ] 实现说话人 embedding 注入

**预计时间**: 2-3 小时

---

### 7. 性能优化

**待办事项**:
- [ ] 音频流式传输 (边生成边播放)
- [ ] 模型预热 (首次推理加速)
- [ ] GPU 加速 (CUDA/Metal)
- [ ] 批量处理
- [ ] 压缩音频 (MP3/Opus)

**预计时间**: 各 2-4 小时

---

## 📊 技术债务

### 1. 测试覆盖

**当前**: 仅 WAV 编码器有单元测试

**待办事项**:
- [ ] TTS 引擎单元测试
- [ ] Cache 模块单元测试
- [ ] HTTP 端点集成测试
- [ ] 端到端测试

---

### 2. 文档

**待办事项**:
- [ ] API 文档 (OpenAPI/Swagger)
- [ ] 架构图
- [ ] 性能基准测试报告

---

### 3. 错误处理

**待办事项**:
- [ ] 统一错误类型
- [ ] 更友好的错误消息
- [ ] 错误日志结构化

---

## 🎯 里程碑

### v0.2.0 - 文件缓存 (预计 1 天)

- [x] 基础 HTTP 服务器
- [ ] 文件缓存系统
- [ ] HTTP 文件服务
- [ ] Plugin URL 模式

### v0.3.0 - ONNX 集成 (等待上游)

- [ ] ort 2.0 正式版发布
- [ ] Kokoro-82M ONNX 推理
- [ ] 真实音频生成

### v0.4.0 - 完整功能 (预计 1 周)

- [ ] 音素化
- [ ] 说话人选择
- [ ] 性能优化

---

## 📞 如何贡献

1. Fork 仓库
2. 创建功能分支: `git checkout -b feature/xxx`
3. 提交更改: `git commit -m 'feat: xxx'`
4. 推送分支: `git push origin feature/xxx`
5. 提交 Pull Request

---

**最后更新**: 2025-11-30
**维护者**: Jason
