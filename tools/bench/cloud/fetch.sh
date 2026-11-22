#!/bin/bash
# Runs as `bench` after setup.sh, in place of build.sh: the published Linux releases from GitHub,
# each checked against its release's SHA256SUMS. The first version is the one measured
# (~/bench/pa/main/rookparcel-nox, as build.sh puts `main`), the others are the controls
# (~/bench/pa/<version>/rookparcel-nox). Releases up to 1.2.0 were called PaTorrent; their
# patorrent-nox is copied under the same name. Also the source (for bench.py) and its venv.
#
#     fetch.sh 1.2.76 1.2.69 1.2.46 1.2.29 1.1.40 1.0.1
set -eux
cd ~
[ -d ~/src ] || git clone -q https://github.com/Balls0fSteel/Rookparcel.git ~/src
(cd ~/src && git fetch -q --tags origin && git checkout -q main && git pull -q --ff-only &&
 git log -1 --format='bench.py from main at %h %s')
first=1
for v in "$@"; do
    label=$v; [ $first = 1 ] && label=main; first=0
    d=~/dl/$v; mkdir -p "$d"; cd "$d"
    base=https://github.com/Balls0fSteel/Rookparcel/releases/download/v$v
    tarball=
    for name in rookparcel patorrent; do
        if curl -fsSLO "$base/$name-$v-linux-x86_64.tar.gz"; then tarball=$name-$v-linux-x86_64.tar.gz; break; fi
    done
    [ -n "$tarball" ] || { echo "no Linux tarball for $v"; exit 1; }
    curl -fsSLO "$base/SHA256SUMS"
    grep " \*\?$tarball\$" SHA256SUMS | sha256sum -c -
    tar xzf "$tarball"
    nox=$(find . -type f \( -name rookparcel-nox -o -name patorrent-nox \) | head -1)
    [ -n "$nox" ] || { echo "no nox binary in $tarball"; exit 1; }
    mkdir -p ~/bench/pa/$label
    install -m 755 "$nox" ~/bench/pa/$label/rookparcel-nox
    ~/bench/pa/$label/rookparcel-nox --version
    cd ~
done
[ -d ~/.venv-linux ] || python3 -m venv ~/.venv-linux
~/.venv-linux/bin/pip install -q deluge-client
echo FETCH DONE
