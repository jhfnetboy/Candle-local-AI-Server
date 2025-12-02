# 🎵 TTS Server - Local Text-to-Speech Service

**Version 0.1.0** | High-performance local TTS server powered by Kokoro-82M ONNX model

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## 📖 Overview

A lightweight, blazing-fast text-to-speech server designed for the MyDictionary Chrome extension. Features 54 high-quality voices with automatic model downloading and intelligent caching.

### ✨ Features

- 🎤 **54 Premium Voices** - British/American English, male/female options
- ⚡ **Lightning Fast** - Rust-powered, sub-second synthesis
- 💾 **Smart Caching** - SHA256-based file caching with TTL
- 🔄 **Auto Download** - Models download automatically on first run
- 🌐 **REST API** - Simple HTTP endpoints for easy integration
- 🎯 **Browser Compatible** - 16-bit PCM WAV output

---

## 🚀 Quick Start

### Option 1: Download Pre-built Binary (Recommended)

#### **macOS (Apple Silicon & Intel)**

**⚠️ 重要: 必须在命令行中运行，不要双击 tts-server 文件！**

```bash
# 1. Download the latest release
curl -L -o tts-server-macos.tar.gz \
  https://github.com/jhfnetboy/Candle-local-AI-Server/releases/download/v0.1.0/tts-server-macos-v0.1.0.tar.gz

# 2. Extract
tar -xzf tts-server-macos-v0.1.0.tar.gz
cd tts-server

# 3. Install espeak-ng (required for phonemization)
brew install espeak-ng

# 4. Remove macOS quarantine attribute and make executable
xattr -d com.apple.quarantine tts-server
chmod +x tts-server

# 5. Start the server (在终端中运行)
./tts-server
```

**常见问题解决**:
- 如果遇到 "cannot be opened because it is from an unidentified developer"
  - 运行: `xattr -d com.apple.quarantine tts-server`
- 如果遇到 "espeak-ng: command not found"
  - 安装: `brew install espeak-ng`

The server will:
- Start on `http://localhost:9527`
- Download models automatically (~49MB, one-time download)
- Create cache directory for audio files

#### **Windows (x64)**

**⚠️ Windows 版本将在 v0.2.0 发布 (预计 12月中旬)**

v0.1.0 目前仅支持 macOS。Windows 用户可以选择:

**选项 1: 等待 v0.2.0 (推荐)**
- 将包含 Windows 安装程序 (.exe)
- 自动安装所有依赖
- 一键启动

**选项 2: 从源码构建 (见下方 "Option 2: Build from Source")**

---

### Option 2: Build from Source

**Prerequisites:**
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- espeak-ng

```bash
# Clone the repository
git clone https://github.com/jhfnetboy/Candle-local-AI-Server.git
cd Candle-local-AI-Server

# Install espeak-ng
# macOS:
brew install espeak-ng
# Ubuntu:
sudo apt-get install espeak-ng
# Windows:
choco install espeak-ng

# Build release version
cargo build --release

# Run
./target/release/tts-server
```

---

## 🔗 Integration with MyDictionary Extension

### Step 1: Start TTS Server

```bash
# Make sure the server is running
./tts-server

# You should see:
# 🚀 启动 TTS 服务器 (Candle Framework)...
# 🎯 服务器监听地址: 0.0.0.0:9527
# 📡 健康检查: http://localhost:9527/health
```

### Step 2: Install MyDictionary Extension

1. Download MyDictionary extension from [Chrome Web Store](#) or build from source
2. The extension will **automatically detect** the local TTS server
3. Open extension settings → **TTS Voice Settings**
4. You'll see a green "✅ Connected" indicator if the server is running

### Step 3: Select Your Voice

1. Go to **TTS Voice Settings** (Extension popup → Settings → Voice Settings)
2. Choose from 54 voices:
   - 🇬🇧 **British English**: George, Daniel, Alice, Emma... (Recommended for learning)
   - 🇺🇸 **American English**: Michael, Nova, Sarah...
3. Click **Save Settings**

### Step 4: Enjoy!

Select any text on a webpage and click the 🔊 TTS button in the sidebar.

---

## 📡 API Reference

### Endpoints

#### `GET /` - Server Info
```bash
curl http://localhost:9527/
```

Response:
```json
{
  "success": true,
  "data": {
    "name": "TTS Server",
    "version": "0.1.0",
    "status": "running",
    "framework": "Candle"
  }
}
```

#### `GET /health` - Health Check
```bash
curl http://localhost:9527/health
```

#### `POST /synthesize` - Text to Speech

**Request:**
```bash
curl -X POST http://localhost:9527/synthesize \
  -H "Content-Type: application/json" \
  -d '{
    "text": "Hello, world!",
    "voice": "bm_george",
    "format": "wav"
  }'
```

**Parameters:**
- `text` (required): Text to synthesize
- `voice` (optional): Voice ID (default: `bm_george`)
- `format` (optional): Output format, currently only `wav` (reserved for future mp3/ogg support)

**Response:**
```json
{
  "file_id": "51f91581302698db",
  "url": "http://localhost:9527/audio/51f91581302698db.wav",
  "cached": false
}
```

#### `GET /audio/:filename` - Get Audio File

```bash
curl http://localhost:9527/audio/51f91581302698db.wav --output output.wav
```

### Voice List

See [VOICE_API.md](VOICE_API.md) for complete list of 54 available voices.

**Recommended voices for English learning:**
- `bm_george` - British male, clear and standard
- `bm_daniel` - British male, accurate pronunciation
- `af_nova` - American female, recommended
- `am_michael` - American male, standard

---

## 🛠️ Configuration

### Port Configuration

By default, the server runs on port `9527`. To change:

Edit `src/main.rs`:
```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 9527));  // Change port here
```

Then rebuild:
```bash
cargo build --release
```

### Cache Configuration

- **Location**: `cache/audio/`
- **TTL**: 1 hour (3600 seconds)
- **Format**: SHA256-based file IDs

To change cache settings, edit `src/main.rs`:
```rust
AudioCache::new("cache/audio", 3600)  // Change TTL (seconds)
```

---

## 🐛 Troubleshooting

### Problem: Server won't start

**Solution 1: Check if port 9527 is already in use**
```bash
# macOS/Linux:
lsof -i :9527

# Windows:
netstat -ano | findstr :9527
```

**Solution 2: Check espeak-ng installation**
```bash
espeak-ng --version
```

If not installed, see [Quick Start](#-quick-start) for installation instructions.

### Problem: Extension shows "Disconnected"

1. Make sure the TTS server is running: `http://localhost:9527/health`
2. Check browser console for CORS errors
3. Restart the server and reload the extension

### Problem: "Model not found" error

The models should download automatically on first run. If they don't:

```bash
# Download manually
./download_models.sh
```

This will download:
- `kokoro-v1.0.onnx` (~24MB)
- `voices-v1.0.bin` (~25MB)

### Problem: Windows - "espeak-ng not found"

1. Download espeak-ng from: https://github.com/espeak-ng/espeak-ng/releases
2. Install and add to PATH
3. Restart your terminal/PowerShell
4. Verify: `espeak-ng --version`

---

## 🏗️ Project Structure

```
tts-server/
├── src/
│   ├── main.rs           # HTTP server & routes
│   ├── tts_engine.rs     # Kokoro ONNX inference
│   ├── cache.rs          # File caching system
│   ├── vocab.rs          # Tokenization
│   └── wav_encoder.rs    # WAV audio encoding
├── checkpoints/          # ONNX models (auto-downloaded)
├── data/voices/          # 54 voice embeddings
├── cache/audio/          # Cached audio files
├── Cargo.toml            # Rust dependencies
└── README.md             # This file
```

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Kokoro-82M](https://github.com/lucasjinreal/Kokoros) - High-quality TTS model
- [ONNX Runtime](https://onnxruntime.ai/) - ML inference engine
- [espeak-ng](https://github.com/espeak-ng/espeak-ng) - Phonemization

---

## 📞 Support

- **GitHub Issues**: [Report a bug](https://github.com/jhfnetboy/Candle-local-AI-Server/issues)
- **Discussions**: [Ask a question](https://github.com/jhfnetboy/Candle-local-AI-Server/discussions)
- **Extension Issues**: [MyDictionary](https://github.com/jhfnetboy/MyDictionary/issues)

---

**Made with ❤️ by Jason**
