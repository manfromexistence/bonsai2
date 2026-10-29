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

mms_root = r"G:\Dx\zzz\MMS-TTS-Hindi"  # parked 2026-10-30
if os.path.exists(os.path.join(mms_root, "config.json")):
    snap = mms_root
elif os.path.isdir(os.path.join(mms_root, "snapshots")):
    snap = next(
        (os.path.join(mms_root, "snapshots", d)
         for d in os.listdir(os.path.join(mms_root, "snapshots"))
         if os.listdir(os.path.join(mms_root, "snapshots", d))),
        None,
    )
else:
    snap = None

if snap:
    import torch
    from transformers import AutoTokenizer, VitsModel

    t0 = time.time()
    mms = VitsModel.from_pretrained(snap).eval()
    tok = AutoTokenizer.from_pretrained(snap)
    print(f"mms load_s={time.time() - t0:.2f} sr={mms.config.sampling_rate}", flush=True)
    inputs = tok(REF, return_tensors="pt")
    t0 = time.time()
    with torch.no_grad():
        wave = mms(**inputs).waveform[0].numpy()
    dt = time.time() - t0
    path = fr"{OUT}\mms_hi.wav"
    sf.write(path, wave, mms.config.sampling_rate)
    dur = len(wave) / mms.config.sampling_rate
    print(f"mms hi audio_s={dur:.2f} wall_s={dt:.2f} rtf={dt / dur:.2f} -> {path}", flush=True)
else:
    print("mms hi not downloaded yet", flush=True)
