"""End-to-end check of 1.2.39-1.2.41 with two real rookparcel-nox processes on loopback.

Seeder S seeds a small file with an awkward name; leecher L has the typed tracker list, the
list from the web and a Run Program line, adds the torrent as a magnet, downloads it, and is
restarted. Both listen on 127.0.0.1 only (web 18111/18112, peers 18121/18122).

    python e2e.py <rookparcel-nox binary> [https list address, or ""] [work folder]

With a list address the leecher fetches it for real (one GET); without one the web list is
skipped. The Run Program line starts this Python with a script that writes its arguments.
Low priority, no windows. Windows only as written (the creation flags).
"""
import json
import os
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
import uuid

import tempfile

# everything the run makes goes here: the third argument, or a new temporary folder
HERE = os.path.abspath(sys.argv[3]) if len(sys.argv) > 3 else tempfile.mkdtemp(prefix="rookparcel-nox-e2e-")
os.makedirs(HERE, exist_ok=True)
NOX = sys.argv[1]
LIST_URL = sys.argv[2] if len(sys.argv) > 2 else ""
PW = "e2e-" + uuid.uuid4().hex[:10]
NAME = "Rook's test & more (1) %TEMP% ;x --delete.bin"
TYPED = "udp://127.0.0.1:9/announce"
IDLE = 0x40 | 0x08000000  # IDLE_PRIORITY_CLASS | CREATE_NO_WINDOW

results = []


def note(ok, what, detail=""):
    results.append((ok, what))
    print(("PASS " if ok else "FAIL ") + what + (": " + str(detail) if detail else ""), flush=True)


class Nox:
    def __init__(self, tag, web, port, save, extra=()):
        self.tag, self.web = tag, web
        self.state = os.path.join(HERE, tag + "-state")
        self.out_path = os.path.join(HERE, tag + ".log")
        os.makedirs(self.state, exist_ok=True)
        os.makedirs(save, exist_ok=True)
        self.args = [NOX, "run", "--state-dir", self.state, "--save", save, "--listen", f"127.0.0.1:{port}",
                     "--webui", f"127.0.0.1:{web}", "--webui-pass", PW, "--no-update-check", *extra]
        self.proc = None
        self.cookie = None

    def start(self):
        self.out = open(self.out_path, "ab")
        self.proc = subprocess.Popen(self.args, stdin=subprocess.PIPE, stdout=self.out, stderr=subprocess.STDOUT,
                                     creationflags=IDLE, cwd=HERE)
        for _ in range(100):
            try:
                self.login()
                return
            except Exception:
                time.sleep(0.2)
        raise RuntimeError(self.tag + " did not come up")

    def stop(self):
        if self.proc and self.proc.poll() is None:
            try:
                self.proc.stdin.write(b"q\n")
                self.proc.stdin.flush()
                self.proc.wait(timeout=30)
            except Exception:
                self.proc.kill()
        self.out.close()

    def req(self, method, path, body=None, ctype=None, headers=None):
        h = {"X-Requested-With": "Rookparcel"}
        if self.cookie:
            h["Cookie"] = self.cookie
        if ctype:
            h["Content-Type"] = ctype
        h.update(headers or {})
        r = urllib.request.Request(f"http://127.0.0.1:{self.web}{path}", data=body, method=method, headers=h)
        try:
            with urllib.request.urlopen(r, timeout=20) as resp:
                return resp.status, resp.read().decode("utf-8", "replace"), resp.headers
        except urllib.error.HTTPError as e:
            return e.code, e.read().decode("utf-8", "replace"), e.headers

    def login(self):
        body = urllib.parse.urlencode({"username": "admin", "password": PW}).encode()
        st, text, hdr = self.req("POST", "/api/v2/auth/login", body, "application/x-www-form-urlencoded")
        if text.strip() != "Ok.":
            raise RuntimeError(f"login: {st} {text}")
        self.cookie = hdr["Set-Cookie"].split(";")[0]

    def get(self, path):
        st, text, _ = self.req("GET", path)
        return json.loads(text) if text[:1] in "[{" else text

    def prefs(self, values, password=True, headers=None):
        v = dict(values)
        if password:
            v["webui.current_password"] = PW
        return self.req("POST", "/pa/prefs", json.dumps(v).encode(), "application/json", headers)[:2]

    def form(self, path, **kv):
        return self.req("POST", path, urllib.parse.urlencode(kv).encode(), "application/x-www-form-urlencoded")[:2]

    def add_file(self, torrent, savepath):
        b = "----e2e" + uuid.uuid4().hex
        parts = []
        for k, v in (("savepath", savepath), ("paused", "false")):
            parts.append(f'--{b}\r\nContent-Disposition: form-data; name="{k}"\r\n\r\n{v}\r\n'.encode())
        parts.append((f'--{b}\r\nContent-Disposition: form-data; name="torrents"; filename="t.torrent"\r\n'
                      'Content-Type: application/x-bittorrent\r\n\r\n').encode() + open(torrent, "rb").read() + b"\r\n")
        parts.append(f"--{b}--\r\n".encode())
        return self.req("POST", "/api/v2/torrents/add", b"".join(parts), f"multipart/form-data; boundary={b}")[:2]

    def torrents(self):
        return self.get("/api/v2/torrents/info")

    def state_of_server(self):
        return self.get("/api/v2/sync/maindata").get("server_state", {})

    def log(self):
        self.out.flush()
        return open(self.out_path, "r", encoding="utf-8", errors="replace").read()


def wait(what, f, secs=60):
    end = time.time() + secs
    while time.time() < end:
        v = f()
        if v:
            return v
        time.sleep(0.5)
    note(False, "waited for " + what)
    return None


def main():
    seed_dir = os.path.join(HERE, "seed-data")
    dl_dir = os.path.join(HERE, "leech-data")
    os.makedirs(seed_dir, exist_ok=True)
    data_path = os.path.join(seed_dir, NAME)
    with open(data_path, "wb") as f:
        f.write(os.urandom(3 * 1024 * 1024 + 123))
    size = os.path.getsize(data_path)
    torrent = os.path.join(HERE, "t.torrent")
    subprocess.run([NOX, "create", data_path, "-o", torrent, "--v1"], check=True, creationflags=IDLE,
                   stdout=subprocess.DEVNULL)

    args_out = os.path.join(HERE, "run-args.json")
    args_py = os.path.join(HERE, "args.py")
    with open(args_py, "w", encoding="utf-8") as f:
        f.write("import json, sys\njson.dump(sys.argv[2:], open(sys.argv[1], 'w', encoding='utf-8'))\n")

    S = Nox("seed", 18111, 18121, seed_dir)
    L = Nox("leech", 18112, 18122, dl_dir)
    try:
        S.start()
        L.start()
        st, text = S.add_file(torrent, seed_dir)
        note(st == 200, "the seeder takes the torrent", text)
        seeded = wait("the seeder to seed", lambda: [t for t in S.torrents() if t.get("progress") == 1], 60)
        if not seeded:
            return
        h = seeded[0]["hash"]
        note(seeded[0]["name"] == NAME, "the awkward name survives", seeded[0]["name"])

        # ---- Run Program: the gate
        line = f'"{sys.executable}" -I "{args_py}" "{args_out}" %N "%F" %L %I %Z "name=%N" end'
        st, text = L.prefs({"run.finished_enabled": True, "run.finished": line}, password=False)
        note(st == 400 and "current_password" in text, "Run Program: refused without the password", text)
        st, text = L.prefs({"run.finished_enabled": True, "run.finished": line}, headers={"X-Forwarded-For": "203.0.113.9"})
        note(st == 400 and "only on the computer" in text, "Run Program: refused when not local, password right", text)
        st, text = L.form("/api/v2/app/setPreferences", json=json.dumps(
            {"autorun_enabled": True, "autorun_program": "calc.exe", "web_ui_current_password": PW}))
        p = L.get("/pa/prefs")["fields"]
        note(p["run.finished"]["value"] == "" and p["run.finished_enabled"]["value"] is False,
             "Run Program: the qBittorrent API cannot set it", p["run.finished"])
        st, text = L.prefs({"run.finished_enabled": True, "run.finished": '"C:\\x\\open.exe %N'})
        note(st == 400, "Run Program: an open quote is refused", text)
        st, text = L.prefs({"run.finished_enabled": True, "run.finished": line})
        note(st in (200, 204), "Run Program: accepted locally with the password", text)
        note("refused (not this computer)" in L.log(), "Run Program: the refusal is in the log")

        # ---- Trackers
        st, text = L.prefs({"trackers.from_url": True, "trackers.url": "http://example.org/list.txt"})
        note(st == 400, "Trackers: an http address is refused", text)
        st, text = L.prefs({"trackers.append": True, "trackers.list": TYPED + "\nnot a tracker"})
        note(st == 400, "Trackers: a junk line is refused", text)
        v = {"trackers.append": True, "trackers.list": TYPED}
        if LIST_URL:
            v.update({"trackers.from_url": True, "trackers.url": LIST_URL})
        st, text = L.prefs(v, password=False)
        note(st == 400, "Trackers: refused without the password", text)
        st, text = L.prefs(v)
        note(st in (200, 204), "Trackers: accepted with the password", text)
        fetched = 0
        if LIST_URL:
            status = wait("the list from the web", lambda: (lambda s: s if "trackers, downloaded" in s or "failed" in s else None)(
                L.get("/pa/prefs/trackers").get("status", "")), 60)
            note(bool(status) and "failed" not in status, "Trackers: the https list was read", status)
            print("     status line:", status)

        # ---- the download, as a magnet (the trackers wait for its metadata)
        magnet = f"magnet:?xt=urn:btih:{h}&x.pe=127.0.0.1:18121"
        st, text = L.form("/api/v2/torrents/add", urls=magnet, savepath=dl_dir)
        note(st == 200, "the leecher takes the magnet", text)
        done = wait("the download", lambda: [t for t in L.torrents() if t["hash"] == h and t.get("progress") == 1], 120)
        if not done:
            print(L.log()[-3000:])
            return
        same = open(os.path.join(dl_dir, NAME), "rb").read() == open(data_path, "rb").read()
        note(same, "the downloaded file is the seeded one")
        tr = [t["url"] for t in L.get(f"/api/v2/torrents/trackers?hash={h}") if "://" in t["url"]]
        fetched = len(tr) - 1
        note(TYPED in tr, "Trackers: the typed tracker is on the new download", tr[:3])
        if LIST_URL:
            note(fetched > 0, f"Trackers: {fetched} trackers of the web list are on it too")
            note(tr[0] == TYPED, "Trackers: the typed one comes first")
        str_s = [t["url"] for t in S.get(f"/api/v2/torrents/trackers?hash={h}") if "://" in t["url"]]
        note(str_s == [], "Trackers: the seeder, which has no list, got none", str_s)

        # ---- Run Program: what the program got
        got = wait("the program's arguments", lambda: os.path.exists(args_out) and json.load(open(args_out, encoding="utf-8")), 30)
        if got:
            want = [NAME, os.path.join(dl_dir, NAME), "", h, str(size), "name=" + NAME, "end"]
            note(got == want, "Run Program: every argument arrived whole", got)
        log = L.log()
        note("Run Program: started" in log and "ended" in log, "Run Program: start and end are logged")

        # ---- Statistics
        s_l, s_s = L.state_of_server(), S.state_of_server()
        note(s_l.get("alltime_dl", 0) >= size, "Statistics: the leecher's all-time download", s_l.get("alltime_dl"))
        note(s_s.get("alltime_ul", 0) >= size, "Statistics: the seeder's all-time upload", s_s.get("alltime_ul"))
        rows = L.get("/pa/session/stats")["rows"]
        note(len(rows) == 14 and rows[0].get("h") == "All Time", "Statistics: the Web UI's rows", rows[1:3])
        before = s_l["alltime_dl"]
        L.stop()
        note(os.path.exists(os.path.join(L.state, "stats.dat")), "Statistics: stats.dat is written")
        note(os.path.exists(os.path.join(L.state, "trackers-url.txt")) == bool(LIST_URL), "Trackers: the cache file")
        os.remove(args_out)
        L.cookie = None
        L.start()
        s2 = L.state_of_server()
        note(s2.get("alltime_dl") == before and s2.get("dl_info_data") == 0,
             "Statistics: the total is back after a restart, nothing moved", (s2.get("alltime_dl"), s2.get("dl_info_data")))
        L.form("/api/v2/torrents/delete", hashes=h, deleteFiles="false")
        wait("the removal", lambda: not L.torrents(), 20)
        note(L.state_of_server().get("alltime_dl") == before, "Statistics: removing the torrent takes nothing away")
        p = L.get("/pa/prefs")["fields"]
        note(p["run.finished"]["value"] == line, "Run Program: the line is kept over the restart")
        time.sleep(3)
        note(not os.path.exists(args_out), "Run Program: nothing ran again after the restart")
        if LIST_URL:
            status = L.get("/pa/prefs/trackers").get("status", "")
            note("downloaded" in status, "Trackers: the list comes from the cache after the restart", status)
    finally:
        L.stop()
        S.stop()
    bad = [w for ok, w in results if not ok]
    print(f"\n{len(results) - len(bad)} passed, {len(bad)} failed")
    for w in bad:
        print("  FAILED:", w)
    sys.exit(1 if bad else 0)


main()
