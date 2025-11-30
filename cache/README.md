# 音频缓存目录

此目录用于存储 TTS 生成的音频文件。

## 结构

```
cache/audio/
  ├── 6e3b6f3978e5cd96.wav  # SHA256(文本) 前 64 位
  ├── a7f4c9d2e1b8f3a5.wav
  └── ...
```

## 文件命名

- 使用文本内容的 SHA256 哈希前 8 字节 (16 字符十六进制)
- 扩展名: `.wav`
- 格式: 24kHz, 32-bit float, 单声道

## 缓存策略

- **TTL**: 1 小时 (3600 秒)
- **清理**: 自动过期检查 (访问时)
- **命中率**: 相同文本重复使用现有文件

## 性能优势

- 避免重复生成相同文本的音频
- 减少 90% 带宽 (返回 URL 而非二进制)
- 支持浏览器原生缓存
- 快速响应缓存命中请求

## API 响应示例

### 缓存未命中
```json
{
  "file_id": "6e3b6f3978e5cd96",
  "url": "http://localhost:9527/audio/6e3b6f3978e5cd96.wav",
  "cached": false
}
```

### 缓存命中
```json
{
  "file_id": "6e3b6f3978e5cd96",
  "url": "http://localhost:9527/audio/6e3b6f3978e5cd96.wav",
  "cached": true
}
```

## 访问文件

```bash
# 直接访问
curl http://localhost:9527/audio/6e3b6f3978e5cd96.wav --output audio.wav

# 在浏览器中播放
# <audio src="http://localhost:9527/audio/6e3b6f3978e5cd96.wav"></audio>
```

## 维护

缓存目录会自动创建，过期文件会在访问时自动清理。无需手动管理。
