"""Hindi STT benchmark: faster-whisper int8 models on real FLEURS speech."""
import csv
import io
import os
import sys
import tarfile
import time

import numpy as np
import soundfile as sf
from faster_whisper import WhisperModel

B = r"G:\Temp\UserTemp\opencode\model-tests"
TAR = fr"{B}\fleurs_hi_test.tar.gz"
TSV = fr"{B}\fleurs_hi_test.tsv"
AUD = fr"{B}\fleurs_hi"
MODELS = {
    "base-hi": r"G:\Dx\models\stt\whisper-base-hindi-ct2",
    "small-hi": r"G:\Dx\models\stt\whisper-small-hindi-ct2",
}
REF_TEXT = "नमस्ते, मैं आपकी मदद के लिए यहाँ हूँ। आज मौसम बहुत अच्छा है।"


def lev(a, b):
    if not a:
        return len(b)
    prev = list(range(len(b) + 1))
    for i, ca in enumerate(a, 1):
        cur = [i]
        for j, cb in enumerate(b, 1):
            cur.append(min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + (ca != cb)))
        prev = cur
    return prev[-1]


def norm(text):
    keep = []
    for ch in text:
        if ch in "।,!?;:\"'()[]{} ":
            continue
        keep.append(ch)
    return "".join(keep)


def cer(ref, hyp):
    r, h = norm(ref), norm(hyp)
    return lev(r, h) / max(len(r), 1)


def wer(ref, hyp):
    r, h = ref.split(), hyp.split()
    return lev(r, h) / max(len(r), 1)


def load_clips(n=5, max_sec=9.0):
    os.makedirs(AUD, exist_ok=True)
    rows = []
    with open(TSV, encoding="utf-8") as f:
        for row in csv.reader(f, delimiter="\t"):
            if len(row) < 6:
                continue
            wav, text, samples = row[1], row[2], int(row[-2])
            if samples / 16000.0 <= max_sec:
                rows.append((wav, text, samples / 16000.0))
            if len(rows) >= n:
                break
    wanted = {w for w, _, _ in rows}
    with tarfile.open(TAR) as tf:
        for member in tf.getmembers():
            name = os.path.basename(member.name)
            if name in wanted and not os.path.exists(os.path.join(AUD, name)):
                member.name = name
                tf.extract(member, AUD)
    return [(os.path.join(AUD, w), t, d) for w, t, d in rows]


def read16(wav):
    audio, sr = sf.read(wav)
    if audio.ndim > 1:
        audio = audio.mean(axis=1)
    if sr != 16000:
        import librosa

        audio = librosa.resample(audio, orig_sr=sr, target_sr=16000)
    return audio.astype("float32")


def transcribe(model, wav):
    audio = read16(wav)
    t0 = time.time()
    segments, info = model.transcribe(audio, language="hi", beam_size=5, vad_filter=False)
    text = "".join(s.text for s in segments).strip()
    return text, time.time() - t0, len(audio) / 16000.0


def main():
    clips = load_clips()
    print(f"clips={len(clips)}", flush=True)
    for name, path in MODELS.items():
        if not os.path.isdir(path):
            print(f"[{name}] missing {path}", flush=True)
            continue
        t0 = time.time()
        model = WhisperModel(path, device="cpu", compute_type="int8", cpu_threads=8)
        load_s = time.time() - t0
        print(f"\n[{name}] ct2 load_s={load_s:.2f}", flush=True)
        cs, ws, rs = [], [], []
        for wav, ref, dur in clips:
            hyp, dt, _ = transcribe(model, wav)
            c, w = cer(ref, hyp), wer(ref, hyp)
            cs.append(c)
            ws.append(w)
            rs.append(dt / max(dur, 0.1))
            print(f"  ref={ref}\n  hyp={hyp}\n  CER={c:.3f} WER={w:.3f} audio={dur:.1f}s wall={dt:.2f}s",
                  flush=True)
        print(f"[{name}] mean CER={np.mean(cs):.3f} mean WER={np.mean(ws):.3f} "
              f"mean RTF={np.mean(rs):.2f}", flush=True)

        # TTS round-trip: judge the generated Hindi clips with this model
        for clip in ("mms_hi.wav", "piper_pratham.wav", "piper_priyamvada.wav"):
            p = os.path.join(r"G:\Temp\UserTemp\opencode\model-tests\out", clip)
            if os.path.exists(p):
                hyp, dt, dur = transcribe(model, p)
                print(f"  TTS[{clip}] CER={cer(REF_TEXT, hyp):.3f} hyp={hyp}", flush=True)


if __name__ == "__main__":
    sys.exit(main())
