#!/bin/bash
# Builds rookparcel-gtk and rookparcel-nox and makes the two AppImages, in the ubuntu:22.04
# container after deps.sh and gtk-stack.sh (packaging/appimage/README.md), so they need glibc
# 2.35 and run on Ubuntu 22.04, Debian 12, RHEL 9 and newer. Run from the repository root:
#   VERSION=1.2.81 APP_ICON_DIR=assets/patina-icons/exports/app/<family> packaging/appimage/build.sh
# The AppImages land in the current folder.
set -euo pipefail
: "${VERSION:?}" "${APP_ICON_DIR:?}"
arch=$(uname -m)
app_id=io.github.balls0fsteel.Rookparcel

# pinned linuxdeploy releases and the GTK plugin's commit (no `continuous` builds)
LINUXDEPLOY=https://github.com/linuxdeploy/linuxdeploy/releases/download/1-alpha-20251107-1
LINUXDEPLOY_APPIMAGE=https://github.com/linuxdeploy/linuxdeploy-plugin-appimage/releases/download/1-alpha-20250213-1
LINUXDEPLOY_GTK=https://raw.githubusercontent.com/linuxdeploy/linuxdeploy-plugin-gtk/7a3fbc31a9e5075073ff8790f26effbac5f84453/linuxdeploy-plugin-gtk.sh
# containers have no FUSE: linuxdeploy's own AppImages unpack themselves to run
export APPIMAGE_EXTRACT_AND_RUN=1

if ! command -v cargo >/dev/null; then
  curl -fsSL --retry 3 https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
fi
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release --locked -p rookparcel-nox -p rookparcel-gtk
# the point of this build: nothing newer than 22.04's glibc 2.35
for b in target/release/rookparcel-gtk target/release/rookparcel-nox; do
  need=$(objdump -T "$b" | grep -o 'GLIBC_[0-9.]*' | sort -uV | tail -n 1)
  echo "$b needs $need"
  [ "$(printf '%s\n' "$need" GLIBC_2.35 | sort -V | tail -n 1)" = GLIBC_2.35 ]
done

tools=${RUNNER_TEMP:-/tmp}/linuxdeploy
mkdir -p "$tools"
curl -fsSL --retry 3 -o "$tools/linuxdeploy" "$LINUXDEPLOY/linuxdeploy-$arch.AppImage"
curl -fsSL --retry 3 -o "$tools/linuxdeploy-plugin-appimage" "$LINUXDEPLOY_APPIMAGE/linuxdeploy-plugin-appimage-$arch.AppImage"
curl -fsSL --retry 3 -o "$tools/linuxdeploy-plugin-gtk.sh" "$LINUXDEPLOY_GTK"
chmod +x "$tools"/*
export PATH="$tools:$PATH" LINUXDEPLOY_OUTPUT_VERSION="$VERSION"
icons=${RUNNER_TEMP:-/tmp}/icons
mkdir -p "$icons"
cp "$APP_ICON_DIR/256.png" "$icons/$app_id.png"
cp "$APP_ICON_DIR/256.png" "$icons/$app_id.Nox.png"
rm -rf AppDir-gtk AppDir-nox

# the GTK GUI, GTK 4 bundled (the plugin sets GTK's module, schema and loader paths). Its
# hook's `GDK_BACKEND=x11` goes: GTK 4 runs on Wayland (checked under GNOME 50), and forced X11
# needs Xwayland
DEPLOY_GTK_VERSION=4 linuxdeploy \
  --appdir AppDir-gtk \
  --executable target/release/rookparcel-gtk \
  --desktop-file "contrib/linux/$app_id.desktop" \
  --icon-file "$icons/$app_id.png" \
  --plugin gtk
hook=AppDir-gtk/apprun-hooks/linuxdeploy-plugin-gtk.sh
grep -q '^export GDK_BACKEND=x11' "$hook"
sed -i '/^export GDK_BACKEND=x11/d' "$hook"
LDAI_OUTPUT="Rookparcel-$VERSION-$arch.AppImage" linuxdeploy --appdir AppDir-gtk --output appimage

# nox: a terminal program, kept out of the menus; OpenSSL bundled
cat > "${RUNNER_TEMP:-/tmp}/$app_id.Nox.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Rookparcel nox
Comment=Headless BitTorrent daemon with a Web UI
Exec=rookparcel-nox
Icon=$app_id.Nox
Terminal=true
NoDisplay=true
Categories=Network;FileTransfer;P2P;
EOF
LDAI_OUTPUT="rookparcel-nox-$VERSION-$arch.AppImage" linuxdeploy \
  --appdir AppDir-nox \
  --executable target/release/rookparcel-nox \
  --desktop-file "${RUNNER_TEMP:-/tmp}/$app_id.Nox.desktop" \
  --icon-file "$icons/$app_id.Nox.png" \
  --output appimage

# the libraries are inside (the container has them too, so ldd alone proves nothing): GTK 4.14
# and GLib 2.80 from gtk-stack.sh, asked for their versions; OpenSSL for nox; and the nox bundle
# starts
python3 - <<'PY'
import ctypes
gtk = ctypes.CDLL("AppDir-gtk/usr/lib/libgtk-4.so.1")
glib = ctypes.CDLL("AppDir-gtk/usr/lib/libglib-2.0.so.0")
v = (gtk.gtk_get_major_version(), gtk.gtk_get_minor_version(), gtk.gtk_get_micro_version())
g = tuple(ctypes.c_uint.in_dll(glib, n).value for n in ("glib_major_version", "glib_minor_version"))
print("bundled: GTK %d.%d.%d, GLib %d.%d" % (v + g))
assert v >= (4, 14, 0) and g >= (2, 80), "an older GTK or GLib was bundled"
PY
ls AppDir-nox/usr/lib/libssl.so.* AppDir-nox/usr/lib/libcrypto.so.*
./"rookparcel-nox-$VERSION-$arch.AppImage" --version
ls -l ./*.AppImage
