# GitHub Release 创建指南

## 📋 准备工作检查清单

- [x] README.md 已完成
- [x] RELEASE_NOTES.md 已创建
- [x] 版本号已更新到 0.1.0
- [x] Release binary 已构建
- [x] Git tag v0.1.0 已创建并推送
- [x] 测试服务器正常运行

## 🚀 创建GitHub Release步骤

### 1. 访问GitHub Release页面

打开: https://github.com/jhfnetboy/Candle-local-AI-Server/releases/new

### 2. 填写Release信息

**Tag version**: `v0.1.0` (已自动选择)

**Release title**: `🎵 TTS Server v0.1.0 - First Stable Release`

**Description**:

将以下内容复制到描述框:

```markdown
# 🎵 TTS Server v0.1.0 - First Stable Release

**High-performance local TTS server with 54 premium voices**

## ✨ Key Features

- 🎤 **54 Premium Voices** - British/American English, male/female options
- ⚡ **Lightning Fast** - Rust-powered, sub-second synthesis
- 💾 **Smart Caching** - SHA256-based file caching with 1-hour TTL
- 🔄 **Auto Download** - Models download automatically on first run
- 🌐 **REST API** - Simple HTTP endpoints for easy integration
- 🎯 **Browser Compatible** - 16-bit PCM WAV output

## 📦 Downloads

### macOS (Apple Silicon & Intel)
- Download: `tts-server-macos-v0.1.0.tar.gz` (34MB)
- Requirements: macOS 10.15+, espeak-ng
- Quick start:
  ```bash
  tar -xzf tts-server-macos-v0.1.0.tar.gz
  cd tts-server
  brew install espeak-ng
  ./tts-server
  ```

### Windows (Coming Soon)
- Windows x64 build will be available in next release
- In the meantime, please build from source

## 🔗 Integration with MyDictionary

Works seamlessly with the **MyDictionary Chrome Extension**:

1. **Download** the TTS server (see above)
2. **Start** the server: `./tts-server`
3. **Install** MyDictionary extension
4. **Open** extension settings → TTS Voice Settings
5. You'll see a **green "✅ Connected"** indicator!

The extension will automatically detect the local server and enable 54 premium voices for offline TTS.

## 📖 Documentation

- **Installation Guide**: See [README.md](https://github.com/jhfnetboy/Candle-local-AI-Server/blob/main/README.md)
- **API Reference**: See [VOICE_API.md](https://github.com/jhfnetboy/Candle-local-AI-Server/blob/main/VOICE_API.md)
- **Release Notes**: See [RELEASE_NOTES.md](https://github.com/jhfnetboy/Candle-local-AI-Server/blob/main/RELEASE_NOTES.md)

## 🎤 Voice Recommendations

**For English Learning:**
- `bm_george` - British male, clear and standard ⭐
- `bm_daniel` - British male, accurate pronunciation ⭐
- `af_nova` - American female, recommended
- `am_michael` - American male, standard

See [VOICE_API.md](https://github.com/jhfnetboy/Candle-local-AI-Server/blob/main/VOICE_API.md) for complete list of 54 voices.

## 🐛 Known Issues

- espeak-ng must be installed separately (not bundled)
- Windows build not available in this release (build from source)
- Model download requires internet connection on first run

## 📝 Changelog

### Added
- Initial release with 54 voices
- REST API server (Axum framework)
- ONNX Runtime integration
- Smart file caching system
- Auto model download
- Health check endpoint

### Technical
- Rust 1.70+ support
- macOS build (Apple Silicon & Intel)
- 16-bit PCM WAV output
- SHA256-based cache keys
- CORS enabled

## 🔜 Next Release (v0.2.0)

Planned features:
- Windows installer with bundled espeak-ng
- Additional output formats (mp3, ogg)
- Voice speed/pitch control
- Batch synthesis API

## 🙏 Credits

- [Kokoro-82M](https://github.com/lucasjinreal/Kokoros) - High-quality TTS model
- [ONNX Runtime](https://onnxruntime.ai/) - ML inference engine
- [espeak-ng](https://github.com/espeak-ng/espeak-ng) - Phonemization

---

**Made with ❤️ by Jason** | License: MIT
```

### 3. 上传Assets

点击 "Attach binaries by dropping them here or selecting them"

上传文件:
```
release-v0.1.0/tts-server-macos-v0.1.0.tar.gz
```

### 4. 发布选项

- [x] **Set as the latest release** (勾选)
- [ ] **Set as a pre-release** (不勾选)

### 5. 点击 "Publish release"

## ✅ 发布后验证

1. 访问: https://github.com/jhfnetboy/Candle-local-AI-Server/releases/latest
2. 确认 v0.1.0 显示为 "Latest"
3. 确认下载链接正常工作
4. 测试下载并运行release包

## 📢 发布后宣传

### 在MyDictionary README中更新

在主项目README中添加TTS Server链接:

```markdown
## 🎵 Offline TTS with 54 Voices

Download the local TTS server for offline text-to-speech:
👉 [TTS Server v0.1.0](https://github.com/jhfnetboy/Candle-local-AI-Server/releases/latest)

Features:
- 54 premium voices (British/American English)
- Auto model download
- Smart caching
- Sub-second synthesis
```

### 社交媒体

可以在以下平台分享:
- GitHub Discussions
- Reddit (r/rust, r/chrome_extensions)
- Twitter/X
- Product Hunt (可选)

---

**注意**:
- Windows版本需要在Windows环境下交叉编译,暂时建议用户从源码构建
- 未来版本计划提供Windows安装程序,包含espeak-ng
