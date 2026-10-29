# TTS models in use (benchmarked on this machine)

This document records the text-to-speech models currently selected for this
project, with measured CPU performance on the reference machine
(Windows, AMD Radeon integrated graphics, no CUDA GPU, ~8 logical cores).

## Selection

| Role | Model | Why |
| --- | --- | --- |
| **Bangla (bn)** - special-purpose | `facebook/mms-tts-ben` (MMS-TTS Bangla, VITS) | Only Bangla-capable model that runs well on CPU; 36.3M params, 16 kHz |
| **Hindi (hi)** - locked 2026-10-30, special-purpose | `dx-tts-hi-1.0` (Piper hi_IN `pratham` voice) via sherpa-onnx | 77 MB, 22.05 kHz, RTF 0.05, round-trip CER 0.000 |
| **English, Arabic, Japanese** | **dx-tts-1.0** (Supertonic 3, ONNX) | Fastest on CPU for these (RTF 0.27-0.36 measured here), 31 languages, 10 preset voices, ~99M params |
| English high-quality (optional) | `hexgrad/Kokoro-82M` | Best English quality, RTF ~1.33 on CPU |
| Multilingual / cloning (GPU or slow CPU) | `k2-fsa/OmniVoice` / `Serveurperso/OmniVoice-GGUF` (Q4_K_M) | 600+ languages + zero-shot cloning; RTF ~4.75–8.05 on CPU |
| Bangla alternative | `rhasspy/piper` `bn_BD-google-medium` | Crisp Bangladeshi voice, 22.05 kHz, RTF ~1.39 |

Bangla is intentionally isolated to MMS-TTS Bangla because the other fast
models (Supertonic 3, Piper English) do not cover Bengali.

## Measured performance (this machine, CPU only)

| Model | Params | On-disk size | Sample rate | RTF (measured) | Avg CPU (8 threads) | Max CPU | Audio file |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **dx-tts-bn-1.0** (MMS Bangla, optimized) | 36.3M | ~38–145 MB (cached) | 16 kHz | **0.22–0.25×** | ~713% | ~819% | `mms_ben_long2.wav` (~55.5 s) |
| **dx-tts-1.0** (Supertonic 3, chunked Hindi) | ~99M | ~383 MB | 24 kHz | **0.18×** | ~605% | ~772% | `supertonic3_hindi_long3.wav` (~87.7 s) |
| **dx-tts-1.0** (Supertonic 3, short) | ~99M | ~383 MB | 24 kHz | 0.22–0.46× | — | — | `supertonic3_jarvis.wav` |
| Kokoro 82M | 82M | ~312 MB | 24 kHz | ~1.33× | — | — | `kokoro` outputs |
| OmniVoice (PyTorch) | 600M | ~3.37 GB | 24 kHz | ~8.05× | — | — | `omnivoice_local.wav` |
| OmniVoice-GGUF Q4_K_M | 0.6B | ~659 MB (407+252) | 24 kHz | ~4.75× | — | — | `omnivoice_gguf_q4.wav` |
| Piper bn_BD-google-medium | ~18M | ~76 MB | 22.05 kHz | ~1.39× | — | — | `piper_bn_test.wav` |
| **dx-tts-hi-1.0** (Piper hi_IN pratham) | ~18M | **77 MB** | 22.05 kHz | **0.05x** | - | - | `dx-tts-hi-1.0.wav` |
| Pocket TTS | ~100M | ~215 MB | 24 kHz | 0.41–1.22× | — | — | `pocket_test.wav` |
| Ming-3.0-tts-0.5B | 0.5B | ~2.75 GB | 44.1 kHz | ~45× (CPU) | — | — | `ming` outputs |

RTF = real-time factor (lower is faster; <1.0 means faster than real-time).

## Audio artifacts

- Bangla (MMS, natural): `G:\Temp\UserTemp\opencode\tts-bench\mms_ben_natural.wav` (~2.0 s)
- Bangla (MMS, ~1 min): `G:\Temp\UserTemp\opencode\tts-bench\mms_ben_long2.wav` (~55.5 s)
- Hindi (Supertonic 3, ~1.5 min): `G:\Temp\UserTemp\opencode\tts-bench\supertonic3_hindi_long3.wav` (~87.7 s)
- English (Supertonic 3): `G:\Temp\UserTemp\opencode\tts-bench\supertonic3_jarvis.wav`
- OmniVoice GGUF Q4: `G:\Temp\UserTemp\opencode\tts-bench\omnivoice_gguf_q4.wav`

## Notes

- MMS-TTS Bangla is non-deterministic (stochastic duration predictor). For reproducible output
  use a fixed seed and `noise_scale=0.5, noise_scale_w=0.6`.
- MMS outputs 16 kHz; resample to 22.05/24 kHz with a high-quality resampler if higher
  fidelity is needed (does not add detail, reduces harshness).
- Supertonic 3 does not support Bangla - that is why Bangla is isolated to MMS.
- Supertonic 3 **writes 44.1 kHz** on this build (measured 2026-10-30 on all
  four languages); the 24 kHz column above is the older reading and should not
  be quoted without re-checking.
- Hindi moved from Supertonic 3 to **Piper hi_IN** on 2026-10-30: 77 MB, 22.05
  kHz, RTF 0.05-0.07, round-trip CER 0.000. Supertonic's Hindi is intelligible
  (CER 0.067) but 5x larger and slower; the earlier "Hindi is weak" note was
  caused by the multilingual STT, not by the voice. **Locked** with sha256 in
  `decisions/configs/speech_hindi.json`.
- The Piper bundles here are the sherpa-onnx conversions
  (`csukuangfj/vits-piper-hi_IN-*-medium`), which carry `tokens.txt` and
  `espeak-ng-data/`; the raw `rhasspy/piper-voices` ONNX alone is not enough
  for sherpa-onnx.
- OmniVoice (600+ languages) is kept available for multilingual/cloning needs but is
  ~8–17× slower than Supertonic on this CPU.

MIT OR Apache-2.0
updated: 2026-10-30 00:00 UTC
