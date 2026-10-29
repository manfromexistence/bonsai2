"""OCR smoke test: PP-OCRv6-Nano through rapidocr_onnxruntime."""
import sys
import time

from rapidocr_onnxruntime import RapidOCR

CONFIG = r"G:\Dx\models\ocr\v6_config.yaml"
IMAGES = sys.argv[1:] or [r"G:\Dx\models\demo_frames_512x512\frame_00.png"]

t0 = time.time()
engine = RapidOCR(config_path=CONFIG)
print(f"load_s={time.time() - t0:.2f}", flush=True)

for image in IMAGES:
    t0 = time.time()
    result, elapse = engine(image)
    dt = time.time() - t0
    texts = [item[1] for item in (result or [])]
    scores = [round(float(item[2]), 3) for item in (result or [])]
    print(f"\nimage={image}")
    print(f"lines={len(texts)} elapsed_s={dt:.2f} per_stage={elapse}")
    for text, score in list(zip(texts, scores))[:15]:
        print(f"  [{score}] {text}")
