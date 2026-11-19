#!/bin/bash
# Prepares a fresh Ubuntu 26.04 machine (a rented VPS) for tools/bench/bench.py. Runs as root:
# the other clients from Ubuntu's packages with their services off, a passwordless-sudo user
# `bench` (bench.py runs Transmission through `sudo -n`), and the two tmpfs mounts, which do not
# survive a reboot. See docs/BENCHMARKS.md, "Running it on a rented machine".
set -eux
cloud-init status --wait || true
export DEBIAN_FRONTEND=noninteractive
apt-get update -q
apt-get install -y -q build-essential pkg-config libssl-dev git python3-venv curl \
    qbittorrent-nox transmission-daemon deluged deluge-console rtorrent
# bench.py starts every client itself
for s in transmission-daemon deluged qbittorrent-nox; do systemctl disable --now "$s" 2>/dev/null || true; done
id bench >/dev/null 2>&1 || useradd -m -s /bin/bash bench
echo 'bench ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/bench
chmod 440 /etc/sudoers.d/bench
mkdir -p /home/bench/.ssh
cp /root/.ssh/authorized_keys /home/bench/.ssh/
chown -R bench:bench /home/bench/.ssh
chmod 700 /home/bench/.ssh
mkdir -p /mnt/benchram /var/lib/transmission-daemon/downloads
mountpoint -q /mnt/benchram || mount -t tmpfs -o size=6g,uid=$(id -u bench),gid=$(id -g bench) tmpfs /mnt/benchram
mountpoint -q /var/lib/transmission-daemon/downloads || mount -t tmpfs \
    -o size=6g,uid=debian-transmission,gid=debian-transmission,mode=0755 tmpfs /var/lib/transmission-daemon/downloads
nproc; free -m; lscpu | grep -E 'Model name|Thread|Core|Socket|Hypervisor'
qbittorrent-nox --version; transmission-daemon --version 2>&1 | head -1; deluged --version | head -1; rtorrent -h | head -1
echo SETUP DONE
