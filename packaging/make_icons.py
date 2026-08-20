#!/usr/bin/env python3
"""Render the Windex app icon.

Draws a 1024x1024 RGBA master PNG with signed-distance-field antialiasing —
no third-party imaging libraries, so this runs on a stock macOS Python. The
bundle script downsamples the master into an .iconset with sips and packs it
with iconutil.

Usage: python3 packaging/make_icons.py <output.png>
"""

import math
import struct
import sys
import zlib

SIZE = 1024

# Background gradient (top -> bottom), sampled in sRGB.
BG_TOP = (0x3B, 0x82, 0xF6)
BG_BOTTOM = (0x1E, 0x3A, 0x8A)

TILE = (0xFF, 0xFF, 0xFF)


def rounded_rect_sdf(px, py, cx, cy, half_w, half_h, radius):
    """Signed distance from (px, py) to a rounded rect. Negative = inside."""
    dx = abs(px - cx) - (half_w - radius)
    dy = abs(py - cy) - (half_h - radius)
    outside = math.hypot(max(dx, 0.0), max(dy, 0.0))
    inside = min(max(dx, dy), 0.0)
    return outside + inside - radius


def coverage(dist, softness=1.0):
    """Antialiased inside-ness for a distance, smoothed over `softness` px."""
    if dist <= -softness:
        return 1.0
    if dist >= softness:
        return 0.0
    return (softness - dist) / (2.0 * softness)


def over(dst, src, alpha):
    """Source-over composite of an opaque src onto dst."""
    return tuple(int(round(s * alpha + d * (1.0 - alpha))) for d, s in zip(dst, src))


def build_pixels():
    # The squircle is inset from the canvas edge the way macOS app icons are.
    inset = 100.0
    half = (SIZE - 2 * inset) / 2.0
    center = SIZE / 2.0
    corner = 185.0

    # Window tiles: one tall pane on the left, two stacked on the right.
    pane_inset = 268.0
    pane_left = pane_inset
    pane_right = SIZE - pane_inset
    pane_top = pane_inset + 18.0
    pane_bottom = SIZE - pane_inset - 18.0
    gap = 26.0
    split_x = pane_left + (pane_right - pane_left) * 0.52
    split_y = pane_top + (pane_bottom - pane_top) * 0.5
    tile_radius = 26.0

    tiles = [
        (pane_left, pane_top, split_x - gap / 2, pane_bottom),
        (split_x + gap / 2, pane_top, pane_right, split_y - gap / 2),
        (split_x + gap / 2, split_y + gap / 2, pane_right, pane_bottom),
    ]
    tiles = [
        (
            (x0 + x1) / 2.0,
            (y0 + y1) / 2.0,
            (x1 - x0) / 2.0,
            (y1 - y0) / 2.0,
        )
        for (x0, y0, x1, y1) in tiles
    ]

    rows = []
    for y in range(SIZE):
        py = y + 0.5
        t = py / SIZE
        bg = tuple(
            int(round(a + (b - a) * t)) for a, b in zip(BG_TOP, BG_BOTTOM)
        )
        row = bytearray()
        for x in range(SIZE):
            px = x + 0.5

            bg_cov = coverage(
                rounded_rect_sdf(px, py, center, center, half, half, corner)
            )
            if bg_cov <= 0.0:
                row += b"\x00\x00\x00\x00"
                continue

            color = bg
            for cx, cy, hw, hh in tiles:
                c = coverage(rounded_rect_sdf(px, py, cx, cy, hw, hh, tile_radius))
                if c > 0.0:
                    color = over(color, TILE, c)

            alpha = int(round(bg_cov * 255))
            row += bytes((color[0], color[1], color[2], alpha))
        rows.append(bytes(row))
    return rows


def write_png(path, rows):
    raw = b"".join(b"\x00" + r for r in rows)

    def chunk(tag, data):
        body = tag + data
        return (
            struct.pack(">I", len(data))
            + body
            + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)
        )

    png = b"\x89PNG\r\n\x1a\n"
    png += chunk(b"IHDR", struct.pack(">IIBBBBB", SIZE, SIZE, 8, 6, 0, 0, 0))
    png += chunk(b"IDAT", zlib.compress(raw, 9))
    png += chunk(b"IEND", b"")

    with open(path, "wb") as fh:
        fh.write(png)


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: make_icons.py <output.png>")
    write_png(sys.argv[1], build_pixels())


if __name__ == "__main__":
    main()
