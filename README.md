# TTS Server - 本地语音合成服务器

基于 Kokoro-82M ONNX 模型的高性能 TTS 服务器。

## 快速开始

### 1. 安装依赖

```bash
brew install espeak-ng  # macOS
```

### 2. 启动服务器

```bash
./start.sh
```

### 3. 测试 API

```bash
curl -X POST http://localhost:9527/synthesize \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello world"}'
```

## 与 Chrome 扩展集成

扩展会自动检测并使用本地 TTS 服务器 (每 30 秒检查一次)。

**启动流程**:

1. 终端 1: `cd tts-server && ./start.sh` (启动 TTS 服务器)
2. 终端 2: `cd .. && pnpm build` (构建扩展)
3. Chrome: 加载 `dist/` 目录
4. 选中文本,点击 TTS 按钮即可

## 技术栈

- Rust + Axum (HTTP 服务器)
- ort 2.0-rc (ONNX Runtime)
- espeak-ng (音素化)
- Kokoro-82M (TTS 模型)

更多文档: https://github.com/lucasjinreal/Kokoros
