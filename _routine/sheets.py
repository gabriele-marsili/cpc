#!/usr/bin/env python3
"""Fogli di anteprima dei frame (da eseguire nel cloud, dove si possono guardare le immagini con Read).

  python3 sheets.py FRAMES_DIR OUT_DIR [PERIOD_SEC=20]
Seleziona i frame "prima di un cambio pagina/scroll" (inchiostro che sparisce) e li
impagina 3x2 con il timestamp, così si legge la lavagna del prof in pochi fogli.
Frame attesi: f_0001.jpg, f_0002.jpg, ... estratti ogni PERIOD_SEC secondi.
"""
import glob, os, sys
import numpy as np
from PIL import Image, ImageDraw
fd, od = sys.argv[1], sys.argv[2]; per = int(sys.argv[3]) if len(sys.argv) > 3 else 20
os.makedirs(od, exist_ok=True)
fs = sorted(glob.glob(os.path.join(fd, "f_*.jpg")))
arr = [np.asarray(Image.open(f).convert("L").resize((342, 256)), dtype=np.int16) for f in fs]
keep = []
for i in range(len(fs)):
    if i == len(fs) - 1:
        keep.append(i); break
    a, b = arr[i][42:] < 120, arr[i + 1][42:] < 120
    if (a & ~b).sum() > 150:
        keep.append(i)
W, H = 640, 480
for s in range(0, len(keep), 6):
    sheet = Image.new("RGB", (W * 3, H * 2), "white"); d = ImageDraw.Draw(sheet)
    for j, k in enumerate(keep[s:s + 6]):
        im = Image.open(fs[k]); w, h = im.size
        im = im.crop((0, int(h * 0.107), w, h)).resize((W, H - 20))
        x, y = (j % 3) * W, (j // 3) * H; sheet.paste(im, (x, y + 20)); d.rectangle([x, y, x + W - 1, y + H - 1], outline="gray")
        t = k * per
        d.text((x + 5, y + 3), f"{os.path.basename(fs[k])}  t={t // 60}:{t % 60:02d}", fill="red")
    sheet.save(os.path.join(od, f"sheet_{s // 6:02d}.png"))
print(len(keep), "frame selezionati,", (len(keep) + 5) // 6, "fogli in", od)
