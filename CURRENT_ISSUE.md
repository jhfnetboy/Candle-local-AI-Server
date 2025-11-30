# ✅ 问题已解决 - 零音频输出

## 根本原因

**Style Vectors 加载错误**:
- `voices-v1.0.bin` 是一个 **NumPy .npz 文件** (压缩档案),包含多个声音
- 每个声音的形状是 `[510, 1, 256]`
- 之前的代码错误地把整个 27MB 文件当成了一个 `[512, 256]` 二进制数组

## 解决方案

### 1. 提取正确的 Style Vectors

使用 Python 从 .npz 文件中提取一个声音的数据:

```python
import numpy as np

data = np.load('data/voices-v1.0.bin', allow_pickle=True)
af_alloy = data['af_alloy']  # shape: [510, 1, 256]
vectors = af_alloy.squeeze(axis=1)  # shape: [510, 256]
vectors.astype(np.float32).tofile('data/voices_simple.bin')
```

### 2. 修正 Rust 代码

- 修改文件路径: `voices-v1.0.bin` → `voices_simple.bin`
- 修正 token 数量: `512` → `510`

## 测试结果

```bash
✅ 音频幅度: max=0.7270 (之前是 0.0000)
✅ 音频时长: 1.8秒 ("Hello world")
✅ 音频格式: pcm_s16le, 24000 Hz
✅ 音量: max_volume=-0.4 dB (正常)
✅ 削波: histogram_0db=6 (极少,正常)
```

## 文件变更

1. ✅ `src/tts_engine.rs:58` - 改用 `voices_simple.bin`
2. ✅ `src/tts_engine.rs:84` - TOKEN_LIMIT: 512 → 510
3. ✅ `data/voices_simple.bin` - 新增 (510KB)

## 下一步

现在可以在 Chrome 扩展中测试音频播放。

---

**解决时间**: 2025-11-30 23:45
**问题持续**: ~2小时
**关键发现**: NumPy .npz 文件格式与预期的原始二进制格式不同
