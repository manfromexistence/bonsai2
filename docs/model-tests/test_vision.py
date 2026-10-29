"""Vision smoke test: VisionPsy-Nano-460M-Flash from the local HF snapshot."""
import sys
import time

import torch
from PIL import Image
from transformers import AutoModelForImageTextToText, AutoProcessor

MODEL = r"G:\Dx\models\vlm\VisionPsy-Nano-460M-Flash"
IMAGE = sys.argv[1] if len(sys.argv) > 1 else r"G:\Dx\models\demo_frames_512x512\frame_00.png"
PROMPT = sys.argv[2] if len(sys.argv) > 2 else "Describe this image in one sentence."

t0 = time.time()
proc = AutoProcessor.from_pretrained(MODEL, trust_remote_code=True)
print(f"processor_s={time.time() - t0:.2f}", flush=True)

from transformers import AutoConfig

cfg = AutoConfig.from_pretrained(MODEL, trust_remote_code=True)
if hasattr(cfg, "compile_inference"):
    cfg.compile_inference = False

t0 = time.time()
model = AutoModelForImageTextToText.from_pretrained(
    MODEL, config=cfg, trust_remote_code=True, dtype=torch.float32
).eval()
print(f"model_load_s={time.time() - t0:.2f} params={sum(p.numel() for p in model.parameters())/1e6:.1f}M", flush=True)

image = Image.open(IMAGE).convert("RGB")
print(f"image={IMAGE} size={image.size}", flush=True)

# The model's jinja chat template concatenates `content` as a string, so a
# vision content-block list cannot go through apply_chat_template. The
# processor builds the image-string and the template itself.
inputs = proc(images=image, text=PROMPT, return_tensors="pt")

t0 = time.time()
with torch.no_grad():
    out = model.generate(**inputs, max_new_tokens=48, greedy=True)
dt = time.time() - t0
new_tokens = out.shape[1]
text = proc.batch_decode(out, skip_special_tokens=True)[0]
print(f"raw_ids={out.tolist()}")
print(f"generate_s={dt:.2f} new_tokens={new_tokens} tps={new_tokens / dt:.1f}")
print("ANSWER:", text.strip())
