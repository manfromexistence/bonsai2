"""Round-trip STT: the TTS clips we just generated, whose text we know."""
import time

import librosa
import soundfile as sf
import sherpa_onnx

B = r"G:\Temp\UserTemp\opencode"
OUT = r"G:\Temp\UserTemp\opencode\model-tests\out"


def load16(path):
    audio, sr = sf.read(path)
    if audio.ndim > 1:
        audio = audio.mean(axis=1)
    if sr != 16000:
        audio = librosa.resample(audio, orig_sr=sr, target_sr=16000)
        sr = 16000
    return audio.astype("float32"), sr


def run(rec, wav, tag):
    audio, sr = load16(wav)
    t0 = time.time()
    stream = rec.create_stream()
    stream.accept_waveform(sr, audio)
    rec.decode_stream(stream)
    dt = time.time() - t0
    dur = len(audio) / sr
    print(f"[{tag}] audio_s={dur:.1f} wall_s={dt:.2f} rtf={dt / dur:.2f}\n  TEXT: {stream.result.text}",
          flush=True)


dolphin = sherpa_onnx.OfflineRecognizer.from_dolphin_ctc(
    model=fr"{B}\asr_dolphin\model.int8.onnx",
    tokens=fr"{B}\asr_dolphin\tokens.txt", num_threads=4,
)
for lang in ("ar", "ja", "hi"):
    run(dolphin, fr"{OUT}\supertonic_{lang}.wav", f"DOLPHIN/{lang}")
run(dolphin, fr"{OUT}\mms_bn.wav", "DOLPHIN/bn")

for lang, tag in (("ja", "ja"), ("auto", "auto")):
    sv = sherpa_onnx.OfflineRecognizer.from_sense_voice(
        model=fr"{B}\tts-bench\sherpa_sensevoice\model.int8.onnx",
        tokens=fr"{B}\tts-bench\sherpa_sensevoice\tokens.txt",
        num_threads=4, sample_rate=16000, use_itn=True, language=lang,
    )
    run(sv, fr"{OUT}\supertonic_ja.wav", f"SENSEVOICE/{tag}")

for lang, wav in (("hi", fr"{OUT}\supertonic_hi.wav"), ("bn", fr"{OUT}\mms_bn.wav")):
    w = sherpa_onnx.OfflineRecognizer.from_whisper(
        encoder=fr"{B}\asr_whisper\base-encoder.int8.onnx",
        decoder=fr"{B}\asr_whisper\base-decoder.int8.onnx",
        tokens=fr"{B}\asr_whisper\base-tokens.txt",
        language=lang, task="transcribe", num_threads=4,
    )
    run(w, wav, f"WHISPER/{lang}")
