#!/bin/bash
# The build tools and libraries gtk-stack.sh and build.sh need, in an ubuntu:22.04 container
# (as root). Meson comes from pip: 22.04's 0.61 is too old for GLib 2.80.
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq --no-install-recommends \
  build-essential pkg-config ninja-build python3-pip git curl ca-certificates xz-utils file \
  libffi-dev libpcre2-dev zlib1g-dev libexpat1-dev libxml2-dev \
  libcairo2-dev libpango1.0-dev libharfbuzz-dev libfribidi-dev libgdk-pixbuf-2.0-dev \
  libepoxy-dev libgraphene-1.0-dev libxkbcommon-dev libx11-dev libxrandr-dev libxi-dev \
  libxcursor-dev libxdamage-dev libxfixes-dev libxinerama-dev libxext-dev libfontconfig1-dev \
  libpng-dev libjpeg-dev libtiff-dev libegl-dev libgl-dev libdrm-dev sassc libssl-dev dpkg-dev \
  libgdk-pixbuf2.0-bin librsvg2-common shared-mime-info >/dev/null
pip3 install -q 'meson==1.5.2' 'packaging==24.1'
