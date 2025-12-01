# TTS Voice Selection API

## 概述

TTS 服务器现在支持 **54 个不同的声音**,包括美式、英式、法语、日语和中文等多种语言和口音。

## API 使用

### 基本请求

```bash
POST http://localhost:9527/synthesize
Content-Type: application/json

{
  "text": "Hello world",
  "voice": "am_michael"  # 可选参数
}
```

### 参数说明

| 参数 | 类型 | 必需 | 默认值 | 说明 |
|-----|------|------|-------|------|
| `text` | string | ✅ | - | 要合成的文本 |
| `voice` | string | ❌ | `af_alloy` | 声音名称 |
| `format` | string | ❌ | `wav` | 音频格式 |

### 响应格式

```json
{
  "file_id": "52efa9a9c840e992",
  "url": "http://localhost:9527/audio/52efa9a9c840e992.wav",
  "cached": false
}
```

## 可用声音列表

### 🇺🇸 美式英语 (American English)

#### 女声 (Female)
- `af_alloy` - Alloy (默认女声)
- `af_aoede` - Aoede
- `af_bella` - Bella
- `af_heart` - Heart
- `af_jessica` - Jessica
- `af_kore` - Kore
- `af_nicole` - Nicole
- `af_nova` - Nova
- `af_river` - River
- `af_sarah` - Sarah
- `af_sky` - Sky

#### 男声 (Male)
- `am_adam` - Adam
- `am_echo` - Echo
- `am_eric` - Eric
- `am_fenrir` - Fenrir
- `am_liam` - Liam
- **`am_michael` - Michael** ⭐ (推荐用于标准美音)
- `am_onyx` - Onyx
- `am_puck` - Puck
- `am_santa` - Santa

### 🇬🇧 英式英语 (British English)

#### 女声 (Female)
- `bf_alice` - Alice
- `bf_emma` - Emma
- `bf_isabella` - Isabella
- `bf_lily` - Lily

#### 男声 (Male)
- `bm_daniel` - Daniel
- `bm_fable` - Fable
- **`bm_george` - George** ⭐ (推荐用于英式发音)
- `bm_lewis` - Lewis

### 🇫🇷 法语 (French)
- `ff_siwis` - Siwis (女声)

### 🇯🇵 日语 (Japanese)
- `jf_alpha` - Alpha (女声)
- `jf_gongitsune` - Gongitsune (女声)
- `jf_nezumi` - Nezumi (女声)
- `jf_tebukuro` - Tebukuro (女声)
- `jm_kumo` - Kumo (男声)

### 🇨🇳 中文 (Chinese)

#### 女声 (Female)
- `zf_xiaobei` - 小贝
- `zf_xiaoni` - 小妮
- `zf_xiaoxiao` - 小小
- `zf_xiaoyi` - 小艺

#### 男声 (Male)
- `zm_yunjian` - 云健
- `zm_yunxi` - 云希
- `zm_yunxia` - 云霞
- `zm_yunyang` - 云扬

### 其他语言
- `ef_dora`, `em_alex`, `em_santa` - 其他欧洲语言
- `hf_alpha`, `hf_beta`, `hm_omega`, `hm_psi` - Hindi
- `if_sara`, `im_nicola` - Italian
- `pf_dora`, `pm_alex`, `pm_santa` - Portuguese

## 使用示例

### cURL 命令

#### 默认声音
```bash
curl -X POST "http://localhost:9527/synthesize" \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello world"}'
```

#### 指定声音
```bash
# 美式男声 Michael
curl -X POST "http://localhost:9527/synthesize" \
  -H "Content-Type: application/json" \
  -d '{"text": "death", "voice": "am_michael"}'

# 英式男声 George
curl -X POST "http://localhost:9527/synthesize" \
  -H "Content-Type: application/json" \
  -d '{"text": "death", "voice": "bm_george"}'
```

### JavaScript (扩展端)

```javascript
const response = await fetch('http://localhost:9527/synthesize', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    text: 'Hello world',
    voice: 'bm_george'  // 可选
  })
});

const result = await response.json();
// result.url: 音频文件 URL
```

## 推荐声音

根据不同使用场景的推荐:

| 场景 | 推荐声音 | 说明 |
|------|---------|------|
| 英文单词学习 | `bm_george`, `bm_daniel` | 英式发音更清晰 |
| 美式英语 | `am_michael`, `am_adam` | 标准美音 |
| 女声朗读 | `af_nova`, `af_sarah` | 声音自然清晰 |
| 中文朗读 | `zf_xiaoxiao`, `zm_yunxi` | 普通话标准 |

## 缓存机制

- 缓存键基于 **文本 + 声音** 的组合
- 相同文本不同声音会生成不同的音频文件
- 缓存有效期: 1 小时 (3600秒)
- 缓存目录: `cache/audio/`

## 性能说明

- 首次合成: ~3-5秒 (包含模型加载)
- 后续合成: ~1-2秒
- 缓存命中: ~10ms
- 所有 54 个声音在服务器启动时一次性加载
