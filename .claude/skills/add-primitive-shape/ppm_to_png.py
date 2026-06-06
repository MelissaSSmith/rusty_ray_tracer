"""Convert a P3 (ASCII) PPM render into a PNG for inspection.

Usage:  python ppm_to_png.py <input.ppm> <output.png>

The ray tracer writes plain-text P3 PPMs. The Pillow available on this machine
predates plain-PPM support (it only reads binary P6), so this parses the P3
tokens by hand and hands raw RGB bytes to Pillow. Writes <output.png> plus an
aspect-preserving <output>_720.png thumbnail.
"""
import sys
from PIL import Image

src = sys.argv[1]
dst = sys.argv[2]

with open(src, "r") as f:
    tokens = []
    for line in f:
        if "#" in line:                       # strip PPM comments
            line = line[: line.index("#")]
        tokens.extend(line.split())

assert tokens[0] == "P3", "expected P3, got %r" % tokens[0]
w = int(tokens[1])
h = int(tokens[2])
maxval = int(tokens[3])
data = tokens[4:]

expected = w * h * 3
print("header: %dx%d maxval=%d  values=%d expected=%d" % (w, h, maxval, len(data), expected))

vals = [int(v) for v in data[:expected]]
if maxval != 255:
    vals = [v * 255 // maxval for v in vals]

img = Image.frombytes("RGB", (w, h), bytes(bytearray(vals)))
img.save(dst)

thumb = img.copy()
thumb.thumbnail((720, 720))
thumb.save(dst.replace(".png", "_720.png"))
print("saved", dst)
