# OCR default model

**Default OCR model for the Dx ecosystem: `dx-ocr-1.0` (PP-OCRv6 Nano / tiny)** (1.5M params, ~1.7 MB det + ~4.3 MB rec ONNX).

- Runtime: RapidOCR (ONNX Runtime), CPU.
- Measured on a 1920x1080 desktop screenshot: **~1.6 s** end to end (det 0.56 s + rec 0.81 s), 58 boxes, full text block captured.
- Preprocessing: ImageNet mean `[0.485, 0.456, 0.406]`, std `[0.229, 0.224, 0.225]`, scale `1/255`, BGR (per `PaddlePaddle/PP-OCRv6_tiny_det_onnx` and `..._rec_onnx` `inference.yml`).
- Character dict: 6904 entries (`G:\Dx\models\dx-ocr-1.0\ppocrv6_keys.txt`).
- Model files: `G:\Dx\models\dx-ocr-1.0\det\inference.onnx`, `G:\Dx\models\dx-ocr-1.0\rec\inference.onnx`; config `G:\Dx\models\dx-ocr-1.0\v6_config.yaml`.

## Fallback

**PP-OCRv5 Mobile** (5M params, ~4.6 MB det + ~15.8 MB rec) is the documented fallback when a slightly larger model is acceptable. It is slower (~3.9 s on the same screenshot) and dropped lines in the measured run, so it is not the default.

## Ranking on the measured screen (1920x1080 terminal screenshot)

| Rank | Model | Params | Total time | Notes |
|---:|---|---:|---:|---|
| 1 | **dx-ocr-1.0** (PP-OCRv6 Nano tiny) | 1.5M | 1.60 s | Default; fastest, no dropped lines |
| 2 | PP-OCRv4 | ~10-15M | 8.50 s | Most accurate on UI chrome; fallback for accuracy |
| 3 | PP-OCRv6 Mobile (small) | 7.7M | 4.60 s | Middle ground |
| 4 | PP-OCRv5 Mobile | 5M | 3.94 s | Drops lines; not default |
| 5 | PP-OCRv6 Server (medium) | 34.5M | 14.65 s | Marginal gain, too slow |

## Evidence

- `G:\Temp\UserTemp\opencode\ocr_v6tiny.txt` (PP-OCRv6 Nano output)
- `G:\Temp\UserTemp\opencode\ocr_v5.txt` (PP-OCRv5 Mobile output)
- `G:\Temp\UserTemp\opencode\ocr_now.txt` (PP-OCRv4 output)
- Timings recorded via RapidOCR `elapse` on `screenshot_now.png`.

MIT OR Apache-2.0
updated: 2026-10-26 00:00 UTC