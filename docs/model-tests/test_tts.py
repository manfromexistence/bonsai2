"""TTS smoke test: Supertonic 3 (local dir, offline) + MMS-TTS Bangla."""
import sys
import time

import soundfile as sf

OUT = r"G:\Temp\UserTemp\opencode\model-tests\out"
MODEL_DIR = r"G:\Dx\models\tts\Supertonic-3"

CASES = [
    ("en", "M1", "DX is online. The chat is running at seventy tokens per second."),
    ("ar", "M2", "\u0645\u0631\u062d\u0628\u0627\u060c \u0623\u0646\u0627 \u0645\u0633\u0627\u0639\u062f\u0643 \u0627\u0644\u0630\u0643\u064a."),
    ("ja", "F1", "\u3053\u3093\u306b\u3061\u306f\u3001DX\u3067\u3059\u3002\u4eca\u65e5\u306f\u3088\u3044\u5929\u6c17\u3067\u3059\u306d\u3002"),
    ("hi", "M3", "\u0928\u092e\u0938\u094d\u0924\u0947, \u092e\u0948\u0902 DX \u0939\u0942\u0901\u0964"),
]

from supertonic import TTS

t0 = time.time()
tts = TTS(model_dir=MODEL_DIR, auto_download=False, intra_op_num_threads=8)
print(f"load_s={time.time() - t0:.2f} model={MODEL_DIR}", flush=True)

for lang, voice, text in CASES:
    style = tts.get_voice_style(voice_name=voice)
    t0 = time.time()
    audio, duration = tts.synthesize(text, voice_style=style, lang=lang, verbose=False)
    dt = time.time() - t0
    path = fr"{OUT}\supertonic_{lang}.wav"
    tts.save_audio(audio, path)
    info = sf.info(path)
    dur = info.frames / info.samplerate
    print(f"[{lang}] voice={voice} sr={info.samplerate} audio_s={dur:.2f} wall_s={dt:.2f} "
          f"rtf={dt / max(dur, 0.01):.2f} -> {path}", flush=True)
