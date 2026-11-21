#!/usr/bin/env python3
"""The imperative-mood word table: generated from pydocstyle's two word lists,
never copied by hand (L11 D-56; SR-RULE-PACKS; W-180). Standard library only.

    data-imperatives.py --check
        Regenerates the table in memory from the pinned source and fails if
        a committed byte differs. The source is pydocstyle's sdist, fetched
        once into the cache directory (the network, here and never in the
        engine, L5), its SHA-256 checked against the pin below before a byte
        of it is read; nothing of it is installed or executed. CI's lint job
        runs this.

    data-imperatives.py --check --offline
        No network: the committed table is schema 1, names the pinned source
        (version and SHA-256), and its two lists are sorted, unique, lower
        case and non-empty.

    data-imperatives.py --write [--from-tarball FILE]
        Writes packs/python/data/python/imperatives.json from the pinned
        source (or from FILE, which must have the pinned SHA-256).

What it writes: ``imperatives`` — the word list pydocstyle calls
``imperatives.txt`` (the imperative form of the verbs docstrings start with,
about 230); ``blacklist`` — its ``imperatives_blacklist.txt`` (first words
that mark a docstring as not imperative: nouns, adjectives, ``this``). Both
are read as pydocstyle reads them: a ``#`` starts a comment, whitespace is
stripped, an empty line is skipped. The lists are sorted and unique, so the
same source writes the same bytes.

Raising the pin is a change to this file (and to L11's D-56 row), which the
decider ratifies again (L11 decision 4).
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
import os
import re
import sys
import tarfile
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TABLE = ROOT / "packs" / "python" / "data" / "python" / "imperatives.json"

# The pin (L11 §3, D-56, records the same version and digest).
PACKAGE = "pydocstyle"
VERSION = "6.3.0"
FILE = "pydocstyle-6.3.0.tar.gz"
URL = "https://files.pythonhosted.org/packages/e9/5c/d5385ca59fd065e3c6a5fe19f9bc9d5ea7f2509fa8c9c22fb6b2031dd953/pydocstyle-6.3.0.tar.gz"
SHA256 = "7ce43f0c0ac87b07494eb9c0b462c0b73e6ff276807f204d6b53edc72b7e44e1"
LISTS = {
    "imperatives": "pydocstyle-6.3.0/src/pydocstyle/data/imperatives.txt",
    "blacklist": "pydocstyle-6.3.0/src/pydocstyle/data/imperatives_blacklist.txt",
}
COMMENT = re.compile(r"\s*#.*")
WORD = re.compile(r"^[a-z]+$")


def cache_dir() -> Path:
    base = os.environ.get("XDG_CACHE_HOME") or str(Path.home() / ".cache")
    return Path(base) / "codelaw" / "imperatives"


def fetch() -> bytes:
    path = cache_dir() / FILE
    if not path.exists():
        path.parent.mkdir(parents=True, exist_ok=True)
        req = urllib.request.Request(URL, headers={"User-Agent": "codelaw-data-imperatives"})
        with urllib.request.urlopen(req, timeout=60) as r:
            data = r.read()
        if hashlib.sha256(data).hexdigest() != SHA256:
            sys.exit(f"{URL}: not the pinned SHA-256 ({SHA256})")
        path.write_bytes(data)
    return path.read_bytes()


def words(text: str) -> list[str]:
    """pydocstyle's `load_wordlist`, sorted and unique."""
    out = set()
    for line in text.splitlines():
        line = COMMENT.sub("", line).strip()
        if line:
            out.add(line)
    return sorted(out)


def generate(tarball: bytes) -> str:
    if hashlib.sha256(tarball).hexdigest() != SHA256:
        sys.exit(f"the source is not the pinned SHA-256 ({SHA256})")
    tar = tarfile.open(fileobj=io.BytesIO(tarball), mode="r:gz")
    lists = {}
    for key, member in LISTS.items():
        f = tar.extractfile(member)
        if f is None:
            sys.exit(f"{member} is not in {FILE}")
        lists[key] = words(f.read().decode("utf-8"))
    source = {"package": PACKAGE, "version": VERSION, "sha256": SHA256, "files": sorted(LISTS.values())}
    lines = [
        "{",
        '"schema": 1,',
        '"generated_by": "scripts/data-imperatives.py",',
        '"generated_from": ' + json.dumps(source) + ",",
    ]
    for i, key in enumerate(("imperatives", "blacklist")):
        lines.append(f'"{key}": [')
        lines.append(",\n".join(json.dumps(w) for w in lists[key]))
        lines.append("]" + ("," if i == 0 else ""))
    lines.append("}")
    return "\n".join(lines) + "\n"


def offline_problems() -> list[str]:
    problems = []
    try:
        t = json.loads(TABLE.read_text(encoding="utf-8"))
    except (OSError, ValueError) as e:
        return [f"{TABLE.relative_to(ROOT)}: {e}"]
    if t.get("schema") != 1:
        problems.append("`schema` is not 1")
    src = t.get("generated_from") or {}
    if (src.get("package"), src.get("version"), src.get("sha256")) != (PACKAGE, VERSION, SHA256):
        problems.append(f"`generated_from` is not {PACKAGE} {VERSION} with SHA-256 {SHA256}")
    for key in LISTS:
        ws = t.get(key)
        if not isinstance(ws, list) or not ws:
            problems.append(f"`{key}` is not a non-empty list")
        elif ws != sorted(set(ws)):
            problems.append(f"`{key}` is not sorted and unique")
        elif not all(isinstance(w, str) and WORD.match(w) for w in ws):
            problems.append(f"`{key}` holds a word that is not lower-case letters")
    return problems


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true", help="fail if the committed table is not what the pinned source writes")
    ap.add_argument("--write", action="store_true", help="write the table from the pinned source")
    ap.add_argument("--offline", action="store_true", help="with --check: the committed table's own structure only")
    ap.add_argument("--from-tarball", metavar="FILE", help="read the source from FILE instead of fetching it")
    a = ap.parse_args()
    if a.check == a.write:
        ap.error("exactly one of --check and --write")
    if a.check and a.offline:
        problems = offline_problems()
        for p in problems:
            print(f"data-imperatives: {p}", file=sys.stderr)
        if not problems:
            print("data-imperatives: the table is well-formed and names its pinned source")
        return 1 if problems else 0
    data = Path(a.from_tarball).read_bytes() if a.from_tarball else fetch()
    text = generate(data)
    if a.write:
        TABLE.parent.mkdir(parents=True, exist_ok=True)
        TABLE.write_text(text, encoding="utf-8")
        print(f"wrote {TABLE.relative_to(ROOT)}")
        return 0
    if not TABLE.exists() or TABLE.read_text(encoding="utf-8") != text:
        print(f"data-imperatives: {TABLE.relative_to(ROOT)} differs from what {PACKAGE} {VERSION} writes", file=sys.stderr)
        return 1
    print(f"data-imperatives: {TABLE.relative_to(ROOT)} is what {PACKAGE} {VERSION} writes")
    return 0


if __name__ == "__main__":
    sys.exit(main())
