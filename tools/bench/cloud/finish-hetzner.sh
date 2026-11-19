#!/bin/bash
# Installed as /usr/local/sbin/bench-finish on a Hetzner Cloud bench server. Uploads the run's
# files to a Storage Box over WebDAV, then deletes the server's SSH key and the server itself, so
# a run costs nothing once it is over, whether or not anyone is watching.
#
#     bench-finish done        # run.sh, at its end: upload; delete only if the upload worked
#     bench-finish deadline    # the systemd timer armed at setup: upload what there is, delete
#
# Settings in /etc/bench-finish.env (root only, written over SSH, never in cloud-init data):
# HCLOUD_TOKEN, STORAGE_HOST, STORAGE_USER, STORAGE_PASS, RUN_NAME, SSH_KEY_ID.
set -u
reason=${1:-done}
exec 9>/run/bench-finish.lock
flock -n 9 || { echo "bench-finish already running"; exit 0; }
. /etc/bench-finish.env
log=/var/log/bench-finish.log
exec >>"$log" 2>&1
echo "== $(date -Is) bench-finish $reason"

B=/home/bench
dav="https://$STORAGE_HOST/$RUN_NAME"
auth=(--user "$STORAGE_USER:$STORAGE_PASS")
echo "$reason $(date -Is)" > /tmp/finish-reason.txt
uploaded=1
curl -sS -f -o /dev/null "${auth[@]}" -X MKCOL "$dav/" || true
for f in $B/bench/results.jsonl $B/bench/run.log $B/bench/machine.txt $B/build.log /root/setup.log \
         /tmp/finish-reason.txt "$log"; do
    [ -f "$f" ] || continue
    ok=0
    for try in 1 2 3 4 5; do
        if curl -sS -f -o /dev/null "${auth[@]}" -T "$f" "$dav/$(basename "$f")"; then ok=1; break; fi
        sleep $((try * 10))
    done
    echo "upload $f: $ok"
    [ $ok = 1 ] || uploaded=0
done

if [ "$reason" = done ] && [ $uploaded = 0 ]; then
    echo "upload failed: the server stays up for a copy by hand until its deadline"
    exit 1
fi

api() { curl -sS -H "Authorization: Bearer $HCLOUD_TOKEN" "$@"; }
[ -n "${SSH_KEY_ID:-}" ] && api -X DELETE "https://api.hetzner.cloud/v1/ssh_keys/$SSH_KEY_ID"
id=$(curl -sS http://169.254.169.254/hetzner/v1/metadata/instance-id)
echo "deleting server $id"
for try in 1 2 3 4 5 6 7 8 9 10; do
    code=$(api -o /dev/null -w '%{http_code}' -X DELETE "https://api.hetzner.cloud/v1/servers/$id")
    echo "delete: HTTP $code"
    case $code in 2*) exit 0 ;; esac
    sleep 30
done
