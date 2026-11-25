#!/bin/bash
# Starts a GTK AppImage on an Xvfb display, as the AppImage catalog's test does, and fails unless
# a window shows within 30 s; with a second argument, saves a picture of the screen there.
# Needs Xvfb and xdotool (and ImageMagick's `import` for the picture), plus what linuxdeploy
# leaves to every desktop: libx11-6 libxcb1 libfontconfig1 libfreetype6 libfribidi0
# libharfbuzz0b libwayland-client0 (GTK 4.14 needs nothing from libwayland-client that 22.04's
# 1.20 lacks; libwayland-egl 1.22 is bundled). Runs the AppImage unpacked, so no FUSE is needed.
# The home folder is a scratch one.
set -euo pipefail
appimage=$(readlink -f "${1:?usage: smoke.sh APPIMAGE [PNG]}")
png=${2:+$(readlink -f "$2")}
work=$(mktemp -d)
cd "$work"
"$appimage" --appimage-extract >/dev/null
display=:$((90 + RANDOM % 9))
Xvfb "$display" -screen 0 1280x800x24 >/dev/null 2>&1 &
xvfb=$!
# (each step `|| true`: under set -e a failing step here would become the script's exit status)
trap 'kill $xvfb 2>/dev/null || true; pkill -f "$work/squashfs-root" 2>/dev/null || true; rm -rf "$work" || true' EXIT
sleep 2
mkdir -p home
DISPLAY=$display HOME=$work/home XDG_CONFIG_HOME=$work/home/.config \
  XDG_DATA_HOME=$work/home/.local/share XDG_CACHE_HOME=$work/home/.cache \
  "$work/squashfs-root/AppRun" >"$work/app.log" 2>&1 &
app=$!
window=""
for _ in $(seq 1 30); do
  sleep 1
  kill -0 $app 2>/dev/null || break
  window=$(DISPLAY=$display xdotool search --onlyvisible --name . 2>/dev/null | head -n 1 || true)
  [ -n "$window" ] && break
done
if [ -z "$window" ]; then
  echo "smoke: no window; the program's output:"
  cat "$work/app.log"
  exit 1
fi
sleep 3
echo "smoke: a window: $(DISPLAY=$display xdotool getwindowname "$window")"
[ -z "$png" ] || DISPLAY=$display import -window root "$png"
grep -iE 'critical|error|warning' "$work/app.log" | head -n 20 || true
kill $app 2>/dev/null || true
