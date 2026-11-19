#!/bin/bash
# Runs as `bench` after build.sh, detached (it takes hours), the published set: `main` against
# the other clients, each built tag as a same-session control. Results: ~/bench/results.jsonl;
# progress and the closing "ALL DONE" line in the log.
#
#     nohup setsid bash run.sh 1.2.69 > ~/bench/run.log 2>&1 < /dev/null &
set -u
cd ~/src
P=~/.venv-linux/bin/python
OUT=~/bench/results.jsonl
A=(--rookparcel ~/bench/pa/main/rookparcel-nox)
for label in "$@"; do A+=(--control "$label=$HOME/bench/pa/$label/rookparcel-nox"); done
echo "START $(date -Is)"
$P tools/bench/bench.py transfer --mib 2048 --runs 5 --out $OUT "${A[@]}"; echo "transfer exit $? $(date -Is)"
$P tools/bench/bench.py many --count 1000 --runs 3 --out $OUT "${A[@]}"; echo "many 1000 exit $? $(date -Is)"
$P tools/bench/bench.py many --count 10000 --runs 3 --out $OUT "${A[@]}"; echo "many 10000 exit $? $(date -Is)"
echo "ALL DONE $(date -Is)"
