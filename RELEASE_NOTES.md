# 🎵 TTS Server v0.1.0 Release Notes

**Release Date**: December 1, 2024

## 🎉 First Stable Release

This is the first public release of the TTS Server - a high-performance local text-to-speech service designed for the MyDictionary Chrome extension.

---

## ✨ Key Features

### 🎤 54 Premium Voices
- **British English**: 8 voices (4 male, 4 female)
  - Recommended: `bm_george`, `bm_daniel` for clear pronunciation
- **American English**: 11 voices (6 male, 5 female)
  - Recommended: `af_nova`, `am_michael` for standard accents

### ⚡ Performance
- **Sub-second synthesis** - Powered by Rust and ONNX Runtime
- **Smart caching** - SHA256-based file caching with 1-hour TTL
- **Low memory footprint** - Optimized for desktop use

### 🔄 Auto Model Download
- Models download automatically on first run
- `kokoro-v1.0.onnx` (~24MB)
- `voices-v1.0.bin` (~25MB)
- Total: ~49MB one-time download

### 🌐 REST API
- Simple HTTP endpoints
- Browser-compatible 16-bit PCM WAV output
- CORS enabled for extension integration

---

## 📦 Downloads

### macOS (Apple Silicon & Intel)
- **File**: `tts-server-macos.tar.gz`
- **Size**: ~8MB (compressed)
- **Requirements**: macOS 10.15+, espeak-ng

### Windows (x64)
- **File**: `tts-server-windows.zip`
- **Size**: ~10MB (compressed)
- **Requirements**: Windows 10+, espeak-ng

### Build from Source
- Requires: Rust 1.70+, espeak-ng
- See [README.md](README.md) for instructions

---

## 🔗 Integration

Works seamlessly with **MyDictionary Chrome Extension**:
1. Download and start the TTS server
2. Install MyDictionary extension
3. Extension auto-detects the local server
4. Select your preferred voice in settings
5. Enjoy offline TTS with 54 premium voices!

---

## 📡 API Endpoints

### `GET /health`
Health check endpoint
```bash
curl http://localhost:9527/health
```

### `POST /synthesize`
Text-to-speech synthesis
```bash
curl -X POST http://localhost:9527/synthesize \
  -H "Content-Type: application/json" \
  -d '{
    "text": "Hello, world!",
    "voice": "bm_george"
  }'
```

### `GET /audio/:filename`
Retrieve synthesized audio
```bash
curl http://localhost:9527/audio/{file_id}.wav --output output.wav
```

See [VOICE_API.md](VOICE_API.md) for complete voice list and examples.

---

## 🛠️ Installation

### Quick Start (macOS)
```bash
# Download
curl -L -o tts-server-macos.tar.gz \
  https://github.com/jhfnetboy/Candle-local-AI-Server/releases/download/v0.1.0/tts-server-macos.tar.gz

# Extract
tar -xzf tts-server-macos.tar.gz
cd tts-server

# Install espeak-ng
brew install espeak-ng

# Run
chmod +x tts-server
./tts-server
```

### Quick Start (Windows)
```powershell
# 1. Download tts-server-windows.zip from release page
# 2. Extract to a folder (e.g., C:\tts-server)
# 3. Install espeak-ng:
choco install espeak-ng

# 4. Run
.\tts-server.exe
```

---

## 🐛 Known Issues

- espeak-ng must be installed separately (not bundled)
- Windows Defender may show security warning on first run (click "Allow access")
- Model download requires internet connection on first run

---

## 🔜 Future Plans

### v0.2.0 (Planned)
- [ ] Additional output formats (mp3, ogg)
- [ ] Voice speed/pitch control
- [ ] Batch synthesis API
- [ ] Windows installer with espeak-ng bundled

### v0.3.0 (Planned)
- [ ] More languages (French, Japanese, Chinese)
- [ ] Custom voice training support
- [ ] WebSocket streaming API

---

## 🙏 Credits

- **Kokoro-82M**: https://github.com/lucasjinreal/Kokoros
- **ONNX Runtime**: https://onnxruntime.ai/
- **espeak-ng**: https://github.com/espeak-ng/espeak-ng

---

## 📝 Changelog

### [0.1.0] - 2024-12-01

#### Added
- Initial release with 54 voices
- REST API server (Axum framework)
- ONNX Runtime integration
- Smart file caching system
- Auto model download
- Health check endpoint
- Complete API documentation

#### Technical
- Rust 1.70+ support
- macOS & Windows builds
- 16-bit PCM WAV output
- SHA256-based cache keys
- CORS enabled

---

## 📞 Support

- **Bug Reports**: [GitHub Issues](https://github.com/jhfnetboy/Candle-local-AI-Server/issues)
- **Discussions**: [GitHub Discussions](https://github.com/jhfnetboy/Candle-local-AI-Server/discussions)
- **Extension Issues**: [MyDictionary](https://github.com/jhfnetboy/MyDictionary/issues)

---

**Made with ❤️ by Jason**

License: MIT
