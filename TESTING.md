# TTS Server 测试指南

## 当前状态

✅ **espeak-ng 集成成功**
- 音素化: 正常 (IPA 输出)
- 服务器: 运行中 (端口 9527)
- 缓存: 工作正常

## 测试步骤

### 1. 启动 TTS 服务器

```bash
cd tts-server
./start.sh
```

**预期输出**:
```
🚀 TTS 服务器启动脚本
================================
✅ espeak-ng: eSpeak NG text-to-speech: 1.52.0
🔧 编译并启动 TTS 服务器...
📡 端口: 9527
🎵 健康检查: http://localhost:9527/health
```

### 2. 测试 API 调用

```bash
# 健康检查
curl http://localhost:9527/health

# 合成语音
curl -X POST http://localhost:9527/synthesize \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello world"}'
```

**预期响应**:
```json
{
  "file_id": "abc123...",
  "url": "http://localhost:9527/audio/abc123.wav",
  "cached": false
}
```

### 3. 测试 Chrome 扩展集成

**前提条件**:
1. ✅ TTS 服务器运行中 (端口 9527)
2. ✅ 扩展已构建 (`pnpm build`)
3. ✅ 扩展已在 Chrome 中**重新加载**

**操作步骤**:

1. 打开 Chrome DevTools (F12)
2. 切换到 "Service Worker" 或 "Background" 标签
3. 在任意网页选中文本
4. 点击扩展的 TTS 按钮
5. 观察日志输出

**预期日志** (成功使用本地服务器):
```
🔊 TTS 请求: "Hello world"
✅ 本地 TTS 服务器可用: {...}
🎵 使用本地 Rust TTS 服务器
🎵 音频 URL: http://localhost:9527/audio/abc123.wav (缓存未命中)
✅ 音频已发送到 Offscreen Document (本地 TTS)
```

**错误日志** (降级到浏览器 TTS):
```
🔊 TTS 请求: "Hello world"
⚠️ 本地 TTS 服务器不可用
🎵 使用浏览器 TTS (SpeechT5)
```

### 4. 排查问题

#### 问题: 扩展仍使用 SpeechT5

**原因**: 扩展未重新加载

**解决方案**:
1. 打开 `chrome://extensions/`
2. 找到 MyDictionary 扩展
3. 点击 🔄 (重新加载) 按钮
4. 刷新测试页面
5. 重试

#### 问题: 服务器连接失败

**检查服务器状态**:
```bash
curl http://localhost:9527/health
```

**检查端口占用**:
```bash
lsof -i:9527
```

#### 问题: 音频无法播放

**检查浏览器日志**:
- 打开 DevTools → Console
- 查看 `[Offscreen]` 相关错误
- 检查 CORS 配置

## 性能验证

### 音频时长测试

```bash
# 短句
curl -X POST http://localhost:9527/synthesize \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello"}' \
  > /tmp/test.json

# 提取文件 ID
FILE_ID=$(cat /tmp/test.json | grep -o '"file_id":"[^"]*"' | cut -d'"' -f4)

# 检查时长
ffprobe -i cache/audio/${FILE_ID}.wav -show_entries format=duration -v quiet -of csv="p=0"
```

**预期结果**:
- "Hello" (1 个词): ~1.8 秒
- "Hello world" (2 个词): ~3.5 秒
- "Hello beautiful world" (3 个词): ~5 秒

### espeak-ng 日志验证

查看服务器日志中的音素化输出:

```
🔊 调用 espeak-ng: Hello world
📋 espeak-ng 返回状态: exit status: 0
✅ espeak-ng 音素化成功
📝 音素: həlˈoʊ wˈɜːld
```

## 已知限制

1. **长句音频时长偏长**: 
   - 原因: Kokoro-82M 模型特性
   - 建议: 文本分段处理

2. **首次推理慢**: 
   - 原因: ONNX 模型加载和预热
   - 改进: 使用量化模型 (INT8)

3. **CPU 推理**: 
   - 当前: 仅 CPU 推理
   - 未来: 支持 GPU 加速 (CUDA/Metal)

## 成功标准

✅ **所有测试通过**:
- [ ] 服务器启动成功
- [ ] API 健康检查正常
- [ ] espeak-ng 音素化工作
- [ ] 音频生成成功
- [ ] Chrome 扩展自动发现本地服务器
- [ ] 音频播放正常

---

**测试时间**: 2025-11-30
**版本**: v0.1.0
