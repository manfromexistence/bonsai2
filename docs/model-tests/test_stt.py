"""STT smoke test for the 5-language chain (sherpa-onnx models, local files)."""
import time

import librosa
import soundfile as sf
import sherpa_onnx

B = r"G:\Temp\UserTemp\opencode"
OUT = []


def load16(path):
    audio, sr = sf.read(path)
    if audio.ndim > 1:
        audio = audio.mean(axis=1)
    if sr != 16000:
        audio = librosa.resample(audio, orig_sr=sr, target_sr=16000)
        sr = 16000
    return audio.astype("float32"), sr


def run(rec, wav, tag, lang):
    audio, sr = load16(wav)
    t0 = time.time()
    stream = rec.create_stream()
    stream.accept_waveform(sr, audio)
    rec.decode_stream(stream)
    dt = time.time() - t0
    dur = len(audio) / sr
    line = f"[{tag}/{lang}] audio_s={dur:.1f} wall_s={dt:.2f} rtf={dt / dur:.2f}\n  TEXT: {stream.result.text}"
    print(line, flush=True)
    OUT.append(line)


# Dolphin CTC int8 - fast path (ar / bn / ja)
dolphin = sherpa_onnx.OfflineRecognizer.from_dolphin_ctc(
    model=fr"{B}\asr_dolphin\model.int8.onnx",
    tokens=fr"{B}\asr_dolphin\tokens.txt",
    num_threads=4,
)
run(dolphin, fr"{B}\tts-bench\arabic_test.wav", "DOLPHIN", "ar")
run(dolphin, fr"{B}\tts-bench\mms_ben_test.wav", "DOLPHIN", "bn")
run(dolphin, fr"{B}\tts-bench\supertonic3_japanese_female.wav", "DOLPHIN", "ja")

# SenseVoice-Small INT8 - Japanese
sv = sherpa_onnx.OfflineRecognizer.from_sense_voice(
    model=fr"{B}\tts-bench\sherpa_sensevoice\model.int8.onnx",
    tokens=fr"{B}\tts-bench\sherpa_sensevoice\tokens.txt",
    num_threads=4, sample_rate=16000, use_itn=True, language="ja",
)
run(sv, fr"{B}\tts-bench\supertonic3_japanese_female.wav", "SENSEVOICE", "ja")

# Whisper base int8 - Bangla + Hindi
for lang, wav in (("bn", fr"{B}\tts-bench\mms_ben_test.wav"),
                  ("hi", fr"{B}\tts-bench\supertonic3_hindi_long3.wav")):
    w = sherpa_onnx.OfflineRecognizer.from_whisper(
        encoder=fr"{B}\asr_whisper\base-encoder.int8.onnx",
        decoder=fr"{B}\asr_whisper\base-decoder.int8.onnx",
        tokens=fr"{B}\asr_whisper\base-tokens.txt",
        language=lang, task="transcribe", num_threads=4,
    )
    run(w, wav, "WHISPER-BASE", lang)

with open(r"G:\Temp\UserTemp\opencode\model-tests\stt_results.md", "w", encoding="utf-8") as f:
    f.write("# STT smoke test\n\n" + "\n".join(OUT) + "\n")
