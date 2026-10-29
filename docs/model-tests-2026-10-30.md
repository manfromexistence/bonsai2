# Local model tests, 2026-10-30

**Tier: executed.** Every number below came from a command run on this
machine on 2026-10-30. The scripts are in `docs/model-tests/`; the models are
the ones in `G:\Dx\models` (see the "Local models" section of `README.md`).

Python used: `G:\Dx\decisions\.venv-vision\Scripts\python.exe`
(`rapidocr-onnxruntime 1.4.4`, `sherpa_onnx 1.13.8`, `transformers 5.19.0`,
`torch 2.14.1+cpu`), plus the system Python 3.12 for the `supertonic` package.

## OCR - PP-OCRv6 Nano

Command: `python test_ocr.py ui_text.png` with
`RapidOCR(config_path="G:\Dx\models\ocr\PP-OCRv6-Nano\v6_config.yaml")`.

| Image | Lines | Load | Total | Result |
| --- | ---: | ---: | ---: | --- |
| 1100x420 rendered UI screenshot | 4 | 0.31 s | 0.51 s | **4/4 exact**, scores 0.958-0.988 |
| 512x512 game frame | 0 | - | 0.12 s | no text in the frame |

Read back exactly: `DX Chat - esportsfire`,
`dx-reasoning is answering at 74.3 tok/s`,
`Server port 3000 - models on 8788`, `OCR check: Hello 1234`.
Detection was 0.40 s of the 0.51 s; recognition 0.03 s.

## Vision - VisionPsy-Nano-460M-Flash

### HF safetensors path (transformers)

The model's `config.json` sets `compile_inference: true`. On this box that
compiles through `torch._inductor`, which needs MSVC `cl`, and fails with
`InvalidCxxCompiler: Compiler: cl is not found`. With `TORCHDYNAMO_DISABLE=1`
and `cfg.compile_inference = False`, it runs eagerly:

| Item | Value |
| --- | --- |
| Load | 2.90 s (processor 0.63 s) |
| Params | 460.1 M, float32, CPU |
| Prompt | "What text is written in this image? Read it exactly." |
| Output | exact read of all four screenshot lines |
| Speed | 48 new tokens in 8.55 s = **5.6 t/s** |

Note: this model's `generate()` returns **only the new ids**, not prompt plus
new; slicing at `input_ids.shape[1]` yields an empty string.

### GGUF path (llama.cpp `mtmd`)

Binary: `G:\Temp\UserTemp\opencode\vkb\bin\llama-mtmd-cli.exe` (the one in
`decisions/.tools/llamacpp` also exists). This is the shipping format and it
needs no compiler.

```powershell
llama-mtmd-cli.exe -m visionpsy-nano-460m-flash-q4_0.gguf `
  --mmproj mmproj-visionpsy-nano-460m-flash-q8.gguf `
  --image ui_text.png -p "What text is written in this image? Read it exactly." `
  -n 64 -t 8 --temp 0
```

- q4_0 + mmproj Q8: exact read of all four lines; model load 1.4 s.
- q4_k_m-imat + mmproj Q8 on `demo_frames_512x512/frame_00.png`:
  "A serene, sunlit scene features a moss-covered tree with a hidden cave,
  surrounded by lush greenery and a few rocks." (`vision_gguf.log`)

## STT - five languages

Both directions were tested: real recordings for English/Arabic, and
TTS-generated clips (text known) for Arabic/Japanese/Bangla/Hindi.

| Test | Model | Audio | Wall | RTF | Verdict |
| --- | --- | ---: | ---: | ---: | --- |
| en (JFK clip) | Phonon-2.5 | 3.8 s | 0.21 s decode | 0.055 | **exact** |
| ar | Phonon-2.5 | 5.0 s | 0.19 s | 0.04 | garbage, Latin transliteration |
| ar | Dolphin CTC int8 | 2.5 s | 0.04 s | 0.02 | **correct** |
| ja | Dolphin CTC int8 | 3.7 s | 0.05 s | 0.01 | **exact** |
| ja | SenseVoice-Small INT8 | 3.7 s | 0.13 s | 0.04 | partial (`日 D X今日良天気`) |
| bn | Dolphin CTC int8 | 2.3 s | 0.03 s | 0.02 | near-correct |
| bn | Whisper base int8 | 2.3 s | 0.38 s | 0.17 | Latin transliteration |
| hi | Dolphin CTC int8 | 2.0 s | 0.04 s | 0.02 | **fails** |
| hi | Whisper base int8 | 2.0 s | 0.25 s | 0.13 | Latin transliteration only |

Findings:

- Phonon-2.5 is exact on English and confirms its European-only scope: Arabic
  comes back as `Marhaban, Ana Jarfis...`, not Arabic script.
- Dolphin is the best on-disk option for Arabic and Japanese, and the fastest
  (RTF 0.01-0.02). SenseVoice is weaker than Dolphin on the Japanese clip.
- **Hindi is the gap.** Neither Dolphin nor Whisper base produces Devanagari;
  both emit romanized text. A Hindi-specific model (IndicConformer or
  whisper-small) is required before Hindi STT can be called done.
- Whisper base accepts only 30 s per call; the 161.7 s Hindi file was truncated
  and produced repeated noise.

## TTS - Supertonic 3 and MMS Bangla

Supertonic was loaded from the local directory with `auto_download=False`, so
nothing was fetched.

| Language | Voice | Audio | Wall | RTF | Output |
| --- | --- | ---: | ---: | ---: | --- |
| en | M1 | 5.09 s | 1.38 s | 0.27 | non-silent, peak 0.42 |
| ar | M2 | 2.51 s | 0.91 s | 0.36 | non-silent, peak 0.72 |
| ja | F1 | 3.69 s | 1.12 s | 0.30 | non-silent, peak 0.37 |
| hi | M3 | 1.95 s | 0.64 s | 0.33 | non-silent, peak 0.32 |
| bn (MMS) | - | 2.27 s | 1.07 s | 0.47 | non-silent, peak 0.94 |

Supertonic 3 writes **44.1 kHz** audio, not the 24 kHz that
`docs/tts-models.md` claims. Model load was 1.36 s. MMS Bangla loads from the
local snapshot in 0.89 s at 16 kHz.

Round-trip check (each TTS clip fed back into STT): Japanese came back exactly,
Arabic and Bangla came back correct or near-correct, and Hindi came back as
"Namaste med DXO" - audible, but only partly intelligible. Hindi is the weakest
of the four Supertonic voices and needs a listening pass before it is called
good.

## What this establishes

| Question | Answer |
| --- | --- |
| Can OCR run? | Yes - PP-OCRv6 Nano, exact on UI text, sub-second |
| Can vision run? | Yes - both HF fp32 and GGUF q4; the GGUF route is the practical one |
| Can STT run in all 5 languages? | 4 of 5 - English, Arabic, Japanese and Bangla work; Hindi is romanized only |
| Can TTS run in all 5 languages? | Yes - Supertonic 3 for en/ar/ja/hi, MMS for bn |

MIT OR Apache-2.0
updated: 2026-10-30 00:00 UTC
