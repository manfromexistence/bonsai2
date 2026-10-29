"""Historical fetch receipt for the Piper Hindi voice.

The voice downloaded here is now `G:\Dx\models\dx-tts-hi-1.0` (Piper hi_IN
`pratham`). Re-running downloads the original HF bundle into that folder; rename
its ONNX to `dx-tts-hi-1.0.onnx` afterwards, as the current model does.
"""
import os
import time

from huggingface_hub import snapshot_download

M = r"G:\Dx\models"
REPO = "csukuangfj/vits-piper-hi_IN-pratham-medium"
DEST = "dx-tts-hi-1.0"

t0 = time.time()
path = snapshot_download(repo_id=REPO, local_dir=rf"{M}\{DEST}")
size = sum(
    os.path.getsize(os.path.join(root, f))
    for root, _, files in os.walk(path)
    for f in files
)
print(f"{REPO}: {size / 1e6:.0f} MB in {time.time() - t0:.1f}s -> {path}", flush=True)
