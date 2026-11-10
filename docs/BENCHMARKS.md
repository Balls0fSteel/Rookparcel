# Benchmarks against other clients

Rookparcel's daemon (`rookparcel-nox`) measured beside the daemons of qBittorrent, Transmission,
Deluge and rTorrent, on the same machine with the same torrents. The website shows the results
([benchmarks.html](../site/benchmarks.html)); this file is the method, so anyone can run it again.
The harness is `tools/bench/bench.py` (Python 3, standard library; Deluge's RPC needs
`pip install deluge-client`) and `tools/bench/summarize.py` turns its JSON lines into tables.

## Results, 2026-11-09 (Rookparcel 1.2.29)

Run on the VMware VM described under Limits, again with 8 GB, in the night of 2026-11-09 to 2026-11-10; raw results in `docs/benchmarks/2026-11-09-ubuntu-vm-1.2.29.jsonl`. Transfer: 2 GiB, 5 runs each, medians with the range; throughput and CPU between 10% and 80% of the payload. Many torrents: 3 runs per count, medians; memory and idle CPU after the restart. The other clients are the same packages as on 2026-10-27 and measure the same as then, which says the VM does too.

- Deluge finished two of its five transfer runs (run 3: its daemon did not answer the harness, `FailedToReconnectException`; runs 4 and 5: no byte in 600 s; cause not found). Five more runs afterwards in a session of its own, beside qBittorrent, all finished; they are in the file as runs 6 to 10 with a `note`, and the table's medians are over the seven good runs.
- rTorrent still slows to about 32 MB/s once some 1.7 GiB of memory-mapped pieces wait to be flushed; Transmission still holds 62.4 MiB/s from one peer.
- Right after adding 10,000, Rookparcel used 318 MB (qBittorrent 636, Transmission 328, Deluge 883, rTorrent 2,102).

### Transfer

| Client | Version | Runs | Throughput (MiB/s) | range | CPU s per GiB | range | First byte to done (s) |
|---|---|---|---|---|---|---|---|
| Rookparcel | rookparcel-nox 1.2.29 | 5 of 5 | 3,154 | 2,124–3,539 | 0.99 | 0.88–1.02 | 1.6 |
| qBittorrent | v5.1.4 | 5 of 5 | 1,475 | 1,398–1,667 | 1.96 | 1.87–2.00 | 2.3 |
| Transmission | Transmission 4.1.1 (56442e2929) | 5 of 5 | 62.4 | 62.3–62.4 | 2.77 | 2.72–2.80 | 41.5 |
| Deluge | Deluge 2.2.1.dev0+20250824 (libtorrent 2.0.12.0) | 7 of 10 | 1,471 | 1,268–1,890 | 1.98 | 1.82–2.04 | 2.0 |
| rTorrent | rTorrent 0.9.8 (libtorrent 0.13.8) | 5 of 5 | 1,769 | 1,216–2,033 | 0.89 | 0.81–1.06 | 127 |

### 1,000 torrents

| Client | Add (s) | Start with all listed (s) | Memory, RSS (MB) | PSS (MB) | Idle CPU (%) | Empty RSS (MB) |
|---|---|---|---|---|---|---|
| Rookparcel | 0.1 | 0.07 | 21.4 | 14.2 | 0.08 | 15.8 |
| qBittorrent | 0.1 | 0.18 | 105 | 89.4 | 0.08 | 53.7 |
| Transmission | 0.5 | 0.10 | 52.4 | 39.8 | 0.23 | 20.8 |
| Deluge | 51.7 | 0.83 | 135 | 120 | 0.33 | 68.4 |
| rTorrent | 1.1 | 0.56 | 230 | 219 | 0.13 | 16.4 |

### 10,000 torrents

| Client | Add (s) | Start with all listed (s) | Memory, RSS (MB) | PSS (MB) | Idle CPU (%) | Empty RSS (MB) |
|---|---|---|---|---|---|---|
| Rookparcel | 1.3 | 0.20 | 67.0 | 59.7 | 0.33 | 15.6 |
| qBittorrent | 1.2 | 0.86 | 612 | 596 | 0.10 | 53.4 |
| Transmission | 5.1 | 1.00 | 327 | 314 | 1.10 | 20.7 |
| Deluge | 517 | 6.02 | 701 | 686 | 0.30 | 68.4 |
| rTorrent | 25.2 | 21.44 | 2,142 | 2,132 | 1.55 | 16.4 |

### Rookparcel 1.0.1, 1.1.40 and 1.2.29 in the same rounds

The published 1.0.1 and 1.1.40 Linux releases ran as `--control` entries beside 1.2.29: the same seed, the same rounds, the first client rotating. Medians with the range.

| Measure | 1.0.1 | 1.1.40 | 1.2.29 |
|---|---|---|---|
| Download, MiB/s (median of 5, range) | 3,160 (3,145–3,187) | 3,156 (3,139–3,169) | 3,154 (2,124–3,539) |
| Download, CPU s per GiB | 0.96 (0.94–0.99) | 0.98 (0.95–0.99) | 0.99 (0.88–1.02) |
| 1,000 torrents: add (s) | 0.12 (0.11–0.13) | 0.11 (0.11–0.12) | 0.12 (0.11–0.12) |
| 1,000 torrents: start with all listed (s) | 0.07 (0.07–0.07) | 0.07 (0.07–0.07) | 0.07 (0.07–0.07) |
| 1,000 torrents: memory after a restart (MB) | 15.0 (14.9–15.0) | 20.9 (20.8–21.0) | 21.4 (21.3–21.5) |
| 1,000 torrents: memory right after adding (MB) | 50.2 (50.1–50.2) | 56.1 (55.9–57.4) | 56.4 (56.4–56.4) |
| 1,000 torrents: idle CPU (%) | 0.10 (0.07–0.10) | 0.08 (0.07–0.12) | 0.08 (0.08–0.12) |
| 10,000 torrents: add (s) | 1.34 (1.06–1.42) | 1.13 (1.09–1.34) | 1.28 (1.07–1.36) |
| 10,000 torrents: start with all listed (s) | 0.20 (0.19–0.20) | 0.19 (0.19–0.20) | 0.20 (0.20–0.20) |
| 10,000 torrents: memory after a restart (MB) | 60.3 (60.2–60.3) | 66.1 (66.1–66.1) | 67.0 (66.9–67.1) |
| 10,000 torrents: memory right after adding (MB) | 319 (309–319) | 316 (305–325) | 318 (315–326) |
| 10,000 torrents: idle CPU (%) | 0.07 (0.07–0.08) | 0.10 (0.07–0.12) | 0.33 (0.25–0.37) |
| Empty session, memory (MB) | 9.5 (9.4–9.5) | 15.4 (15.3–15.4) | 15.6 (15.6–15.8) |

- **Unchanged:** download speed and CPU per GiB, add, start, memory right after adding.
- **Memory:** about 6 MB more since 1.1.17's update check (see the 1.1.40 check below), and 0.5 to 0.9 MB more in 1.2.29 than in 1.1.40.
- **Regression, idle CPU with 10,000 torrents:** 0.33% of one core (0.25 to 0.37) against 0.07% for 1.0.1 and 0.10% for 1.1.40; the ranges do not overlap. Not at 1,000 torrents. Still under the 1% target of `docs/ARCHITECTURE.md`, but above qBittorrent's 0.10% now. Found and fixed in 1.2.30: see "The idle CPU regression" below.
- The first Rookparcel run of the session was the slow one again (2,124 MiB/s, as on 2026-10-27).

## Engine and GUIs, 2026-11-10 (1.2.29 against 1.0.1)

On the development PC (Windows 11, i9-13900K), nothing else running, release builds; not part of the comparison with other clients.

**Engine alone** (`lt-swarm`, the CI set, 1.0.1's and 1.2.29's builds alternating, three rounds): no difference. 10,000 simulated torrents update in 8-9 ms; 256 MiB over loopback at 298-315 MB/s (one 1.2.29 round at 187); 5,000 torrents parsed in 104-113 ms (one cold 1.0.1 round at 163); 1,000 saved torrents restored in 14-22 ms (1.0.1: 25-39 ms). `load --count 100000 --idle 60`, once each: restored in 1,318 ms (1.0.1: 1,199 ms), 184 MB (183), idle 0.00% (0.03%).

**Win32 GUI, 100,000 saved torrents** (`lt-swarm load --count 100000 --keep`, a portable copy, `ROOKPARCEL_BENCH=1` on the hidden desktop, three runs each; 1.0.1 is the published zip on state its own `lt-swarm` wrote):

| Measure | 1.0.1 | 1.2.29 |
|---|---|---|
| All rows listed after start (s) | 1.26-1.28 | 1.37-1.38 |
| Idle 60 s, GUI thread (% of a core) | 0.05-0.18 | 0.16-0.18 |
| Idle 60 s, whole process (%) | 0.08-0.36 | 0.81-0.96 |
| Working set after 60 s idle (MB) | 249-252 | 264-269 |
| Sort by Name / Size / Status (ms) | 49-53 / 20 / 57-61 | 32-36 / 18-19 / 56-59 |
| Select All (ms) | 4.3-4.5 | 3.7-3.9 |
| Add a magnet / remove one, until painted (ms) | 17-20 / 15-28 | 16-20 / 12 |
| Remove 1,000, wall (ms) | 510-536 | 463-502 |

- **Regressions:** the process's idle CPU about three times 1.0.1's (off the GUI thread; fixed in 1.2.30, below), the list 0.1 s later, 15 MB more memory.
- **Better:** the Name sort.
- With `--demo 100000` (2,000 torrents moving, three runs): rows after 0.16-0.29 s, sorts 32-48 / 19-23 / 56-61 ms, GUI thread 1.2-1.4%, working set 221-225 MB: as on 2026-10-23.
- While it closes, 1.2.29's working set rose from 270 MB to 358 MB twice and 640 MB once (one sample each, 5 s apart; 1.0.1 not sampled there): to be looked at.

**Mac GUI** (release build on the Intel macOS 15.8 VM, `--demo 100000`, three runs): the list after 0.54-0.57 s (0.48 s on 2026-11-06, one run), 191 MB by `top` (194 MB). The sort was not timed again.

## The idle CPU regression, found and fixed 2026-11-10 (1.2.30)

Two causes, both work that grew with the number of torrents:

- **The daemon after a restart (the 0.33%).** Since 1.1.78 the Web UI asked the engine for every restored torrent's metadata, 128 torrents per status tick, to learn its private flag (`isPrivate` in the list). Each answer read and parsed the info dictionary from `metadata.pack`: about 1.2% of a core on the engine thread for 78 s with 10,000 torrents, of which the idle window (60 to 120 s after the start) caught the end. Found by per-thread CPU in 20 s steps and `strace` (an `lseek` and a `read` of the pack per torrent). A release-by-release run put it between 1.1.40 (0.10%) and 1.2.0 (0.32%); 1.2.5, 1.2.19 and 1.2.21 measured 0.37, 0.28 and 0.22%. Fix: the engine keeps the flag (`TorrentStatus::private`, `lt-private` in resume data, remote protocol 17), so nothing is read. Records written before 1.2.30 are asked once, as before, and the engine saves the answer: the first start after the upgrade still has the pass, the second has not (checked on 1.2.29's records).
- **The engine with many stopped torrents (the Win32 process at 100,000).** `lsd_tick` counted the running torrents on every tick by walking every slot (since the staggered LSD of 2026-10-30, 1.1.x). `perf` on a build with line tables, 100,000 stopped torrents in `rookparcel-nox`: 66% of the engine thread's idle time in that walk, 23 clock ticks in 30 s (0.77% of a core); 1 tick with the fix, which walks the active torrents only. `lt-swarm load --idle` never showed it because it runs without LSD.

Measured with the fix (`docs/benchmarks/2026-11-10-ubuntu-vm-idle-fix-1.2.30.jsonl`; 10,000 torrents, three rounds, medians with the range; the 1.2.30 binary built on the VM from the working tree, with line tables):

| | 1.0.1 | 1.2.29 | 1.2.30 (fix) |
|---|---:|---:|---:|
| Idle CPU after a restart (%) | 0.10 (0.08–0.10) | 0.33 (0.33–0.35) | 0.12 (0.10–0.12) |
| Idle CPU after adding (%) | 0.08 (0.07–0.12) | 0.08 (0.08–0.12) | 0.10 (0.08–0.10) |
| Add (s) | 1.26 | 1.30 | 1.33 |
| Memory after a restart (MB) | 60.3 | 67.1 | 67.4 |

Win32 at 100,000 real torrents on the development PC, two runs: the process idles at 0.36-0.39% (1.2.29: 0.81-0.96%; 1.0.1 the same morning: 0.26%), its engine thread uses 60-95 ms of CPU in the idle minute (1.2.29: about 350 ms; 1.0.1: about 50 ms). The site's tables stay those of the published 1.2.29 until a release with the fix is measured in a full run.

## Earlier results, 2026-10-27 (Rookparcel 1.0.1)

Run on the VMware VM described under Limits; raw results in `docs/benchmarks/2026-10-27-ubuntu-vm.jsonl`. Transfer: 2 GiB, 5 runs each, medians with the range; throughput and CPU between 10% and 80% of the payload; *first byte to done* includes the end of the download (rTorrent slows to about 32 MB/s once some 1.7 GiB of memory-mapped pieces wait to be flushed; Transmission holds 62.4 MiB/s from one peer). Many torrents: one run per count; memory and idle CPU after the restart. Right after adding 10,000, Rookparcel used 319 MB (qBittorrent 636, Transmission 328, Deluge 905, rTorrent 2,101): it keeps metadata only for active torrents, which a restart leaves out.

### Transfer

| Client | Version | Runs | Throughput (MiB/s) | range | CPU s per GiB | range | First byte to done (s) |
|---|---|---|---|---|---|---|---|
| Rookparcel | rookparcel-nox 1.0.1 | 5 of 5 | 3,592 | 2,124–3,605 | 1.04 | 0.94–1.16 | 1.6 |
| qBittorrent | v5.1.4 | 5 of 5 | 1,471 | 1,380–1,662 | 2.14 | 2.06–2.22 | 2.5 |
| Transmission | Transmission 4.1.1 (56442e2929) | 5 of 5 | 62.4 | 62.4–62.5 | 2.66 | 2.66–2.72 | 41.2 |
| Deluge | Deluge 2.2.1.dev0+20250824 (libtorrent 2.0.12.0) | 5 of 5 | 1,566 | 1,217–1,670 | 2.11 | 1.97–2.22 | 2.0 |
| rTorrent | rTorrent 0.9.8 (libtorrent 0.13.8) | 5 of 5 | 1,924 | 1,721–2,056 | 0.85 | 0.81–0.94 | 126 |

### 1,000 torrents

| Client | Add (s) | Start with all listed (s) | Memory, RSS (MB) | PSS (MB) | Idle CPU (%) | Empty RSS (MB) |
|---|---|---|---|---|---|---|
| Rookparcel | 0.1 | 0.07 | 15.1 | 10.9 | 0.05 | 9.5 |
| qBittorrent | 0.1 | 0.18 | 105 | 90.1 | 0.07 | 53.6 |
| Transmission | 0.5 | 0.09 | 52.4 | 40.4 | 0.20 | 20.9 |
| Deluge | 51.6 | 0.91 | 135 | 120 | 0.22 | 68.5 |
| rTorrent | 1.1 | 0.55 | 230 | 219 | 0.10 | 16.5 |

### 10,000 torrents

| Client | Add (s) | Start with all listed (s) | Memory, RSS (MB) | PSS (MB) | Idle CPU (%) | Empty RSS (MB) |
|---|---|---|---|---|---|---|
| Rookparcel | 1.3 | 0.19 | 60.2 | 56.1 | 0.07 | 9.5 |
| qBittorrent | 1.2 | 0.86 | 612 | 597 | 0.07 | 53.5 |
| Transmission | 4.9 | 1.00 | 327 | 315 | 0.77 | 20.8 |
| Deluge | 517 | 5.76 | 701 | 686 | 0.28 | 68.4 |
| rTorrent | 24.7 | 22.62 | 2,142 | 2,132 | 1.25 | 16.4 |

## Release check, 1.1.40 (2026-11-03)

The same VM, now with 4 GB of RAM instead of 8, so the transfer was 1 GiB (the 2 GiB one does not fit in tmpfs twice); Rookparcel and qBittorrent only, with the published 1.0.1 binary run beside 1.1.40 the same day as the control. Raw results: `docs/benchmarks/2026-11-03-ubuntu-vm-4gb-1.1.40.jsonl`. It was not published.

- **Transfer, 1 GiB, 10 runs each, alternating:** 1.1.40 2,123 MiB/s and 1.17 CPU s/GiB, 1.0.1 2,099 MiB/s and 1.23: the same. Both are lower than the published 2 GiB run, and so is qBittorrent (1,256 MiB/s, 2.23 s/GiB): the smaller VM and payload.
- **10,000 torrents, add:** three alternating runs each, 1.0.1 1.37-1.84 s (median 1.68), 1.1.40 1.71-2.16 s (median 1.78): within run-to-run noise. Start with all listed, memory after adding and idle CPU unchanged.
- **Memory after a restart:** 66 MB at 10,000 torrents (60 for 1.0.1), 21 MB at 1,000 (15), 15.5 MB empty (9.5). The difference is `rookparcel-nox`'s update check (since 1.1.17): its one HTTPS request 10-20 s after start adds about 5.5 MB that stays (10.4 MB with `--no-update-check`, measured at 40 s). The smart disk I/O threads add about 0.1 MB.

## What is measured

**Many torrents** (`bench.py many --count N`). Every client gets the same N generated torrents,
the shape of `tools/swarm`'s corpus: 10 MB to 20 GB (log-uniform), about 1,500 pieces each, 40%
single-file and 60% with 2 to 41 files, one tracker that refuses connections, and no data on
disk. They are added started, through each client's own API, in batches where it has them.

- *Add*: from the first add request until the client's API lists all N.
- *Start with all listed*: the client is stopped (SIGTERM, letting it save its state) and
  started again; the time from starting the process until its API lists all N.
- *Memory* after the restart and 60 s of settling: RSS (resident memory) and PSS (RSS with
  shared libraries divided among the processes using them), summed over the client's processes.
- *Idle CPU*: the client's CPU time over the next 60 s, untouched (no API calls), as a percentage
  of one core.
- `--runs N` repeats all of it N times, a different client first each round; the tables show
  medians (one run on 2026-10-27, three since 2026-11-09).

**Transfer** (`bench.py transfer --mib 2048 --runs 5`). One 2 GiB file of random data in one
torrent (1 MiB pieces), on a tmpfs so the disk is not measured. A reference qBittorrent seeds it,
taking TCP only so every client uses the same transport; a tracker in the harness gives each
side the other's address. Each client downloads it with a fresh profile, five times, a
different client first each round.

- The bytes the downloader received from the seed are read from the kernel's per-connection
  counters (`ss -ti`) every 50 ms: the same for every client, where the clients' own APIs lag by
  up to a second.
- *Throughput* and *CPU per GiB* are taken between 10% and 80% of the payload, leaving out the
  announce, the connection and the end of the download.
- Each download ends when the client says every piece is verified; the harness then stops the
  client and compares the file's SHA-1 with the original.

## Fair-play rules

- Every client runs from a throwaway profile with DHT, local peer discovery, peer exchange and
  port mapping off (no outside peers), everything else at its defaults: queueing, caches,
  connection limits and encryption are each client's own. Nothing is tuned for Rookparcel.
- Rookparcel is a published release (`rookparcel-1.2.29-linux-x86_64.tar.gz` for the current
  results; 1.0.1 and 1.1.40 were `patorrent-<version>-linux-x86_64.tar.gz`), each checked
  against its `SHA256SUMS`, not a local build. The other clients are the distribution's packages.
- Ubuntu confines `transmission-daemon` with an AppArmor profile to its service user's folders,
  so the harness runs it as that user (`debian-transmission`), as the packaged service does,
  with its downloads on a tmpfs of its own. The others run as the logged-in user.
- CPU time is user plus system time from `/proc`, summed over the client's processes and
  threads; a `sudo runuser` that only starts a client is left out.
- Results are medians; the range across runs is shown with them.

## Limits

- One machine, a virtual one (VMware Workstation, 8 vCPUs and 8 GB on an Intel i9-13900K host,
  Ubuntu 26.04.1). VMware gives the guest no steal time, so a busy host shows up as extra CPU
  time in the guest; runs are repeated and interleaved for that reason, and a single run should
  not be read on its own.
- Transfers between two processes on one machine show what a client's code costs, not how it
  behaves on the internet: there is no latency, no packet loss and no swarm.
- Because the seed takes TCP only, clients that try uTP first (Rookparcel, qBittorrent, Deluge,
  Transmission) wait for that attempt to fail before connecting; the time to the first byte is
  recorded but not compared.
- The Windows GUI and the GTK GUI are not measured here; `docs/ARCHITECTURE.md` and PLAN.md
  have the 100,000-torrent measurements of the Windows GUI.

## Running it again

On a Linux machine with the clients installed (Ubuntu: `qbittorrent-nox transmission-daemon
deluged deluge-console rtorrent`) and Rookparcel's Linux tarball unpacked (`--rookparcel PATH`
names its `rookparcel-nox`; without it `~/bench/pa/rookparcel-nox`). Add these to each `bench.py`
line below:

```bash
A="--rookparcel ~/bench/pa/rookparcel-1.2.29-linux-x86_64/rookparcel-nox"
sudo mount -t tmpfs -o size=6g,uid=$(id -u),gid=$(id -g) tmpfs /mnt/benchram
sudo mount -t tmpfs -o size=6g,uid=debian-transmission,gid=debian-transmission,mode=0755 \
    tmpfs /var/lib/transmission-daemon/downloads
python3 -m venv ~/.venv-linux && ~/.venv-linux/bin/pip install deluge-client
~/.venv-linux/bin/python tools/bench/bench.py transfer --mib 2048 --runs 5 --out results.jsonl $A
~/.venv-linux/bin/python tools/bench/bench.py many --count 1000 --runs 3 --out results.jsonl $A
~/.venv-linux/bin/python tools/bench/bench.py many --count 10000 --runs 3 --out results.jsonl $A
python3 tools/bench/summarize.py results.jsonl
python3 tools/bench/summarize.py results.jsonl --fill site/benchmarks.html
```

All of it took 4 h 17 min on 2026-11-09 (transfer 45 min, of which 20 were two Deluge timeouts;
1,000 torrents 1.5 h; 10,000 torrents 2 h): most is the two 60 s settle and two 60 s idle
windows per client and run. The two tmpfs mounts do not survive a reboot.

`--control LABEL=PATH` (several allowed) measures another `rookparcel-nox`, an older release, in
the same rounds as client `rookparcel@LABEL`; `summarize.py results.jsonl --versions` puts the
versions side by side, and the published tables leave those rows out. A regression is only
shown by such a control: numbers from another day or another VM size do not compare.

`--clients rookparcel,qbittorrent` picks clients; `--keep` leaves the profiles in `~/bench/work`.
Transmission needs root through `sudo -n` (the AppArmor note above). The raw results of each
published run are in `docs/benchmarks/`.
