#!/usr/bin/env python3
"""Stato della routine CPC: cosa c'è di nuovo o modificato rispetto all'ultima esecuzione.

  python3 check_new.py            -> JSON con i file nuovi/modificati (sorgenti) e i file
                                     generati dalla routine che Octech ha modificato a mano
  python3 check_new.py --segna    -> registra lo stato attuale come "già integrato"
  python3 check_new.py --segna-generati FILE...  -> registra l'hash dei file generati (dopo averli scritti)

Funziona sia nella shell del Mac (device_bash, cartelle in $HOME/mnt) sia con
CPC_ROOT / DL_ROOT impostate a mano.
"""
import hashlib, json, os, re, sys, time

HOME = os.path.expanduser("~")
ROOT = os.environ.get("CPC_ROOT", os.path.join(HOME, "mnt", "CPC"))
DL = os.environ.get("DL_ROOT", os.path.join(HOME, "mnt", "Downloads"))
STATE = os.path.join(ROOT, "_routine", "state.json")

SKIP_DIRS = {".git", "target", "__pycache__", "_routine", "board", "figs", ".pytest_cache"}
SKIP_FILES = {".DS_Store", "Cargo.lock", "claude_notes.md"}
GEN_RE = re.compile(r"^L\d+\.(tex|pdf|aux|log|out|toc)$|_transcript\.txt$")
# registrazioni / materiale del corso in Download
DL_RE = re.compile(r"^(l|lez|lezione|lecture)[ _-]?0*\d{1,2}\b.*\.(mp4|mov|m4a|mkv|webm)$", re.I)
DL_KW = re.compile(r"cpc|competitive|rossano", re.I)
LESSON_RE = re.compile(r"(?:^|/)(?:l|lez|lezione|lecture)[ _-]?0*(\d{1,2})(?:\b|_|\.)", re.I)


def sha1(p, cap=None):
    h = hashlib.sha1()
    with open(p, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return h.hexdigest()


def sources():
    out = {}
    for base, dirs, files in os.walk(os.path.join(ROOT, "Lessons")):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS and not d.startswith(".")]
        for f in files:
            if f in SKIP_FILES or GEN_RE.search(f) or f.startswith("."):
                continue
            p = os.path.join(base, f)
            out[os.path.relpath(p, ROOT)] = p
    if os.path.isdir(DL):
        for f in os.listdir(DL):
            p = os.path.join(DL, f)
            if os.path.isfile(p) and (DL_RE.match(f) or DL_KW.search(f)):
                out["DOWNLOADS/" + f] = p
    return out


def load():
    try:
        return json.load(open(STATE))
    except FileNotFoundError:
        return {"seen": {}, "generated": {}, "last_run": None}


def sig(p):
    st = os.stat(p)
    return {"size": st.st_size, "mtime": int(st.st_mtime)}


def lesson_of(key):
    m = LESSON_RE.search(key.split("DOWNLOADS/")[-1] if key.startswith("DOWNLOADS/") else key)
    return int(m.group(1)) if m else None


def main():
    s = load()
    args = sys.argv[1:]
    if args[:1] == ["--segna"]:
        s["seen"] = {k: sig(p) for k, p in sources().items()}
        s["last_run"] = time.strftime("%Y-%m-%d %H:%M")
        json.dump(s, open(STATE, "w"), indent=1, sort_keys=True)
        print("stato salvato:", len(s["seen"]), "file")
        return
    if args[:1] == ["--segna-generati"]:
        for p in args[1:]:
            ap = os.path.abspath(p)
            s["generated"][os.path.relpath(ap, ROOT)] = sha1(ap)
        json.dump(s, open(STATE, "w"), indent=1, sort_keys=True)
        print("generati registrati:", len(args) - 1)
        return
    new, changed = [], []
    for k, p in sorted(sources().items()):
        cur = sig(p)
        old = s["seen"].get(k)
        item = {"file": k, "path": p, "lesson": lesson_of(k), **cur}
        if old is None:
            new.append(item)
        elif old != cur:
            changed.append(item)
    edited = []
    for rel, h in s["generated"].items():
        p = os.path.join(ROOT, rel)
        if os.path.exists(p) and sha1(p) != h:
            edited.append(rel)
    print(json.dumps({"last_run": s["last_run"], "new": new, "changed": changed,
                      "generated_edited_by_user": edited}, indent=1))


if __name__ == "__main__":
    main()
