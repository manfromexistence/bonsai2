"""Supertonic 3 Hindi clip for the same reference sentence (system Python)."""
import soundfile as sf

from supertonic import TTS

REF = "नमस्ते, मैं आपकी मदद के लिए यहाँ हूँ। आज मौसम बहुत अच्छा है।"
OUT = r"G:\Temp\UserTemp\opencode\model-tests\out\supertonic_hi_ref.wav"

tts = TTS(model_dir=r"G:\Dx\models\dx-tts-1.0", auto_download=False, intra_op_num_threads=8)
style = tts.get_voice_style(voice_name="M3")
audio, _ = tts.synthesize(REF, voice_style=style, lang="hi")
tts.save_audio(audio, OUT)
info = sf.info(OUT)
print(f"supertonic hi sr={info.samplerate} audio_s={info.frames / info.samplerate:.2f} -> {OUT}")
