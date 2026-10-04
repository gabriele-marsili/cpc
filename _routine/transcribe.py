#!/usr/bin/env python3
"""Trascrizione (inglese) di una registrazione CPC, da eseguire NEL CLOUD (il Mac non ha la CPU/rete adatta).

Setup nel cloud (HuggingFace è bloccato dal proxy, GitHub no):
  pip install sherpa-onnx --break-system-packages
  cd /tmp && curl -sL https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8.tar.bz2 | tar xj
  curl -sLo /tmp/silero_vad.onnx https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/silero_vad.onnx
  ffmpeg -v error -y -i audio.ogg -ar 16000 -ac 1 audio.wav
Uso:
  nohup python3 transcribe.py audio.wav transcript.txt > tr.log 2>&1 &     (~10 min per 100 min di audio con 2 core)
  opzionale: python3 transcribe.py audio.wav out.txt START_SEC DURATION_SEC   (prova su un pezzo)
Scrive righe "[MMM:SS] testo". Alla fine stampa "done <secondi>" su stderr.
"""
import sys, time, wave, numpy as np, sherpa_onnx
M = "/tmp/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/"
rec = sherpa_onnx.OfflineRecognizer.from_transducer(
    encoder=M + "encoder.int8.onnx", decoder=M + "decoder.int8.onnx", joiner=M + "joiner.int8.onnx",
    tokens=M + "tokens.txt", num_threads=2, model_type="nemo_transducer")
cfg = sherpa_onnx.VadModelConfig()
cfg.silero_vad.model = "/tmp/silero_vad.onnx"; cfg.silero_vad.min_silence_duration = 0.5
cfg.silero_vad.max_speech_duration = 25; cfg.sample_rate = 16000
vad = sherpa_onnx.VoiceActivityDetector(cfg, buffer_size_in_seconds=60)
w = wave.open(sys.argv[1]); sr = w.getframerate()
a = np.frombuffer(w.readframes(w.getnframes()), dtype=np.int16).astype(np.float32) / 32768
start = float(sys.argv[3]) if len(sys.argv) > 3 else 0
a = a[int(start * sr):]
if len(sys.argv) > 4:
    a = a[:int(float(sys.argv[4]) * sr)]
out = open(sys.argv[2], "w"); t0 = time.time()
win = cfg.silero_vad.window_size
def flush():
    while not vad.empty():
        seg = vad.front; s = rec.create_stream(); s.accept_waveform(sr, seg.samples); rec.decode_stream(s)
        ts = start + seg.start / sr; txt = s.result.text.strip()
        if txt:
            out.write(f"[{int(ts // 60):03d}:{int(ts % 60):02d}] {txt}\n"); out.flush()
        vad.pop()
for i in range(0, len(a), win):
    vad.accept_waveform(a[i:i + win]); flush()
vad.flush(); flush()
print("done", time.time() - t0, file=sys.stderr)
