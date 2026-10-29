# STT + TTS language coverage on this box

**Target languages (5):** English, Arabic, Japanese, Bangla, Hindi.

> **All 5 languages have a working STT and TTS model on disk** as of
> 2026-10-30. Hindi was the last gap and now uses a Hindi fine-tune for STT
> and Piper for TTS. Chinese was dropped from the target list.
>
> Hindi pins, with sha256 and measured CER/RTF: `decisions/configs/speech_hindi.json`.

**Combined footprint: ~1.33-1.44 GB** (STT ~828 MB + TTS ~498-605 MB).

---

## STT (Speech-to-Text) - all 5 covered

| Model | Size | Languages it covers | Role |
|---|---:|---|---|
| **Phonon-2.5** | ~156 MB | English (+ 24 other European) | locked default |
| **SenseVoice-Small INT8** | 226 MB | Japanese | locked (ja/zh) |
| **whisper-base-hindi int8** | **76 MB** | Hindi | locked (hi), added 2026-10-30 |
| **whisper-small int8** | ~244 MB | Bangla | locked (bn) |
| **Moonshine streaming tiny-ar** | ~27 MB | Arabic | locked (ar) |
| **Dolphin-base-ctc-int8** | 99 MB | Arabic, Bangla, Japanese (fast path) | locked (fast path) |

**STT total: ~828 MB** for all 5 languages (English, Arabic, Japanese, Bangla, Hindi).

## TTS (Text-to-Speech) - all 5 covered

| Model | Size | Languages it covers | Role |
|---|---:|---|---|
| **Supertonic 3** | ~383 MB | English, Arabic, Japanese (+ 28 others) | locked default |
| **Piper hi_IN** | **77 MB** | Hindi | locked (hi), added 2026-10-30 |
| **MMS-TTS Bangla** | ~38-145 MB | Bangla | locked (bn) |

**TTS total: ~498-605 MB** for all 5 languages (English, Arabic, Japanese, Bangla, Hindi).

## Grand total

| | Size |
|---|---:|
| STT (5 languages) | ~828 MB |
| TTS (5 languages) | ~498-605 MB |
| **Combined** | **~1.33-1.44 GB** |

## Per-language detail

| # | Language | STT model | TTS model |
|---|---|---|---|
| 1 | English | Phonon-2.5 | Supertonic 3 |
| 2 | Arabic | Moonshine tiny-ar + Dolphin | Supertonic 3 |
| 3 | Japanese | SenseVoice-Small INT8 / Dolphin | Supertonic 3 |
| 4 | Bangla | whisper-small int8 + Dolphin | MMS-TTS Bangla |
| 5 | Hindi | whisper-base-hindi int8 | Piper hi_IN |

## Notes

- Hindi cost two downloads on 2026-10-30: `collabora/whisper-base-hindi`
  (585 MB fp32, converted to a 76 MB int8 CTranslate2 runtime) and Piper
  hi_IN `pratham`/`priyamvada`/`rohan` (77 MB each with espeak-ng data).
  Measured: Hindi STT median CER 0.032, Hindi TTS round-trip CER 0.000.
- Stock multilingual whisper-base int8 is **not** a Hindi model: it emitted
  Latin transliteration or Urdu on the same clips (mean CER 1.015).
- Chinese was dropped from the target list; if needed later, add MMS-TTS Chinese
  (~38-145 MB, CC-BY-NC license).
- Supertonic 3 covers 31 languages total (more than the 5 target). MMS-TTS
  Bangla is the only per-language TTS model kept.
- Dolphin-base-ctc-int8 is a fast path for Asian languages (~48x RTF on Chinese);
  keeping it is worth the 99 MB.

---

MIT OR Apache-2.0
updated: 2026-10-30 00:00 UTC