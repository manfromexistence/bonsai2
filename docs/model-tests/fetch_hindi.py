"""Fetch the Hindi candidates into G:\Dx\models (HF snapshot, resumable)."""
import os
import time

from huggingface_hub import hf_hub_download, snapshot_download

M = r"G:\Dx\models"
JOBS = [
    ("collabora/whisper-base-hindi", rf"{M}\stt\whisper-base-hindi"),
    ("collabora/whisper-small-hindi", rf"{M}\stt\whisper-small-hindi"),
    ("facebook/mms-tts-hin", rf"{M}\tts\MMS-TTS-Hindi"),
]

for repo, dst in JOBS:
    t0 = time.time()
    print(f"== {repo}", flush=True)
    path = snapshot_download(repo_id=repo, local_dir=dst)
    size = sum(
        os.path.getsize(os.path.join(root, f))
        for root, _, files in os.walk(path)
        for f in files
    )
    print(f"   {size / 1e6:.0f} MB in {time.time() - t0:.1f}s -> {path}", flush=True)

VOICES = ["pratham", "priyamvada", "rohan"]
for voice in VOICES:
    for ext in (".onnx", ".onnx.json"):
        rel = f"hi/hi_IN/{voice}/medium/hi_IN-{voice}-medium{ext}"
        t0 = time.time()
        p = hf_hub_download(
            repo_id="rhasspy/piper-voices",
            filename=rel,
            local_dir=rf"{M}\tts\piper-hi",
        )
        print(f"   {rel} {os.path.getsize(p) / 1e6:.1f} MB {time.time() - t0:.1f}s", flush=True)

print("done")
