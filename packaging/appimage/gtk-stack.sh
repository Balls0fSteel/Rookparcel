#!/bin/bash
# GTK 4.14 and what Ubuntu 22.04 has too old for it (GLib 2.80, Wayland 1.22, its protocols), for
# the AppImages (packaging/appimage/README.md). Runs as root in a throwaway ubuntu:22.04
# container, on x86_64 or aarch64, after deps.sh. Each library is installed for /usr through
# $STAGE and then laid over the container's own /usr, so pkg-config, linuxdeploy and its GTK
# plugin find it at the system's paths. With a complete $STAGE (the release job caches it by this
# file's hash) it only lays it over again.
set -euo pipefail

STAGE=${STAGE:-/opt/rp-gtk-stage}
WORK=${WORK:-/tmp/rp-gtk-src}
LIBDIR=lib/$(dpkg-architecture -qDEB_HOST_MULTIARCH)

# name version url sha256
SOURCES=(
  "glib 2.80.5 https://download.gnome.org/sources/glib/2.80/glib-2.80.5.tar.xz 9f23a9de803c695bbfde7e37d6626b18b9a83869689dd79019bf3ae66c3e6771"
  "wayland 1.22.0 https://gitlab.freedesktop.org/wayland/wayland/-/releases/1.22.0/downloads/wayland-1.22.0.tar.xz 1540af1ea698a471c2d8e9d288332c7e0fd360c8f1d12936ebb7e7cbc2425842"
  "wayland-protocols 1.36 https://gitlab.freedesktop.org/wayland/wayland-protocols/-/releases/1.36/downloads/wayland-protocols-1.36.tar.xz 71fd4de05e79f9a1ca559fac30c1f8365fa10346422f9fe795f74d77b9ef7e92"
  "gtk 4.14.5 https://download.gnome.org/sources/gtk/4.14/gtk-4.14.5.tar.xz 5547f2b9f006b133993e070b87c17804e051efda3913feaca1108fa2be41e24d"
)

# lays $STAGE over /; 22.04's own GLib 2.72 files go, so linuxdeploy's GTK plugin, which copies
# every libgio-*.so* it finds, bundles only the new ones
overlay() {
  cp -a "$STAGE/." /
  rm -f /usr/"$LIBDIR"/lib{glib,gio,gobject,gmodule,gthread}-2.0.so.0.7200.*
  ldconfig
}

if [ -f "$STAGE/.complete" ]; then
  echo "gtk-stack: laying the cached $STAGE over /"
  overlay
  pkg-config --modversion glib-2.0 gtk4 wayland-client
  exit 0
fi

rm -rf "$STAGE"
mkdir -p "$WORK" "$STAGE"
cd "$WORK"

fetch() { # fetch NAME VERSION URL SHA256: the unpacked source folder's path
  local name=$1 version=$2 url=$3 sum=$4 file
  file=$(basename "$url")
  [ -f "$file" ] || curl -fsSL --retry 3 -o "$file" "$url"
  if [[ "$sum" == SHA_* ]]; then
    echo "$name: sha256 $(sha256sum "$file" | cut -d' ' -f1) (not pinned yet)" >&2
  else
    echo "$sum  $file" | sha256sum -c - >&2
  fi
  rm -rf "$name-$version"
  tar xf "$file"
  echo "$WORK/$name-$version"
}

build() { # build SRC MESON-OPTIONS...
  local src=$1; shift
  meson setup "$src/_build" "$src" --prefix=/usr --libdir="$LIBDIR" --buildtype=release \
    -Dwrap_mode=nofallback "$@"
  meson compile -C "$src/_build"
  meson install -C "$src/_build" --destdir "$STAGE"
  overlay
}

for s in "${SOURCES[@]}"; do
  read -r name version url sum <<< "$s"
  src=$(fetch "$name" "$version" "$url" "$sum")
  case $name in
    glib) build "$src" -Dtests=false -Dintrospection=disabled -Dnls=disabled -Dselinux=disabled \
            -Dlibmount=disabled -Dman-pages=disabled -Ddocumentation=false -Dsysprof=disabled ;;
    wayland) build "$src" -Dtests=false -Ddocumentation=false -Ddtd_validation=false ;;
    wayland-protocols) build "$src" -Dtests=false ;;
    gtk) build "$src" -Dx11-backend=true -Dwayland-backend=true -Dbroadway-backend=false \
           -Dmedia-gstreamer=disabled -Dprint-cpdb=disabled -Dprint-cups=disabled \
           -Dvulkan=disabled -Dcloudproviders=disabled -Dsysprof=disabled -Dtracker=disabled \
           -Dcolord=disabled -Df16c=disabled -Dintrospection=disabled -Ddocumentation=false \
           -Dman-pages=false -Dscreenshots=false -Dbuild-demos=false -Dbuild-testsuite=false \
           -Dbuild-examples=false -Dbuild-tests=false ;;
  esac
done
touch "$STAGE/.complete"
glib-compile-schemas /usr/share/glib-2.0/schemas
pkg-config --modversion glib-2.0 gtk4 wayland-client
