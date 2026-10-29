"""Fetch sherpa-onnx Piper Hindi TTS bundles (ONNX + tokens + espeak data)."""
import os
import time

from huggingface_hub import snapshot_download

M = r"G:\Dx\models\tts"
for repo, name in [
    ("csukuangfj/vits-piper-hi_IN-pratham-medium", "piper-hi-pratham"),
    ("csukuangfj/vits-piper-hi_IN-priyamvada-medium", "piper-hi-priyamvada"),
]:
    t0 = time.time()
    path = snapshot_download(repo_id=repo, local_dir=rf"{M}\{name}")
    size = sum(
        os.path.getsize(os.path.join(root, f))
        for root, _, files in os.walk(path)
        for f in files
    )
    print(f"{repo}: {size / 1e6:.0f} MB in {time.time() - t0:.1f}s -> {path}", flush=True)
print("done")
