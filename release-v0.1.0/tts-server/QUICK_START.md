# 🎵 TTS Server - 快速开始

## macOS 用户 (必须使用命令行)

**⚠️ 重要: 不要双击 tts-server 文件！请在终端中运行。**

### 步骤 1: 安装依赖

```bash
brew install espeak-ng
```

如果没有 Homebrew:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 步骤 2: 允许运行程序

macOS 会阻止"未识别开发者"的程序。运行以下命令解除限制:

```bash
xattr -d com.apple.quarantine tts-server
chmod +x tts-server
```

### 步骤 3: 启动服务器

```bash
./tts-server
```

首次运行会自动下载模型 (~49MB)，之后启动更快。

### 步骤 4: 验证安装

打开新终端窗口:
```bash
curl http://localhost:9527/health
```

看到 `{"success":true,"data":"healthy"}` 说明成功！

---

## Windows 用户

Windows 版本将在 v0.2.0 发布 (预计12月中旬)。

现在可以从源码构建:
1. 安装 [Rust](https://www.rust-lang.org/tools/install) 和 [espeak-ng](https://github.com/espeak-ng/espeak-ng/releases)
2. 克隆仓库并运行 `cargo build --release`

---

## 与 MyDictionary 扩展集成

1. 保持服务器运行
2. 安装 MyDictionary Chrome 扩展
3. 打开扩展设置 → TTS Voice Settings
4. 看到绿色 "✅ Connected" 状态
5. 选择你喜欢的 54 种语音之一
6. 开始使用离线 TTS！

---

## 常见问题

**Q: "tts-server" cannot be opened because it is from an unidentified developer**

A: 运行命令解除限制:
```bash
xattr -d com.apple.quarantine tts-server
chmod +x tts-server
```

**Q: espeak-ng: command not found**

A: 安装 espeak-ng:
```bash
brew install espeak-ng
```

**Q: 如何在后台运行？**

A: 使用 nohup:
```bash
nohup ./tts-server > tts-server.log 2>&1 &
```

停止后台进程:
```bash
pkill tts-server
```

---

📖 详细文档: [README.md](https://github.com/jhfnetboy/Candle-local-AI-Server#readme)
🐛 报告问题: [GitHub Issues](https://github.com/jhfnetboy/Candle-local-AI-Server/issues)
