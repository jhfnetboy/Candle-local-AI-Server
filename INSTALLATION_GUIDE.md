# 🚀 TTS Server 安装指南

## macOS 用户安装步骤

### 1. 下载

下载最新版本:
```bash
curl -L -o tts-server-macos.tar.gz \
  https://github.com/jhfnetboy/Candle-local-AI-Server/releases/download/v0.1.0/tts-server-macos-v0.1.0.tar.gz
```

或者直接在浏览器中下载: [下载链接](https://github.com/jhfnetboy/Candle-local-AI-Server/releases/latest)

### 2. 解压

```bash
tar -xzf tts-server-macos-v0.1.0.tar.gz
cd tts-server
```

### 3. 安装依赖 (espeak-ng)

**使用 Homebrew (推荐)**:
```bash
brew install espeak-ng
```

如果没有 Homebrew，先安装:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 4. 允许运行程序

macOS 会阻止"未识别的开发者"的程序运行。你需要：

**方法 1: 使用命令行删除隔离属性 (推荐)**
```bash
xattr -d com.apple.quarantine tts-server
chmod +x tts-server
```

**方法 2: 在系统设置中允许**
1. 尝试运行 `./tts-server`
2. 如果弹出"无法打开"错误
3. 打开 **系统设置 → 隐私与安全性**
4. 找到 "tts-server" 的提示，点击 **"仍要打开"**
5. 再次运行 `./tts-server`

### 5. 启动服务器

```bash
./tts-server
```

你应该看到:
```
🎵 TTS Server v0.1.0
📡 Server starting on http://localhost:9527
📥 Downloading models (first run only)...
✅ Server ready!
```

### 6. 验证安装

打开新的终端窗口:
```bash
curl http://localhost:9527/health
```

应该返回:
```json
{"success":true,"data":"healthy"}
```

---

## Windows 用户安装步骤

### Windows 版本暂未发布

v0.1.0 版本仅支持 macOS。Windows 用户可以:

**选项 1: 等待 v0.2.0 版本 (推荐)**
- 预计 2024年12月中旬发布
- 将包含 Windows 安装程序 (.exe)
- 自动安装所有依赖

**选项 2: 从源码构建**

1. 安装依赖:
   - [Rust](https://www.rust-lang.org/tools/install)
   - [espeak-ng](https://github.com/espeak-ng/espeak-ng/releases)

2. 克隆并构建:
   ```powershell
   git clone https://github.com/jhfnetboy/Candle-local-AI-Server.git
   cd Candle-local-AI-Server
   cargo build --release
   ```

3. 运行:
   ```powershell
   target\release\tts-server.exe
   ```

---

## 常见问题

### ❓ "tts-server" cannot be opened because it is from an unidentified developer

**解决方法**:
```bash
xattr -d com.apple.quarantine tts-server
chmod +x tts-server
./tts-server
```

### ❓ 命令 `espeak-ng` not found

**解决方法**:
```bash
brew install espeak-ng
```

### ❓ 服务器无法启动，端口已被占用

**解决方法**:
```bash
# 检查是否有其他程序占用 9527 端口
lsof -i :9527

# 杀死占用端口的进程
kill -9 <PID>
```

### ❓ 模型下载失败

**解决方法**:
- 检查网络连接
- 重新启动服务器会自动重试
- 或者手动下载模型文件到 `checkpoints/` 目录

### ❓ MyDictionary 扩展无法检测到服务器

**解决方法**:
1. 确保服务器正在运行: `curl http://localhost:9527/health`
2. 刷新扩展设置页面
3. 检查浏览器控制台是否有 CORS 错误

---

## 如何在后台运行

### macOS/Linux

```bash
# 后台运行
nohup ./tts-server > tts-server.log 2>&1 &

# 停止后台进程
pkill tts-server
```

### 开机自启动 (macOS)

创建 `~/Library/LaunchAgents/com.tts-server.plist`:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.tts-server</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/YOUR_USERNAME/tts-server/tts-server</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

加载配置:
```bash
launchctl load ~/Library/LaunchAgents/com.tts-server.plist
```

---

## 卸载

```bash
# 停止服务器
pkill tts-server

# 删除程序文件
rm -rf ~/tts-server

# 删除自启动配置 (如果有)
launchctl unload ~/Library/LaunchAgents/com.tts-server.plist
rm ~/Library/LaunchAgents/com.tts-server.plist
```

---

## 需要帮助？

- 📖 [完整文档](https://github.com/jhfnetboy/Candle-local-AI-Server#readme)
- 🐛 [报告问题](https://github.com/jhfnetboy/Candle-local-AI-Server/issues)
- 💬 [讨论区](https://github.com/jhfnetboy/Candle-local-AI-Server/discussions)
