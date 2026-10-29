"""Generate the same Hindi sentence with the Piper voices and, if present, MMS."""
import os
import time

import sherpa_onnx
import soundfile as sf

OUT = r"G:\Temp\UserTemp\opencode\model-tests\out"
REF = "नमस्ते, मैं आपकी मदद के लिए यहाँ हूँ। आज मौसम बहुत अच्छा है।"


def piper(voice):
    d = rf"G:\Dx\models\tts\piper-hi-{voice}"
    onnx = next(os.path.join(d, f) for f in os.listdir(d) if f.endswith(".onnx"))
    cfg = sherpa_onnx.OfflineTtsConfig(
        model=sherpa_onnx.OfflineTtsModelConfig(
            vits=sherpa_onnx.OfflineTtsVitsModelConfig(
                model=onnx,
                tokens=fr"{d}\tokens.txt",
                data_dir=fr"{d}\espeak-ng-data",
            ),
            num_threads=4,
        ),
        max_num_sentences=1,
    )
    t0 = time.time()
    tts = sherpa_onnx.OfflineTts(cfg)
    load_s = time.time() - t0
    t0 = time.time()
    audio = tts.generate(REF, sid=0, speed=1.0)
    dt = time.time() - t0
    path = fr"{OUT}\piper_{voice}.wav"
    sf.write(path, audio.samples, audio.sample_rate)
    dur = len(audio.samples) / audio.sample_rate
    print(f"piper {voice} load_s={load_s:.2f} sr={audio.sample_rate} audio_s={dur:.2f} "
          f"wall_s={dt:.2f} rtf={dt / dur:.2f} -> {path}", flush=True)


for v in ("pratham", "priyamvada"):
    piper(v)
