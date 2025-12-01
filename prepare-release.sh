#!/bin/bash
# TTS Server v0.1.0 Release Preparation Script

set -e

VERSION="0.1.0"
BINARY_NAME="tts-server"
RELEASE_DIR="release-v${VERSION}"

echo "🚀 Preparing TTS Server v${VERSION} Release..."

# Clean previous release
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# Build release binary
echo "📦 Building release binary..."
cargo build --release

# Create release package structure
echo "📁 Creating package structure..."
mkdir -p "$RELEASE_DIR/tts-server"
mkdir -p "$RELEASE_DIR/tts-server/checkpoints"
mkdir -p "$RELEASE_DIR/tts-server/data/voices"
mkdir -p "$RELEASE_DIR/tts-server/cache/audio"

# Copy binary
cp "target/release/$BINARY_NAME" "$RELEASE_DIR/tts-server/"

# Copy essential files
cp README.md "$RELEASE_DIR/tts-server/"
cp LICENSE "$RELEASE_DIR/tts-server/"
cp VOICE_API.md "$RELEASE_DIR/tts-server/"
cp RELEASE_NOTES.md "$RELEASE_DIR/tts-server/"
cp download_models.sh "$RELEASE_DIR/tts-server/"
cp start.sh "$RELEASE_DIR/tts-server/"

# Copy voice data
cp -r data/voices/*.bin "$RELEASE_DIR/tts-server/data/voices/" 2>/dev/null || true
cp -r data/voices/index.json "$RELEASE_DIR/tts-server/data/voices/" 2>/dev/null || true

# Create README for release
cat > "$RELEASE_DIR/tts-server/QUICK_START.md" << 'EOF'
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
EOF

# Create archive
cd "$RELEASE_DIR"

# macOS (universal binary)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "📦 Creating macOS archive..."
    tar -czf "tts-server-macos-v${VERSION}.tar.gz" tts-server/
    echo "✅ Created: tts-server-macos-v${VERSION}.tar.gz"
    echo "📊 Size: $(du -h "tts-server-macos-v${VERSION}.tar.gz" | cut -f1)"
fi

cd ..

echo ""
echo "✅ Release preparation complete!"
echo "📂 Release directory: $RELEASE_DIR"
echo ""
echo "Next steps:"
echo "1. Test the release package"
echo "2. Create GitHub release tag: git tag -a v${VERSION} -m 'Release v${VERSION}'"
echo "3. Push tag: git push origin v${VERSION}"
echo "4. Upload $RELEASE_DIR/tts-server-macos-v${VERSION}.tar.gz to GitHub Release"
echo "5. Copy RELEASE_NOTES.md content to GitHub Release description"
