#!/usr/bin/env python3
"""summarize.py: medians of bench.py's JSON lines, as Markdown (docs) or HTML (site).

    summarize.py results.jsonl [--html | --bars]
    summarize.py results.jsonl --versions [--html]            # Rookparcel's versions side by side
    summarize.py results.jsonl --fill site/benchmarks.html   # the charts and tables in place
    summarize.py add-only.jsonl --fill-add site/benchmarks.html BLOCK   # an adding-only run

Every table on the page gets a grid of small bar charts (`bench:charts-NAME`), one per column or
measure: median bars from zero, a whisker for the range across runs, plain HTML and CSS.
"""

import json
import statistics
import sys

ORDER = ["rookparcel", "qbittorrent", "transmission", "deluge", "rtorrent"]
NAMES = {"rookparcel": "Rookparcel", "qbittorrent": "qBittorrent", "transmission": "Transmission",
         "deluge": "Deluge", "rtorrent": "rTorrent"}


def load(path):
    with open(path) as f:
        rows = [json.loads(line) for line in f if line.strip()]
    for r in rows:
        # Rookparcel was PaTorrent until 1.2.2: the runs of October 2026 say so
        r["client"] = r["client"].replace("patorrent", "rookparcel")
    return rows


def many_ok(rows, client, count):
    return [r for r in rows if r["scenario"] == "many" and r["client"] == client
            and r["count"] == count and "error" not in r]


def transfer_ok(rows, client):
    return [r for r in rows if r["scenario"] == "transfer" and r["client"] == client
            and "error" not in r and r.get("verified")]


def runs_note(counts):
    """`median of N, ` when every client has the same N > 1 runs."""
    counts = {n for n in counts if n}
    if len(counts) == 1 and max(counts) > 1:
        return f"median of {max(counts)}, "
    return "median, " if counts and max(counts) > 1 else ""


def med(rows, key):
    vals = [r[key] for r in rows if isinstance(r.get(key), (int, float))]
    return statistics.median(vals) if vals else None


def spread(rows, key):
    vals = [r[key] for r in rows if isinstance(r.get(key), (int, float))]
    return (min(vals), max(vals)) if vals else (None, None)


def fmt(v, digits=1):
    if v is None:
        return "–"
    if v >= 100:
        return f"{v:,.0f}"
    return f"{v:,.{digits}f}"


def transfer_table(rows):
    out = [["Client", "Version", "Runs", "Throughput (MiB/s)", "range", "CPU s per GiB", "range",
            "First byte to done (s)"]]
    for c in ORDER:
        rs = [r for r in rows if r["scenario"] == "transfer" and r["client"] == c]
        if not rs:
            continue
        ok = [r for r in rs if "error" not in r and r.get("verified")]
        lo, hi = spread(ok, "mib_per_s")
        clo, chi = spread(ok, "cpu_s_per_gib")
        version = next((r["version"] for r in rs if "version" in r), "")
        for r in ok:
            r["done_s"] = r["seconds"] - r["first_byte_s"]
        out.append([NAMES[c], version, f"{len(ok)} of {len(rs)}", fmt(med(ok, "mib_per_s")),
                    f"{fmt(lo)}–{fmt(hi)}", fmt(med(ok, "cpu_s_per_gib"), 2), f"{fmt(clo, 2)}–{fmt(chi, 2)}",
                    fmt(med(ok, "done_s"))])
    return out


def many_table(rows, count):
    out = [["Client", "Add (s)", "Start with all listed (s)", "Memory, RSS (MB)", "PSS (MB)",
            "Idle CPU (%)", "Empty RSS (MB)"]]
    for c in ORDER:
        rs = [r for r in rows if r["scenario"] == "many" and r["client"] == c and r["count"] == count]
        if not rs:
            continue
        ok = many_ok(rows, c, count)
        if not ok:
            out.append([NAMES[c], rs[-1]["error"][:60], "", "", "", "", ""])
            continue
        out.append([NAMES[c], fmt(med(ok, "add_s")), fmt(med(ok, "restart_s"), 2),
                    fmt(med(ok, "restarted_rss_mb")), fmt(med(ok, "restarted_pss_mb")),
                    fmt(med(ok, "restarted_idle_cpu_pct"), 2), fmt(med(ok, "empty_rss_mb"))])
    return out


def versions_table(rows):
    """Rookparcel's versions measured in one session (`bench.py --control`), one column each."""
    clients = []
    for r in rows:
        if r["client"].split("@")[0] == "rookparcel" and r["client"] not in clients:
            clients.append(r["client"])
    clients.sort(key=lambda c: (c == "rookparcel", [int(p) if p.isdigit() else 0
                                                     for p in c.partition("@")[2].split(".")]))

    def label(c):
        version = next((r["version"] for r in rows if r["client"] == c and "version" in r), c)
        return version.split()[-1]

    def cell(rs, key, digits=1):
        lo, hi = spread(rs, key)
        if lo is None:
            return "–"
        return f"{fmt(med(rs, key), digits)} ({fmt(lo, digits)}–{fmt(hi, digits)})"
    out = [["Measure"] + [label(c) for c in clients]]
    t = {c: transfer_ok(rows, c) for c in clients}
    if any(t.values()):
        out.append([f"Download, MiB/s ({runs_note(map(len, t.values()))}range)"]
                   + [cell(t[c], "mib_per_s") for c in clients])
        out.append(["Download, CPU s per GiB"] + [cell(t[c], "cpu_s_per_gib", 2) for c in clients])
    for count in sorted({r["count"] for r in rows if r["scenario"] == "many"}):
        m = {c: many_ok(rows, c, count) for c in clients}
        if not any(m.values()):
            continue
        for key, name, digits in (("add_s", "add (s)", 2), ("restart_s", "start with all listed (s)", 2),
                                  ("restarted_rss_mb", "memory after a restart (MB)", 1),
                                  ("added_rss_mb", "memory right after adding (MB)", 1),
                                  ("restarted_idle_cpu_pct", "idle CPU (%)", 2)):
            out.append([f"{count:,} torrents: {name}"] + [cell(m[c], key, digits) for c in clients])
        if count == max(r["count"] for r in rows if r["scenario"] == "many"):
            out.append(["Empty session, memory (MB)"] + [cell(m[c], "empty_rss_mb") for c in clients])
    return out


def markdown(table):
    lines = ["| " + " | ".join(table[0]) + " |", "|" + "---|" * len(table[0])]
    lines += ["| " + " | ".join(row) + " |" for row in table[1:]]
    return "\n".join(lines)


def html(table):
    import html as h
    head = "".join(f'<th scope="col">{h.escape(c)}</th>' for c in table[0])
    body = []
    for row in table[1:]:
        cells = [f'<td class="client">{h.escape(row[0])}</td>']
        cells += [f'<td class="num">{h.escape(v)}</td>' if v[:1].isdigit() or v[:1] == "–"
                  else f"<td>{h.escape(v)}</td>" for v in row[1:]]
        body.append("<tr>" + "".join(cells) + "</tr>")
    return (f'<table class="listview bench">\n<thead><tr>{head}</tr></thead>\n<tbody>\n'
            + "\n".join(body) + "\n</tbody>\n</table>")


def bars(caption, note, values, unit, digits=0, names=NAMES, accent=("rookparcel",), small=False):
    """A bar chart in plain HTML: one bar per client (or version), the `accent` ones in the
    accent colour. `values` holds (key, median) or (key, median, low, high); a low-high pair
    draws a whisker over the bar for the range across runs. Bars start at zero."""
    import html as h
    values = [v if len(v) == 4 else (*v, None, None) for v in values]
    values = [v for v in values if v[1] is not None]
    top = max(max(m, hi or 0) for _, m, _, hi in values) or 1
    items = []
    for c, v, lo, hi in values:
        width = max(100 * v / top, 0.6)
        cls = ' class="pa"' if c in accent else ""
        text = f"{fmt(v, digits)} {unit}".strip()
        rng, tip = "", text
        if lo is not None and hi is not None and fmt(lo, digits) != fmt(hi, digits):
            rng = (f'<span class="rng" style="left:{100 * lo / top:.1f}%;'
                   f'width:{100 * (hi - lo) / top:.1f}%"></span>')
            tip = f"{text}, range {fmt(lo, digits)}–{fmt(hi, digits)}"
        items.append(f'<li{cls} title="{h.escape(names.get(c, c))}: {h.escape(tip)}">'
                     f'<span class="who">{h.escape(names.get(c, c))}</span><span class="track">'
                     f'<span class="bar" style="width:{width:.1f}%"></span>{rng}</span>'
                     f'<span class="val">{h.escape(text)}</span></li>')
    cls = "chart small" if small else "chart"
    return (f'<figure class="{cls}">\n<figcaption><strong>{h.escape(caption)}</strong> '
            f'<span>{h.escape(note)}</span></figcaption>\n<ul class="bars">\n'
            + "\n".join(items) + "\n</ul>\n</figure>")


def grid(figures):
    return '<div class="chart-grid">\n' + "\n".join(figures) + "\n</div>"


def stat(rs, key):
    """(median, low, high) of `key` over runs `rs`, or Nones."""
    lo, hi = spread(rs, key)
    return (med(rs, key), lo, hi) if lo is not None else (None, None, None)


# the columns of many_table, as charts: key, caption, unit, digits, which way is better
MANY_MEASURES = [
    ("add_s", "Add", "s", 2, "lower"),
    ("restart_s", "Start with all listed", "s", 2, "lower"),
    ("restarted_rss_mb", "Memory after a restart (RSS)", "MB", 1, "lower"),
    ("restarted_pss_mb", "Memory after a restart (PSS)", "MB", 1, "lower"),
    ("restarted_idle_cpu_pct", "Idle CPU", "%", 2, "lower"),
    ("empty_rss_mb", "Empty session, memory (RSS)", "MB", 1, "lower"),
]


def many_charts(rows, count):
    """Small charts beside many_table: one per column, one bar per client."""
    m = {c: many_ok(rows, c, count) for c in ORDER}
    note = runs_note(map(len, m.values()))
    return grid([bars(cap, f"{unit}, {note}{better} is better",
                      [(c, *stat(m[c], key)) for c in ORDER if m[c]], unit, digits, small=True)
                 for key, cap, unit, digits, better in MANY_MEASURES])


def transfer_charts(rows):
    t = {c: transfer_ok(rows, c) for c in ORDER}
    for rs in t.values():
        for r in rs:
            r["done_s"] = r["seconds"] - r["first_byte_s"]
    note = runs_note(map(len, t.values()))
    return grid([bars("First byte to done", f"seconds, {note}lower is better",
                      [(c, *stat(t[c], "done_s")) for c in ORDER if t[c]], "s", 1, small=True)])


def version_clients(rows, others=()):
    """Rookparcel's versions in a run, oldest first and `rookparcel` (the newest) last, then
    `others` that ran; with their labels."""
    clients = []
    for r in rows:
        if r["client"].split("@")[0] == "rookparcel" and r["client"] not in clients:
            clients.append(r["client"])
    clients.sort(key=lambda c: (c == "rookparcel", [int(p) if p.isdigit() else 0
                                                     for p in c.partition("@")[2].split(".")]))
    clients += [c for c in others if any(r["client"] == c for r in rows)]
    names = {c: next((r["version"] for r in rows if r["client"] == c and "version" in r), c).split()[-1]
             if c.startswith("rookparcel") else NAMES.get(c, c) for c in clients}
    return clients, names


def versions_charts(rows):
    """Small charts beside versions_table: one per measure, one bar per version, the newest
    in the accent colour."""
    clients, names = version_clients(rows)
    figs = []
    t = {c: transfer_ok(rows, c) for c in clients}
    if any(t.values()):
        note = runs_note(map(len, t.values()))
        figs.append(bars("Download speed", f"MiB/s, {note}higher is better",
                         [(c, *stat(t[c], "mib_per_s")) for c in clients], "MiB/s", 0, names, small=True))
        figs.append(bars("Download, CPU per GiB", f"seconds, {note}lower is better",
                         [(c, *stat(t[c], "cpu_s_per_gib")) for c in clients], "s", 2, names, small=True))
    counts = sorted({r["count"] for r in rows if r["scenario"] == "many"})
    for count in counts:
        m = {c: many_ok(rows, c, count) for c in clients}
        if not any(m.values()):
            continue
        note = runs_note(map(len, m.values()))
        for key, cap, unit, digits in (("add_s", "add", "s", 2), ("restart_s", "start with all listed", "s", 2),
                                       ("restarted_rss_mb", "memory after a restart", "MB", 1),
                                       ("added_rss_mb", "memory right after adding", "MB", 0),
                                       ("restarted_idle_cpu_pct", "idle CPU", "%", 2)):
            figs.append(bars(f"{count:,} torrents: {cap}", f"{unit}, {note}lower is better",
                             [(c, *stat(m[c], key)) for c in clients], unit, digits, names, small=True))
        if count == counts[-1]:
            figs.append(bars("Empty session, memory", f"MB, {note}lower is better",
                             [(c, *stat(m[c], "empty_rss_mb")) for c in clients], "MB", 1, names,
                             small=True))
    return grid(figs)


def add_charts(rows):
    """A run that measured only adding (versions and other clients): one chart per count."""
    clients, names = version_clients(rows, others=ORDER[1:])
    figs = []
    for count in sorted({r["count"] for r in rows if r["scenario"] == "many"}):
        m = {c: many_ok(rows, c, count) for c in clients}
        note = runs_note(map(len, m.values()))
        figs.append(bars(f"Adding {count:,} torrents", f"seconds, {note}lower is better",
                         [(c, *stat(m[c], "add_s")) for c in clients], "s", 2, names, small=True))
    return grid(figs)


def fill(page, rows):
    """Replaces what stands between `<!-- bench:NAME -->` and `<!-- /bench:NAME -->` in `page`."""
    import re
    blocks = {f"chart-{i}": c for i, c in enumerate(charts(rows).split("\n\n"))}
    blocks["table-transfer"] = html(transfer_table(rows))
    blocks["charts-transfer"] = transfer_charts(rows)
    if any(r["client"].startswith("rookparcel@") for r in rows):
        blocks["table-versions"] = html(versions_table(rows))
        blocks["charts-versions"] = versions_charts(rows)
    for count in {r["count"] for r in rows if r["scenario"] == "many"}:
        blocks[f"table-{count}"] = html(many_table(rows, count))
        blocks[f"charts-{count}"] = many_charts(rows, count)
    fill_blocks(page, blocks)


def fill_blocks(page, blocks):
    import re
    with open(page, encoding="utf-8") as f:
        text = f.read()
    for name, block in blocks.items():
        text, n = re.subn(rf"(<!-- bench:{name} -->).*?(<!-- /bench:{name} -->)",
                          lambda m: f"{m[1]}\n{block}\n{m[2]}", text, flags=re.S)
        if n != 1:
            print(f"{page}: no bench:{name} block", file=sys.stderr)
    with open(page, "w", encoding="utf-8", newline="\n") as f:
        f.write(text)


def charts(rows):
    t = {c: transfer_ok(rows, c) for c in ORDER}
    m = {c: many_ok(rows, c, 10000) for c in ORDER}
    tn, mn = runs_note(map(len, t.values())), runs_note(map(len, m.values()))

    def tm(key):
        return [(c, *stat(t[c], key)) for c in ORDER]

    def mm(key):
        return [(c, *stat(m[c], key)) for c in ORDER]
    return "\n\n".join([
        bars("Memory with 10,000 torrents, after a restart", f"RSS, {mn}lower is better",
             mm("restarted_rss_mb"), "MB"),
        bars("Memory right after adding 10,000 torrents", f"RSS, {mn}lower is better", mm("added_rss_mb"), "MB"),
        bars("Start until all 10,000 are listed", f"seconds, {mn}lower is better", mm("restart_s"), "s", 2),
        bars("Download speed from one peer", f"MiB/s, {tn}higher is better", tm("mib_per_s"), "MiB/s"),
        bars("CPU time per GiB downloaded", f"seconds, {tn}lower is better", tm("cpu_s_per_gib"), "s", 2),
    ])


def main():
    rows = load(sys.argv[1])
    if "--fill-add" in sys.argv:
        # a run that measured only adding, into its own block: --fill-add PAGE BLOCK
        i = sys.argv.index("--fill-add")
        fill_blocks(sys.argv[i + 1], {sys.argv[i + 2]: add_charts(rows)})
        return
    if "--fill" in sys.argv:
        fill(sys.argv[sys.argv.index("--fill") + 1], rows)
        return
    if "--bars" in sys.argv:
        print(charts(rows))
        return
    render = html if "--html" in sys.argv else markdown
    if "--versions" in sys.argv:
        print(render(versions_table(rows)))
        return
    print("## Transfer\n")
    print(render(transfer_table(rows)))
    for count in sorted({r["count"] for r in rows if r["scenario"] == "many"}):
        print(f"\n## {count:,} torrents\n")
        print(render(many_table(rows, count)))


if __name__ == "__main__":
    main()
