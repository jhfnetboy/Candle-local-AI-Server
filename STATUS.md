# TTS Server - 当前状态

**日期**: 2025-11-30
**版本**: 0.1.0

## ✅ 已完成

- [x] HTTP 服务器 (Axum, 端口 9527)
- [x] API 端点 (/, /health, /synthesize)
- [x] WAV 编码器 (24kHz, 32-bit float)
- [x] 测试音调生成 (440Hz)
- [x] 自动发现机制
- [x] ort 2.0-rc 依赖添加
- [x] **文件缓存系统** (SHA256 哈希, 1 小时 TTL)
- [x] **静态文件服务** (`GET /audio/:filename`)
- [x] **URL 模式响应** (返回音频 URL 而非二进制)

## ⏳ 进行中

无

## ❌ 阻塞

### ONNX Runtime 集成

**问题**:
1. ort 1.x 全部被 yanked
2. ort 2.0-rc API 变化大，示例不完整
3. 需要更好的文档或工作示例

**解决方案**:
- 等待 ort 2.0 正式版
- 或找到社区完整示例
- 暂时使用测试音调验证架构

## 📝 模型格式总结

| 格式 | 可用性 | 大小 | Rust 支持 |
|------|--------|------|----------|
| PyTorch (.pth) | ✅ | 327 MB | ❌ (需 Python) |
| ONNX (fp32) | ✅ | 1.3 GB | ✅ (ort crate) |
| ONNX (量化) | ✅ | 更小 | ✅ |
| Safetensors | ❌ | - | - |

**建议**: 使用 ONNX格式 + ort 2.0 正式版

## 🎯 下一步

1. **ONNX 推理集成** - 等待 ort 2.0 正式版或更好的示例
2. **音素化** - 文本预处理提升质量
3. **说话人选择** - 多语音支持

## 📊 API 使用示例

### 合成音频
```bash
curl -X POST http://localhost:9527/synthesize \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello, world"}'

# 响应:
{
  "file_id": "6e3b6f3978e5cd96",
  "url": "http://localhost:9527/audio/6e3b6f3978e5cd96.wav",
  "cached": false  # 首次生成
}
```

### 获取音频文件
```bash
curl http://localhost:9527/audio/6e3b6f3978e5cd96.wav --output audio.wav
```

### 缓存命中
```bash
# 重复相同文本
curl -X POST http://localhost:9527/synthesize \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello, world"}'

# 响应:
{
  "file_id": "6e3b6f3978e5cd96",
  "url": "http://localhost:9527/audio/6e3b6f3978e5cd96.wav",
  "cached": true  # 缓存命中，立即返回
}
```

---
**维护者**: Jason
