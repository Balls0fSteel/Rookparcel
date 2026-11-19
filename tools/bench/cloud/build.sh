#!/bin/bash
# Runs as `bench` after setup.sh: Rust, the source, and rookparcel-nox built at `main` and at each
# control tag, with the release workflow's command, into ~/bench/pa/<label>/rookparcel-nox
# (`main` for the branch, the tag without its `v` for the others), and the venv for bench.py.
#
#     build.sh v1.2.69 [v1.2.46 ...]
set -eux
cd ~
[ -x ~/.cargo/bin/cargo ] || curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
. ~/.cargo/env
rustc --version
[ -d ~/src ] || git clone -q https://github.com/Balls0fSteel/Rookparcel.git ~/src
cd ~/src
git fetch -q --tags origin
git checkout -q main && git pull -q --ff-only
git log -1 --format='main at %h %s'
build() {  # label, source dir
    (cd "$2" && CARGO_TARGET_DIR=~/target-$1 cargo build -q --release --locked -p rookparcel-nox)
    mkdir -p ~/bench/pa/$1
    cp ~/target-$1/release/rookparcel-nox ~/bench/pa/$1/
    ~/bench/pa/$1/rookparcel-nox --version
}
build main ~/src
for tag in "$@"; do
    label=${tag#v}
    [ -d ~/tags/$label ] || git worktree add -q --detach ~/tags/$label "$tag"
    build "$label" ~/tags/$label
done
[ -d ~/.venv-linux ] || python3 -m venv ~/.venv-linux
~/.venv-linux/bin/pip install -q deluge-client
echo BUILD DONE
