#!/bin/bash

# TTS Server Startup Script
# 用于启动本地 TTS 服务器 (端口 9527)

set -e

cd "$(dirname "$0")"

echo "🚀 TTS 服务器启动脚本"
echo "================================"

# 检查 espeak-ng
if ! command -v espeak-ng &> /dev/null; then
    echo "❌ 错误: espeak-ng 未安装"
    echo "📦 请运行: brew install espeak-ng"
    exit 1
fi

echo "✅ espeak-ng: $(espeak-ng --version | head -1)"

# 检查端口占用
if lsof -ti:9527 >/dev/null 2>&1; then
    echo "⚠️ 端口 9527 已被占用"
    read -p "是否杀死占用进程? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        lsof -ti:9527 | xargs kill -9
        echo "✅ 已清理端口"
    else
        echo "❌ 退出启动"
        exit 1
    fi
fi

# 创建缓存目录
mkdir -p cache/audio

echo ""
echo "🔧 编译并启动 TTS 服务器..."
echo "📡 端口: 9527"
echo "🎵 健康检查: http://localhost:9527/health"
echo ""
echo "按 Ctrl+C 停止服务器"
echo "================================"
echo ""

# 启动服务器 (release 模式以获得更好的性能)
cargo run --release
