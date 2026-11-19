#!/bin/bash
# Runs as `bench` after build.sh, detached (it takes hours), the published set: `main` against
# the other clients, each built tag as a same-session control. Results: ~/bench/results.jsonl;
# progress and the closing "ALL DONE" line in the log; the machine in ~/bench/machine.txt.
# On a Hetzner server armed with finish-hetzner.sh, the end uploads the files and deletes the
# server; `run.sh --finish` does only that step (launch-hetzner.ps1 -SelfTest calls it).
#
#     nohup setsid bash run.sh 1.2.69 > ~/bench/run.log 2>&1 < /dev/null &
set -u

# bench-finish is root's alone (mode 700, it reads the root-only token file), so `bench` asks
# sudo whether it is there: a plain `[ -x ]` is false for this user and skipped it (2026-11-19).
finish() {
    if sudo -n test -x /usr/local/sbin/bench-finish; then
        sudo -n /usr/local/sbin/bench-finish done
        echo "bench-finish exit $? $(date -Is)"
    fi
}
if [ "${1:-}" = --finish ]; then finish; exit; fi

cd ~/src
P=~/.venv-linux/bin/python
OUT=~/bench/results.jsonl
A=(--rookparcel ~/bench/pa/main/rookparcel-nox)
for label in "$@"; do A+=(--control "$label=$HOME/bench/pa/$label/rookparcel-nox"); done
{ lscpu; echo; free -m; echo; uname -a; cat /etc/os-release; echo; git log -1 --format='main at %H'
  for b in ~/bench/pa/*/rookparcel-nox; do "$b" --version; done; } > ~/bench/machine.txt 2>&1
echo "START $(date -Is)"
$P tools/bench/bench.py transfer --mib 2048 --runs 5 --out $OUT "${A[@]}"; echo "transfer exit $? $(date -Is)"
$P tools/bench/bench.py many --count 1000 --runs 3 --out $OUT "${A[@]}"; echo "many 1000 exit $? $(date -Is)"
$P tools/bench/bench.py many --count 10000 --runs 3 --out $OUT "${A[@]}"; echo "many 10000 exit $? $(date -Is)"
echo "ALL DONE $(date -Is)"
finish
