#!/bin/bash

set -e

echo "📦 下载 Kokoro-82M TTS 模型..."

# 创建目录
mkdir -p checkpoints data

# 下载 ONNX 模型 (~310 MB)
if [ ! -f "checkpoints/kokoro-v1.0.onnx" ]; then
    echo "⬇️  下载 ONNX 模型 (约 310 MB)..."
    curl -L "https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX-timestamped/resolve/main/onnx/model.onnx" \
        -o checkpoints/kokoro-v1.0.onnx \
        --progress-bar
    echo "✅ ONNX 模型下载完成"
else
    echo "✅ ONNX 模型已存在，跳过下载"
fi

# 下载语音数据 (~50 MB)
if [ ! -f "data/voices-v1.0.bin" ]; then
    echo "⬇️  下载语音数据 (50 MB)..."
    curl -L "https://github.com/thewh1teagle/kokoro-onnx/releases/download/model-files-v1.0/voices-v1.0.bin" \
        -o data/voices-v1.0.bin \
        --progress-bar
    echo "✅ 语音数据下载完成"
else
    echo "✅ 语音数据已存在，跳过下载"
fi

# 下载 tokenizer 配置
if [ ! -f "checkpoints/tokenizer.json" ]; then
    echo "⬇️  下载 tokenizer 配置..."
    curl -L "https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX-timestamped/resolve/main/tokenizer.json" \
        -o checkpoints/tokenizer.json \
        --progress-bar
    echo "✅ Tokenizer 下载完成"
else
    echo "✅ Tokenizer 已存在，跳过下载"
fi

echo ""
echo "🎉 所有模型文件下载完成！"
echo ""
echo "文件列表:"
ls -lh checkpoints/
ls -lh data/
