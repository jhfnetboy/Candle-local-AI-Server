# MyDictionary TTS Server

本地 TTS 服务器，支持多模型切换，提供高质量中英文语音合成。

## 功能特性

- ✅ **多模型支持**: SpeechT5 (英文) + CosyVoice (中英文)
- ✅ **API 切换**: RESTful API 动态切换模型
- ✅ **本地运行**: 完全离线，保护隐私
- ✅ **高音质**: 支持 Top1 中文 TTS 模型
- ✅ **跨平台**: 支持 CPU/CUDA 加速

## 快速开始

### 1. 安装依赖

```bash
cd tts-server
pip install -r requirements.txt
```

### 2. 启动服务器

```bash
python server.py
```

服务器将在 `http://localhost:5050` 启动。

### 3. 测试 API

```bash
# 检查服务器状态
curl http://localhost:5050/

# 查看可用模型
curl http://localhost:5050/models

# 加载模型
curl -X POST http://localhost:5050/models/speecht5/load

# 合成语音
curl -X POST http://localhost:5050/synthesize \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello World"}' \
  --output test.wav
```

## API 文档

### GET `/`
获取服务器信息

**响应**:
```json
{
  "name": "MyDictionary TTS Server",
  "version": "1.0.0",
  "status": "running",
  "current_model": "speecht5",
  "available_models": ["speecht5", "cosyvoice"]
}
```

### GET `/models`
获取所有可用模型

**响应**:
```json
{
  "success": true,
  "models": [
    {
      "id": "speecht5",
      "name": "SpeechT5 (English)",
      "language": "en",
      "quality": 6,
      "speed": "fast",
      "loaded": true,
      "current": true
    },
    {
      "id": "cosyvoice",
      "name": "CosyVoice (中英文)",
      "language": "zh-en",
      "quality": 9,
      "speed": "medium",
      "loaded": false,
      "current": false
    }
  ]
}
```

### POST `/models/{model_key}/load`
加载指定模型

**参数**:
- `model_key`: 模型 ID (`speecht5` | `cosyvoice`)

**响应**:
```json
{
  "success": true,
  "message": "模型 speecht5 加载成功",
  "current_model": "speecht5"
}
```

### POST `/synthesize`
合成语音

**请求 Body**:
```json
{
  "text": "要合成的文本",
  "speaker_id": 0,  // 可选: speaker ID
  "format": "wav"   // 可选: 输出格式 (wav | mp3)
}
```

**响应**: 音频文件 (audio/wav 或 audio/mp3)

### GET `/health`
健康检查

**响应**:
```json
{
  "status": "healthy",
  "device": "cpu",
  "models_loaded": 1,
  "current_model": "speecht5"
}
```

## 支持的模型

### SpeechT5 (默认)
- **语言**: 英文
- **质量**: 6/10
- **速度**: 快
- **内存**: ~500MB
- **特点**: 轻量级，适合快速响应

### CosyVoice (开发中)
- **语言**: 中英文
- **质量**: 9/10
- **速度**: 中等
- **内存**: ~2GB
- **特点**: 高质量，自然流畅，中文发音优秀

## 与 MyDictionary 集成

MyDictionary Chrome 扩展会自动检测本地 TTS 服务器：

1. **优先使用本地服务器** (如果可用)
2. **回退到浏览器 TTS** (SpeechT5 ONNX)

### 扩展端配置

在 `popup.html` 设置中：
- 🔘 **自动模式**: 优先本地，自动回退
- 🔘 **仅本地**: 强制使用本地服务器
- 🔘 **仅浏览器**: 仅使用浏览器 TTS

## 性能优化

### GPU 加速
自动检测 CUDA，使用 GPU 加速推理：
```bash
# 检查 CUDA 可用性
python -c "import torch; print(torch.cuda.is_available())"
```

### 内存优化
- 使用 `torch.no_grad()` 减少内存占用
- 支持按需加载模型
- 自动释放未使用模型

## 开发计划

- [x] SpeechT5 基础支持
- [ ] CosyVoice 集成
- [ ] 多 Speaker 支持
- [ ] 音量/语速控制
- [ ] 音频缓存
- [ ] WebSocket 流式传输

## 故障排除

### 服务器启动失败
```bash
# 检查端口是否被占用
lsof -i :5050

# 更换端口
python server.py --port 5051
```

### 模型下载慢
使用 Hugging Face 镜像：
```bash
export HF_ENDPOINT=https://hf-mirror.com
python server.py
```

### CUDA 内存不足
降低 batch size 或使用 CPU：
```bash
# 强制使用 CPU
CUDA_VISIBLE_DEVICES="" python server.py
```

## License

MIT License - 详见 LICENSE 文件
