#!/usr/bin/env python3
"""The polyfill table's checks and its generator (L6 decision 15; L11 D-53;
W-175). Standard library only (L11).

    python3 scripts/data-polyfills-check.py --check
        Offline: data-packages/polyfills/data/polyfills.json is schema 1, its
        version is YYYY.M.D, `generated_from` names core-js-compat with a
        version and an integrity, and the rows are well-formed: `features`
        sorted and unique, `since` parallel to it with versions of the
        engines listed, every entry's indexes inside `features`. CI's lint
        job runs it.

    python3 scripts/data-polyfills-check.py --refresh [--version V] [--from-tarball FILE]
        Online (CI's weekly job; by hand with --from-tarball, offline): reads
        core-js-compat's published tarball (the registry's latest, or V),
        checks its integrity against the registry's, and writes the table
        from its `data.json` (feature -> first version per engine) and
        `entries.json` (core-js entry -> features). Nothing is installed or
        executed from the package. The table's version is today's date when
        its rows changed, and stays when they did not. Exit 0.
"""

from __future__ import annotations

import argparse
import base64
import datetime
import hashlib
import io
import json
import re
import sys
import tarfile
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TABLE = ROOT / "data-packages" / "polyfills" / "data" / "polyfills.json"
VERSION = re.compile(r"^\d+(\.\d+)*$")
SOURCE = "core-js-compat"
REGISTRY = "https://registry.npmjs.org/core-js-compat"


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
    if src.get("package") != SOURCE or not VERSION.match(str(src.get("version", ""))) or not str(src.get("integrity", "")).startswith("sha512-"):
        problems.append("`generated_from` is not {package: core-js-compat, version, integrity: sha512-…}")
    engines = t.get("engines")
    features = t.get("features")
    since = t.get("since")
    entries = t.get("entries")
    if not isinstance(engines, list) or not engines or engines != sorted(set(engines)):
        problems.append("`engines` is not a sorted list of unique names")
        engines = []
    if not isinstance(features, list) or not features or features != sorted(set(features)):
        problems.append("`features` is not a sorted list of unique names")
        features = []
    if not isinstance(since, list) or len(since) != len(features):
        problems.append("`since` is not parallel to `features`")
        since = []
    for name, row in zip(features, since):
        # An empty row is a feature no engine has built in yet.
        if not isinstance(row, dict):
            problems.append(f"{name}: not a table of versions")
            continue
        for engine, v in row.items():
            if engine not in engines:
                problems.append(f"{name}: `{engine}` is not in `engines`")
            if not VERSION.match(str(v)):
                problems.append(f"{name}: `{engine}` {v!r} is not a version")
    if not isinstance(entries, dict) or not entries:
        problems.append("`entries` is empty")
        entries = {}
    for entry, idx in entries.items():
        if not entry.startswith("core-js"):
            problems.append(f"{entry}: an entry is a core-js path")
        if not isinstance(idx, list) or any(not isinstance(i, int) or not 0 <= i < len(features) for i in idx):
            problems.append(f"{entry}: an index is outside `features`")
    for p in problems[:50]:
        print(f"data-polyfills: {p}", file=sys.stderr)
    if problems:
        print(f"data-polyfills: {len(problems)} problem(s)", file=sys.stderr)
        return 1
    print(f"✔ data-polyfills {t['version']}: {len(features)} features, {len(entries)} entries, from {SOURCE} {src['version']}")
    return 0


def fetch(url: str) -> bytes:
    req = urllib.request.Request(url, headers={"User-Agent": "codelaw-data-polyfills"})
    with urllib.request.urlopen(req, timeout=60) as r:  # noqa: S310 — the registry, CI only (L5)
        return r.read()


def build(tarball: bytes, version: str) -> dict:
    integrity = "sha512-" + base64.b64encode(hashlib.sha512(tarball).digest()).decode()
    with tarfile.open(fileobj=io.BytesIO(tarball), mode="r:gz") as tar:
        def member(name: str) -> dict:
            f = tar.extractfile(f"package/{name}")
            if f is None:
                raise SystemExit(f"data-polyfills: {SOURCE} {version} has no {name}")
            return json.loads(f.read().decode("utf-8"))
        data, entries = member("data.json"), member("entries.json")
    features = sorted(data)
    index = {name: i for i, name in enumerate(features)}
    engines = sorted({e for row in data.values() for e in row})
    return {
        "schema": 1,
        "version": "",
        "generated_from": {"package": SOURCE, "version": version, "integrity": integrity},
        "engines": engines,
        "features": features,
        "since": [{e: str(row[e]) for e in sorted(row)} for row in (data[f] for f in features)],
        "entries": {entry: sorted(index[f] for f in names if f in index) for entry, names in sorted(entries.items())},
    }


def refresh(version: str | None, tarball_path: str | None) -> int:
    if tarball_path:
        raw = Path(tarball_path).read_bytes()
        m = re.search(r"core-js-compat-(\d+\.\d+\.\d+)\.tgz$", tarball_path)
        if not m:
            raise SystemExit("data-polyfills: --from-tarball wants a file named core-js-compat-<version>.tgz")
        version = m.group(1)
    else:
        meta = json.loads(fetch(f"{REGISTRY}/{version or 'latest'}"))
        version = meta["version"]
        raw = fetch(meta["dist"]["tarball"])
        want = meta["dist"].get("integrity", "")
        got = "sha512-" + base64.b64encode(hashlib.sha512(raw).digest()).decode()
        if want and want != got:
            raise SystemExit(f"data-polyfills: the tarball's integrity {got} is not the registry's {want}")
    table = build(raw, version)
    old = load() if TABLE.exists() else {}
    same = all(old.get(k) == table[k] for k in ("engines", "features", "since", "entries")) and (old.get("generated_from") or {}).get("version") == version
    today = datetime.date.today()
    table["version"] = old["version"] if same and old.get("version") else f"{today.year}.{today.month}.{today.day}"
    TABLE.parent.mkdir(parents=True, exist_ok=True)
    TABLE.write_text(json.dumps(table, separators=(",", ":"), ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"data-polyfills {table['version']}: {len(table['features'])} features from {SOURCE} {version}" + (" (unchanged)" if same else ""))
    return 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--refresh", action="store_true")
    ap.add_argument("--version")
    ap.add_argument("--from-tarball")
    a = ap.parse_args()
    if a.refresh:
        return refresh(a.version, a.from_tarball)
    return check()


if __name__ == "__main__":
    sys.exit(main())
