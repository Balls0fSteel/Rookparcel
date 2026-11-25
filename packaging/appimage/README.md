# AppImages

The release job's `appimage` job makes `Rookparcel-<version>-<arch>.AppImage` (the GTK GUI) and
`rookparcel-nox-<version>-<arch>.AppImage` in an `ubuntu:22.04` container, natively on x86_64 and
aarch64, so they need glibc 2.35 (the GUI's, for the bundled Cairo of 22.04; nox 2.34) and run on Ubuntu 22.04, Debian 12 and newer (D81). The
AppImage catalog (appimage.github.io) starts each app on 22.04.

| Script | What |
|---|---|
| `deps.sh` | the build tools and `-dev` packages (as root); Meson from pip, as 22.04's is too old for GLib 2.80 |
| `gtk-stack.sh` | GLib 2.80.5, Wayland 1.22.0, wayland-protocols 1.36 and GTK 4.14.5 from checksummed tarballs, installed for `/usr` through `/opt/rp-gtk-stage` and laid over the container's `/usr` (22.04's GLib 2.72 files removed); a complete stage (the job's cache) is only laid over again |
| `build.sh` | `cargo build --release` of both programs (checks they need no glibc past 2.35), linuxdeploy with its GTK plugin, the bundled GTK and GLib versions, nox's `--version` |
| `smoke.sh APPIMAGE [PNG]` | starts the GTK AppImage on Xvfb with a scratch home and fails without a window in 30 s; job `appimage-start` runs it in a clean 22.04 container |

GTK is built without media (GStreamer), printing, Vulkan, introspection and the Broadway backend,
which the GUI does not use; X11 and Wayland are on. Its 22.04 dependencies (Pango 1.50, Cairo,
HarfBuzz, gdk-pixbuf, Graphene, libepoxy) are new enough as they are.

A local run, on the Xubuntu test VM or any machine with Docker, from the repository root:

```bash
docker run --rm -v "$PWD:/w" -w /w ubuntu:22.04 bash -c '
  bash packaging/appimage/deps.sh && bash packaging/appimage/gtk-stack.sh &&
  VERSION=0.0.0 APP_ICON_DIR=assets/patina-icons/exports/app/$(tr -d "[:space:]" < assets/patina-icons/app-icon.txt) \
    bash packaging/appimage/build.sh'
```

The stack takes about 10 minutes to build; keep the container (or `/opt/rp-gtk-stage`) to reuse it.
To move to a newer GTK, change the version, URL and checksum in `gtk-stack.sh` (GNOME publishes a
`.sha256sum` beside each tarball) and check that its other requirements are still met on 22.04.
