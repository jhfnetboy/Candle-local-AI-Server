#!/usr/bin/env python3
"""
MyDictionary TTS Server
本地 TTS 服务器,支持多模型切换
"""

from flask import Flask, request, jsonify, send_file
from flask_cors import CORS
import torch
import numpy as np
from transformers import AutoProcessor, AutoModel
import soundfile as sf
import io
import logging
import os

# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = Flask(__name__)
CORS(app)  # 允许 Chrome Extension 跨域访问

class TTSModelManager:
    """TTS 模型管理器"""

    def __init__(self):
        self.models = {}
        self.processors = {}
        self.current_model = None
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        logger.info(f"🖥️  使用设备: {self.device}")

        # 可用模型配置
        self.available_models = {
            "speecht5": {
                "name": "SpeechT5 (English)",
                "model_id": "microsoft/speecht5_tts",
                "vocoder_id": "microsoft/speecht5_hifigan",
                "language": "en",
                "quality": 6,
                "speed": "fast"
            },
            "cosyvoice": {
                "name": "CosyVoice (中英文)",
                "model_id": "FunAudioLLM/CosyVoice-300M",
                "language": "zh-en",
                "quality": 9,
                "speed": "medium"
            }
        }

    def load_model(self, model_key):
        """加载指定模型"""
        if model_key in self.models:
            logger.info(f"✅ 模型 {model_key} 已加载")
            self.current_model = model_key
            return True

        if model_key not in self.available_models:
            logger.error(f"❌ 未知模型: {model_key}")
            return False

        config = self.available_models[model_key]
        logger.info(f"📥 开始加载模型: {config['name']}")

        try:
            if model_key == "speecht5":
                # 加载 SpeechT5
                processor = AutoProcessor.from_pretrained(config["model_id"])
                model = AutoModel.from_pretrained(config["model_id"]).to(self.device)
                vocoder = AutoModel.from_pretrained(config["vocoder_id"]).to(self.device)

                self.models[model_key] = {"model": model, "vocoder": vocoder}
                self.processors[model_key] = processor

            elif model_key == "cosyvoice":
                # 加载 CosyVoice (需要特殊处理)
                # TODO: 实现 CosyVoice 加载逻辑
                logger.warning("⚠️  CosyVoice 支持开发中...")
                return False

            self.current_model = model_key
            logger.info(f"✅ 模型加载成功: {config['name']}")
            return True

        except Exception as e:
            logger.error(f"❌ 模型加载失败: {e}")
            return False

    def synthesize(self, text, **kwargs):
        """生成语音"""
        if not self.current_model:
            raise ValueError("没有加载的模型")

        if self.current_model == "speecht5":
            return self._synthesize_speecht5(text, **kwargs)
        elif self.current_model == "cosyvoice":
            return self._synthesize_cosyvoice(text, **kwargs)

    def _synthesize_speecht5(self, text, speaker_id=0):
        """SpeechT5 合成"""
        model_data = self.models["speecht5"]
        processor = self.processors["speecht5"]

        # 准备输入
        inputs = processor(text=text, return_tensors="pt").to(self.device)

        # 加载 speaker embeddings (使用预设的)
        # TODO: 支持自定义 speaker embeddings
        embeddings_dataset = torch.load(
            "speaker_embeddings.pt",
            map_location=self.device
        ) if os.path.exists("speaker_embeddings.pt") else None

        if embeddings_dataset is None:
            # 使用默认 embeddings
            speaker_embeddings = torch.zeros((1, 512)).to(self.device)
        else:
            speaker_embeddings = embeddings_dataset[speaker_id].unsqueeze(0)

        # 生成语音
        with torch.no_grad():
            speech = model_data["model"].generate_speech(
                inputs["input_ids"],
                speaker_embeddings,
                vocoder=model_data["vocoder"]
            )

        # 转换为 numpy array
        audio = speech.cpu().numpy()
        sample_rate = 16000

        return audio, sample_rate

    def _synthesize_cosyvoice(self, text, **kwargs):
        """CosyVoice 合成 (TODO)"""
        raise NotImplementedError("CosyVoice 支持开发中")


# 全局模型管理器
model_manager = TTSModelManager()

@app.route("/")
def index():
    """API 信息"""
    return jsonify({
        "name": "MyDictionary TTS Server",
        "version": "1.0.0",
        "status": "running",
        "current_model": model_manager.current_model,
        "available_models": list(model_manager.available_models.keys())
    })

@app.route("/models", methods=["GET"])
def list_models():
    """获取可用模型列表"""
    models_info = []
    for key, config in model_manager.available_models.items():
        models_info.append({
            "id": key,
            "name": config["name"],
            "language": config["language"],
            "quality": config["quality"],
            "speed": config["speed"],
            "loaded": key in model_manager.models,
            "current": key == model_manager.current_model
        })

    return jsonify({
        "success": True,
        "models": models_info
    })

@app.route("/models/<model_key>/load", methods=["POST"])
def load_model(model_key):
    """加载指定模型"""
    success = model_manager.load_model(model_key)

    if success:
        return jsonify({
            "success": True,
            "message": f"模型 {model_key} 加载成功",
            "current_model": model_manager.current_model
        })
    else:
        return jsonify({
            "success": False,
            "error": f"模型 {model_key} 加载失败"
        }), 500

@app.route("/synthesize", methods=["POST"])
def synthesize():
    """合成语音"""
    data = request.get_json()

    if not data or "text" not in data:
        return jsonify({
            "success": False,
            "error": "缺少 text 参数"
        }), 400

    text = data["text"]

    # 可选参数
    speaker_id = data.get("speaker_id", 0)
    output_format = data.get("format", "wav")  # wav | mp3

    try:
        # 生成语音
        audio, sample_rate = model_manager.synthesize(
            text,
            speaker_id=speaker_id
        )

        # 转换为音频文件
        audio_buffer = io.BytesIO()
        sf.write(audio_buffer, audio, sample_rate, format=output_format)
        audio_buffer.seek(0)

        # 返回音频文件
        return send_file(
            audio_buffer,
            mimetype=f"audio/{output_format}",
            as_attachment=True,
            download_name=f"tts.{output_format}"
        )

    except Exception as e:
        logger.error(f"❌ 合成失败: {e}")
        return jsonify({
            "success": False,
            "error": str(e)
        }), 500

@app.route("/health", methods=["GET"])
def health():
    """健康检查"""
    return jsonify({
        "status": "healthy",
        "device": model_manager.device,
        "models_loaded": len(model_manager.models),
        "current_model": model_manager.current_model
    })


if __name__ == "__main__":
    # 默认加载 SpeechT5
    logger.info("🚀 启动 TTS 服务器...")
    model_manager.load_model("speecht5")

    # 启动服务器
    app.run(
        host="0.0.0.0",
        port=5050,
        debug=False
    )
