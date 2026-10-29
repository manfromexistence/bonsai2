"""Resume whisper-small-hindi and MMS-Hindi over plain HTTP (xet hangs here)."""
import os
import time

from huggingface_hub import hf_hub_download, snapshot_download

M = r"G:\Dx\models"
t0 = time.time()
p = hf_hub_download(
    repo_id="collabora/whisper-small-hindi",
    filename="model.safetensors",
    local_dir=rf"{M}\stt\whisper-small-hindi",
)
print(f"small safetensors {os.path.getsize(p) / 1e6:.0f} MB in {time.time() - t0:.0f}s", flush=True)

t0 = time.time()
d = snapshot_download(repo_id="facebook/mms-tts-hin", local_dir=rf"{M}\tts\MMS-TTS-Hindi")
size = sum(os.path.getsize(os.path.join(r, f)) for r, _, fs in os.walk(d) for f in fs)
print(f"mms-hi {size / 1e6:.0f} MB in {time.time() - t0:.0f}s -> {d}", flush=True)
