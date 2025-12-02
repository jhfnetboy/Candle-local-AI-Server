# 🎵 TTS Server - Quick Start

## macOS Installation

1. Extract this archive
2. Install espeak-ng:
   ```bash
   brew install espeak-ng
   ```
3. Make executable:
   ```bash
   chmod +x tts-server
   ```
4. Start the server:
   ```bash
   ./tts-server
   ```

## Windows Installation

1. Extract this archive
2. Install espeak-ng:
   - Download from: https://github.com/espeak-ng/espeak-ng/releases
   - Or use chocolatey: `choco install espeak-ng`
3. Run:
   ```
   tts-server.exe
   ```

## First Run

On first run, the server will:
- Start on http://localhost:9527
- Auto-download models (~49MB)
- Create cache directory

## Verify Installation

```bash
curl http://localhost:9527/health
```

Should return: `{"success":true,"data":"healthy"}`

## Integration

Install the MyDictionary Chrome extension and it will auto-detect the server!

For detailed documentation, see README.md
