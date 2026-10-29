# STT model in use

## Locked model

- **Model:** `FermionResearch/Phonon-2.5` (Phonon-2.5)
- **Role:** Default speech-to-text (ASR) for this project
- **Location:** `C:\Users\Computer\.cache\fermion\speech\FermionResearch__Phonon-2.5\` (cached via Fermion)
- **Size:** ~156 MB
- **Languages:** 25 European languages
- **Quality:** 1.60 % WER on LibriSpeech clean, 5.06 % Open-ASR mean (per project claims)
- **License:** (see FermionResearch/Phonon-2.5 model card)

## How to run

Using the Fermion CLI (venv at `G:\Dx\decisions\.venv-vision`):

```powershell
G:\Dx\decisions\.venv-vision\Scripts\fermion.exe transcribe "FermionResearch/Phonon-2.5" path\to\audio.wav
```

Outputs the transcript to stdout (all logs to stderr), so it pipes cleanly:

```powershell
...fermion.exe transcribe "FermionResearch/Phonon-2.5" clip.wav > transcript.txt
```

## Measured performance (this machine, CPU)

| Audio | Duration | Elapsed | RTF |
| --- | --- | --- | --- |
| `pocket_test.wav` | 4.08 s | ~3.31 s | ~0.81× |
| `demo_speech.wav` | 40.5 s | ~10.68 s | ~0.26x (~3.7x real-time) |

RTF improves on longer clips (model load / warm-up amortized).

## Notes

- Phonon-2.5 supersedes Phonon-2 (164 MB) on speed (~13 % faster on long clips),
  CPU efficiency (~12 % less avg CPU) and accuracy, while staying multilingual.
- Phonon-2 remains selectable via `DX_DECISIONS_STT_MODEL`.
- The 0.081 RTF row in `CLAIMS.md` was measured on Parakeet, not Phonon-2.5;
  do not carry it forward (see `CLAIMS.md` for the develop note).

## Evaluated alternatives (2026-10-26)

Tested on this machine via sherpa-onnx. Full results in
`G:\Dx\decisions\docs\ASR_MODELS.md`.

| Model | Size | English (40s) | Asian langs | Verdict |
|---|---:|---|---|---|
| Whisper Base (int8) | 152 MB | 4.86 s, accurate | Good (Chinese 1.135s) | Best English fallback |
| **dx-stt-multi-1.0** (Dolphin CTC int8) | 99 MB | Broken on English | 0.117s, 48x RTF | Fastest on Asian languages; copied into `G:\Dx\models\dx-stt-multi-1.0\` on 2026-10-30 |
| OmniASR CTC 300M (int4) | 237 MB | 9.29 s, accurate | 1.693s on Chinese | Most comprehensive (1600+ langs) |

**Recommendation:** **dx-stt-1.0** (Phonon-2.5) remains the locked default (25 European languages,
fastest on long clips). For non-European languages, use Whisper Base as fallback
(English + 99 langs) or Dolphin CTC (Asian languages, fastest).

## Hindi - added 2026-10-30

The multilingual Whisper Base int8 that used to stand in for Hindi is not usable:
on five FLEURS `hi_in` test clips it emitted Latin transliteration or Urdu script
(mean CER 1.015). A Hindi fine-tune measured as int8 CTranslate2 is the locked
default:

| Model | On-disk (int8) | Mean CER | Median CER | RTF |
| --- | ---: | ---: | ---: | ---: |
| **dx-stt-hi-1.0** (`collabora/whisper-base-hindi`) | **76 MB** | **0.135** | **0.032** | **0.22** |
| stock whisper-base int8 (replaced) | 152 MB | 1.015 | - | 0.13 |

The mean is dominated by one hard proper-noun clip; the median is 0.032. The
pin, with sha256 and the measured values, is
`decisions/configs/speech_hindi.json`.

Paths:

- int8 runtime: `G:\Dx\models\dx-stt-hi-1.0\` (76 MB)
- evaluation log: `docs/model-tests/hindi_bench.log`

Convert another Whisper fine-tune:

```powershell
ct2-transformers-converter --model <hf-dir> --output_dir <ct2-dir> `
  --quantization int8 --copy_files tokenizer.json preprocessor_config.json `
  generation_config.json
```

Run:

```python
from faster_whisper import WhisperModel
model = WhisperModel(r"G:\Dx\models\dx-stt-hi-1.0",
                     device="cpu", compute_type="int8", cpu_threads=8)
segments, info = model.transcribe(audio_float32_16k, language="hi", beam_size=5)
text = "".join(s.text for s in segments)
```

Pass a float32 numpy array, not a path: faster-whisper 1.2.1 in this venv has a
PyAV mismatch (`open() got an unexpected keyword argument 'metadata_errors'`).

MIT OR Apache-2.0
updated: 2026-10-30 00:00 UTC
