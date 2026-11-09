"""Compare two PNGs: size, differing pixel count, bounding boxes of difference clusters.

usage: python -I compare.py A.png B.png [diff-out.png]
"""
import sys
from PIL import Image, ImageChops


def clusters(mask, gap=6):
    """Group set pixels of a 1-bit mask into boxes by merging boxes closer than `gap` px."""
    w, h = mask.size
    px = mask.load()
    boxes = []
    # row-run based boxes, then merge
    for y in range(h):
        x = 0
        while x < w:
            if px[x, y]:
                x0 = x
                while x < w and px[x, y]:
                    x += 1
                boxes.append([x0, y, x - 1, y])
            else:
                x += 1
    merged = True
    while merged:
        merged = False
        out = []
        while boxes:
            b = boxes.pop()
            i = 0
            while i < len(boxes):
                c = boxes[i]
                if (b[0] - gap <= c[2] and c[0] - gap <= b[2] and b[1] - gap <= c[3] and c[1] - gap <= b[3]):
                    b = [min(b[0], c[0]), min(b[1], c[1]), max(b[2], c[2]), max(b[3], c[3])]
                    boxes.pop(i)
                    merged = True
                else:
                    i += 1
            out.append(b)
        boxes = out
    return sorted(boxes, key=lambda b: (b[1], b[0]))


def main():
    a = Image.open(sys.argv[1]).convert("RGB")
    b = Image.open(sys.argv[2]).convert("RGB")
    print(f"A {sys.argv[1]}: {a.size[0]}x{a.size[1]}")
    print(f"B {sys.argv[2]}: {b.size[0]}x{b.size[1]}")
    if a.size != b.size:
        print("SIZE MISMATCH; comparing the common top-left area")
        w, h = min(a.size[0], b.size[0]), min(a.size[1], b.size[1])
        a, b = a.crop((0, 0, w, h)), b.crop((0, 0, w, h))
    diff = ImageChops.difference(a, b).convert("L").point(lambda v: 255 if v else 0).convert("1")
    total = a.size[0] * a.size[1]
    n = sum(1 for v in diff.getdata() if v)
    print(f"differing pixels: {n} of {total} ({100.0 * n / total:.3f}%)")
    if n:
        bb = diff.getbbox()
        print(f"overall bbox (x0,y0,x1,y1 inclusive-exclusive): {bb}")
        px = diff.load()
        for i, (x0, y0, x1, y1) in enumerate(clusters(diff), 1):
            cnt = sum(1 for y in range(y0, y1 + 1) for x in range(x0, x1 + 1) if px[x, y])
            print(f"  region {i}: x {x0}-{x1}, y {y0}-{y1} ({x1 - x0 + 1}x{y1 - y0 + 1}), {cnt} px")
    if len(sys.argv) > 3:
        out = Image.new("RGB", a.size, (255, 255, 255))
        out.paste((255, 0, 0), None, diff)
        out.save(sys.argv[3])
        print(f"diff mask written to {sys.argv[3]}")


if __name__ == "__main__":
    main()
