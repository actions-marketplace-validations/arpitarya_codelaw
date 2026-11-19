#!/usr/bin/env python3
"""The confusables table's checks and its generator (L6 decision 15; L11 D-54;
W-182). Standard library only (L11).

    python3 scripts/data-confusables-check.py --check
        Offline: data-packages/confusables/data/confusables.json is schema 1,
        its version is YYYY.M.D, `generated_from` names Unicode's
        confusables.txt with a version, a versioned URL and a SHA-256, and the
        rows are well-formed: `from` a sorted list of unique non-ASCII code
        points, `to` and `names` parallel to it (`to` one printable ASCII character), and
        `target_names` naming every `to`. The NOTICE carries the Unicode
        License v3 and the source's copyright line. CI's lint job runs it.

    python3 scripts/data-confusables-check.py --refresh [--version V] [--from-file FILE]
        Online (CI's weekly job; by hand with --from-file, offline): reads
        Unicode's `confusables.txt` (the latest, or version V, from
        unicode.org/Public/<V>/security/), refuses it unless the licence page
        still says UNICODE LICENSE V3 (L2), and writes the table: every
        single-code-point, non-ASCII source whose prototype is one printable
        ASCII character. The names come from the file's own comments. The table's version is today's date when its
        rows changed, and stays when they did not. With --from-file the
        NOTICE is left as it is. Exit 0.
"""

from __future__ import annotations

import argparse
import datetime
import hashlib
import json
import re
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PACKAGE = ROOT / "data-packages" / "confusables"
TABLE = PACKAGE / "data" / "confusables.json"
NOTICE = PACKAGE / "NOTICE"
SOURCE = "Unicode confusables.txt"
VERSIONED = "https://www.unicode.org/Public/{}/security/confusables.txt"
LATEST = "https://www.unicode.org/Public/security/latest/confusables.txt"
LICENCE = "https://www.unicode.org/license.txt"
LICENCE_HEAD = "UNICODE LICENSE V3"


def load() -> dict:
    return json.loads(TABLE.read_text(encoding="utf-8"))


def check() -> int:
    problems: list[str] = []
    t = load()
    if t.get("schema") != 1:
        problems.append("`schema` is not 1")
    if not re.fullmatch(r"\d{4}\.\d{1,2}\.\d{1,2}", str(t.get("version", ""))):
        problems.append(f"`version` {t.get('version')!r} is not YYYY.M.D")
    src = t.get("generated_from") or {}
    ok_src = (
        src.get("source") == SOURCE
        and re.fullmatch(r"\d+\.\d+\.\d+", str(src.get("version", "")))
        and src.get("url") == VERSIONED.format(src.get("version"))
        and re.fullmatch(r"[0-9a-f]{64}", str(src.get("sha256", "")))
    )
    if not ok_src:
        problems.append("`generated_from` is not {source: Unicode confusables.txt, version, url: the versioned URL, sha256}")
    cps, to, names, tnames = t.get("from"), t.get("to"), t.get("names"), t.get("target_names")
    if not isinstance(cps, list) or not cps or any(not isinstance(c, int) or isinstance(c, bool) for c in cps):
        problems.append("`from` is not a list of code points")
        cps = []
    elif cps != sorted(set(cps)):
        problems.append("`from` is not sorted and unique")
    elif cps[0] < 128 or cps[-1] > 0x10FFFF:
        problems.append("`from` holds an ASCII or out-of-range code point")
    if not isinstance(to, list) or len(to) != len(cps):
        problems.append("`to` is not parallel to `from`")
        to = []
    if not isinstance(names, list) or len(names) != len(cps):
        problems.append("`names` is not parallel to `from`")
        names = []
    if not isinstance(tnames, dict):
        problems.append("`target_names` is not a table")
        tnames = {}
    for cp, target in zip(cps, to):
        if not isinstance(target, str) or len(target) != 1 or not 32 <= ord(target) < 127:
            problems.append(f"U+{cp:04X}: `{target}` is not one printable ASCII character")
        elif target not in tnames:
            problems.append(f"U+{cp:04X}: `{target}` has no name in `target_names`")
    for cp, name in zip(cps, names):
        if not isinstance(name, str) or not name:
            problems.append(f"U+{cp:04X}: no name")
    notice = NOTICE.read_text(encoding="utf-8") if NOTICE.exists() else ""
    if LICENCE_HEAD not in notice or "©" not in notice:
        problems.append("NOTICE does not carry the Unicode License v3")
    for p in problems[:50]:
        print(f"data-confusables: {p}", file=sys.stderr)
    if problems:
        print(f"data-confusables: {len(problems)} problem(s)", file=sys.stderr)
        return 1
    print(f"✔ data-confusables {t['version']}: {len(cps)} characters, from {SOURCE} {src['version']}")
    return 0


def fetch(url: str) -> bytes:
    req = urllib.request.Request(url, headers={"User-Agent": "codelaw-data-confusables"})
    with urllib.request.urlopen(req, timeout=60) as r:  # noqa: S310 — unicode.org, CI only (L5)
        return r.read()


LINE = re.compile(r"^([0-9A-F ]+);\s*([0-9A-F ]+);\s*MA\s*#\S*\s*\(.*?\)\s+(.*?) → (.*?)\s*#")


def build(raw: bytes) -> tuple[dict, str]:
    text = raw.decode("utf-8-sig")
    m = re.search(r"^# Version: (\d+\.\d+\.\d+)\s*$", text, re.M)
    if not m:
        raise SystemExit("data-confusables: the file has no `# Version:` line")
    version = m.group(1)
    rows: dict[int, tuple[str, str]] = {}
    tnames: dict[str, str] = {}
    for line in text.splitlines():
        if not line or line.startswith("#"):
            continue
        body = LINE.match(line + " #")
        if not body:
            raise SystemExit(f"data-confusables: a line is not the format UTS #39 gives: {line!r}")
        src = [int(x, 16) for x in body.group(1).split()]
        dst = [int(x, 16) for x in body.group(2).split()]
        # One code point for one printable ASCII character: the confusion a
        # reader makes. Ligatures, fractions and the ellipsis (prototypes of
        # several characters) are not that.
        if len(src) != 1 or src[0] < 128 or len(dst) != 1 or not 32 <= dst[0] < 127:
            continue
        target = "".join(map(chr, dst))
        if src[0] in rows:
            raise SystemExit(f"data-confusables: U+{src[0]:04X} is listed twice")
        rows[src[0]] = (target, body.group(3))
        if tnames.setdefault(target, body.group(4)) != body.group(4):
            raise SystemExit(f"data-confusables: `{target}` is named two ways")
    cps = sorted(rows)
    table = {
        "schema": 1,
        "version": "",
        "generated_from": {"source": SOURCE, "version": version, "url": VERSIONED.format(version), "sha256": hashlib.sha256(raw).hexdigest()},
        "from": cps,
        "to": [rows[c][0] for c in cps],
        "names": [rows[c][1] for c in cps],
        "target_names": dict(sorted(tnames.items())),
    }
    return table, text


def notice(licence: str, header: str) -> str:
    copyright_line = next((l.lstrip("# ").strip() for l in header.splitlines() if "©" in l), "© Unicode, Inc.")
    return (
        "codelaw-data-confusables / @codelaw/data-confusables — codelaw's confusables table.\n\n"
        "The rows in data/confusables.json are generated by scripts/data-confusables-check.py\n"
        "--refresh from Unicode's confusables.txt (UTS #39, Unicode Security Mechanisms):\n"
        "every single-code-point, non-ASCII source whose prototype is one printable ASCII\n"
        "character, with the names the file's own comments give. The package's `generated_from` names the\n"
        "version, the versioned URL and the SHA-256 of the file read.\n\n"
        f"confusables.txt\n{copyright_line}\n"
        "Unicode and the Unicode Logo are registered trademarks of Unicode, Inc.\n"
        "Licensed under the Unicode License v3 (https://www.unicode.org/license.txt):\n\n"
        f"{licence.strip()}\n\n"
        "Licence of this package: MIT, as codelaw; the data keeps the licence above.\n"
    )


def refresh(version: str | None, path: str | None) -> int:
    licence = ""
    if path:
        raw = Path(path).read_bytes()
    else:
        raw = fetch(VERSIONED.format(version) if version else LATEST)
        licence = fetch(LICENCE).decode("utf-8")
        if not licence.lstrip().startswith(LICENCE_HEAD):
            raise SystemExit(f"data-confusables: {LICENCE} no longer says {LICENCE_HEAD}; L2 is the decider's to re-judge")
    table, text = build(raw)
    got = table["generated_from"]["version"]
    if not path:
        pinned = fetch(VERSIONED.format(got))
        if pinned != raw:
            raise SystemExit(f"data-confusables: the file at {VERSIONED.format(got)} is not the one read; refusing to pin a moving file")
    elif version and version != got:
        raise SystemExit(f"data-confusables: the file is version {got}, not {version}")
    old = load() if TABLE.exists() else {}
    keys = ("from", "to", "names", "target_names")
    same = all(old.get(k) == table[k] for k in keys) and (old.get("generated_from") or {}).get("sha256") == table["generated_from"]["sha256"]
    today = datetime.date.today()
    table["version"] = old["version"] if same and old.get("version") else f"{today.year}.{today.month}.{today.day}"
    TABLE.parent.mkdir(parents=True, exist_ok=True)
    TABLE.write_text(json.dumps(table, separators=(",", ":"), ensure_ascii=False) + "\n", encoding="utf-8")
    if licence:
        NOTICE.write_text(notice(licence, text), encoding="utf-8")
    print(f"data-confusables {table['version']}: {len(table['from'])} characters from {SOURCE} {got}" + (" (unchanged)" if same else ""))
    return 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--refresh", action="store_true")
    ap.add_argument("--version")
    ap.add_argument("--from-file")
    a = ap.parse_args()
    if a.refresh:
        return refresh(a.version, a.from_file)
    return check()


if __name__ == "__main__":
    sys.exit(main())
