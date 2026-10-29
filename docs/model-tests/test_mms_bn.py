"""MMS-TTS Bangla smoke test from the local snapshot (no download)."""
import time

import soundfile as sf
import torch
from transformers import AutoTokenizer, VitsModel

MODEL = (
    r"G:\Dx\models\tts\MMS-TTS-Bangla\snapshots"
    r"\0da99de6074c8829121cdabfbdba423af18e8e56"
)
OUT = r"G:\Temp\UserTemp\opencode\model-tests\out\mms_bn.wav"
TEXT = "\u0986\u09ae\u09bf \u09ac\u09be\u0982\u09b2\u09be\u09af\u09bc \u0995\u09a5\u09be \u09ac\u09b2\u09a4\u09c7 \u09aa\u09be\u09b0\u09bf\u0964"

t0 = time.time()
model = VitsModel.from_pretrained(MODEL).eval()
tok = AutoTokenizer.from_pretrained(MODEL)
print(f"load_s={time.time() - t0:.2f} sr={model.config.sampling_rate}", flush=True)

inputs = tok(TEXT, return_tensors="pt")
t0 = time.time()
with torch.no_grad():
    wave = model(**inputs).waveform[0].numpy()
dt = time.time() - t0
sf.write(OUT, wave, model.config.sampling_rate)
dur = len(wave) / model.config.sampling_rate
print(f"audio_s={dur:.2f} wall_s={dt:.2f} rtf={dt / dur:.2f} -> {OUT}")
