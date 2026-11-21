#!/usr/bin/env python3
"""The table of moved typing and collections names `py/deprecated-import`
(Ruff's UP035) reads, and its generator (L11 D-57; W-181). Standard library
only (L11).

    python3 scripts/ruff-moved-names.py --check
        Offline: packs/python/data/python/moved-names.json is schema 1, names
        its source (Ruff, a tag, a commit, the source file's SHA-256), and its
        entries are well-formed, sorted and unique. CI's lint job runs it.

    python3 scripts/ruff-moved-names.py --verify
        Online (CI's lint job; by hand with --from-source, offline): reads the
        pinned source file at the pinned commit, checks its SHA-256 and that
        regenerating the table gives the committed file, byte for byte.

    python3 scripts/ruff-moved-names.py --refresh [--tag T] [--from-source FILE]
        Online or from a saved file: rewrites the table from Ruff's
        `crates/ruff_linter/src/rules/pyupgrade/rules/deprecated_import.rs`
        at tag T (the pin's, by default) and pins the commit and the file's
        SHA-256. Nothing is compiled or executed from the source; the member
        lists are read as text. Nothing here is in the engine: the engine
        reads the table the script wrote (L5).

Every `const NAME: &[&str]` of the source must be one of `CONSTS` below, so a
list Ruff adds in a later tag stops the script instead of going unread.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TABLE = ROOT / "packs" / "python" / "data" / "python" / "moved-names.json"
REPO = "astral-sh/ruff"
PATH = "crates/ruff_linter/src/rules/pyupgrade/rules/deprecated_import.rs"
PIN_TAG = "0.16.9"
PIN_COMMIT = "0be08a206f9c3180afd3e93bcc792ed5cb1f4db1"
VERSION = re.compile(r"^3\.\d+$")

# const name -> (imported from, moved to, first Python version, first version it stops being reported)
CONSTS: dict[str, tuple[str, str, str, str | None]] = {
    "COLLECTIONS_TO_ABC": ("collections", "collections.abc", "3.0", None),
    "PIPES_TO_SHLEX": ("pipes", "shlex", "3.0", None),
    "TYPING_EXTENSIONS_TO_TYPING": ("typing_extensions", "typing", "3.0", None),
    "MYPY_EXTENSIONS_TO_TYPING_37": ("mypy_extensions", "typing", "3.7", None),
    "TYPING_EXTENSIONS_TO_TYPING_37": ("typing_extensions", "typing", "3.7", None),
    "TYPING_IO_TO_TYPING_37": ("typing.io", "typing", "3.7", None),
    "TYPING_RE_TO_TYPING_37": ("typing.re", "typing", "3.7", "3.9"),
    "MYPY_EXTENSIONS_TO_TYPING_38": ("mypy_extensions", "typing", "3.8", None),
    "TYPING_EXTENSIONS_TO_TYPING_38": ("typing_extensions", "typing", "3.8", None),
    "TYPING_TO_COLLECTIONS_ABC_39": ("typing", "collections.abc", "3.9", None),
    "TYPING_TO_COLLECTIONS_39": ("typing", "collections", "3.9", None),
    "TYPING_TO_RE_39": ("typing", "re", "3.9", None),
    "TYPING_RE_TO_RE_39": ("typing.re", "re", "3.9", None),
    "TYPING_EXTENSIONS_TO_TYPING_39": ("typing_extensions", "typing", "3.9", None),
    "TYPING_TO_COLLECTIONS_ABC_310": ("typing", "collections.abc", "3.10", None),
    "TYPING_EXTENSIONS_TO_TYPING_310": ("typing_extensions", "typing", "3.10", None),
    "TYPING_EXTENSIONS_TO_TYPING_311": ("typing_extensions", "typing", "3.11", None),
    "BACKPORTS_STR_ENUM_TO_ENUM_311": ("backports.strenum", "enum", "3.11", None),
    "TYPING_EXTENSIONS_TO_TYPING_312": ("typing_extensions", "typing", "3.12", None),
    "TYPING_EXTENSIONS_TO_COLLECTIONS_ABC_312": ("typing_extensions", "collections.abc", "3.12", None),
    "TYPING_EXTENSIONS_TO_TYPES_312": ("typing_extensions", "types", "3.12", None),
    "TYPING_EXTENSIONS_TO_TYPING_313": ("typing_extensions", "typing", "3.13", None),
    "TYPING_EXTENSIONS_TO_TYPES_313": ("typing_extensions", "types", "3.13", None),
    "TYPING_EXTENSIONS_TO_WARNINGS_313": ("typing_extensions", "warnings", "3.13", None),
}
# `typing` names moved and renamed (no fix): the pairs of TYPING_TO_RENAME_PY39.
RENAMES = "TYPING_TO_RENAME_PY39"
RENAMES_FROM = ("typing", "3.9")


def fetch(url: str) -> bytes:
    req = urllib.request.Request(url, headers={"User-Agent": "codelaw-ruff-moved-names"})
    with urllib.request.urlopen(req, timeout=60) as r:  # noqa: S310 — GitHub, CI and maintainers only (L5)
        return r.read()


def code_of(text: str) -> str:
    """The source with its `//` comments dropped (the member lists carry
    notes about names Ruff leaves out)."""
    return "\n".join(re.sub(r"//.*$", "", line) for line in text.splitlines())


def build(source: bytes, tag: str, commit: str) -> dict:
    text = code_of(source.decode("utf-8"))
    found = dict(re.findall(r"const (\w+): &\[&str\] = &\[(.*?)\];", text, re.S))
    pairs = re.findall(r"const " + RENAMES + r": &\[\(&str, &str\)\] = &\[(.*?)\n\];", text, re.S)
    if len(pairs) != 1:
        raise SystemExit(f"ruff-moved-names: {RENAMES} is not in the source")
    unknown = sorted(set(found) - set(CONSTS))
    missing = sorted(set(CONSTS) - set(found))
    if unknown or missing:
        raise SystemExit(f"ruff-moved-names: the source's lists differ from CONSTS: unknown {unknown}, missing {missing}")
    entries = []
    for name, (module, target, since, until) in CONSTS.items():
        for member in re.findall(r'"([^"]+)"', found[name]):
            entry = {"module": module, "name": member, "target": target, "since": since}
            if until:
                entry["until"] = until
            entries.append(entry)
    for member, target in re.findall(r'\(\s*"([^"]+)",\s*"([^"]+)"\s*,?\s*\)', pairs[0]):
        entries.append({"module": RENAMES_FROM[0], "name": member, "target": target, "since": RENAMES_FROM[1], "rename": True})
    entries.sort(key=lambda e: (e["module"], e["name"], e["target"], key(e["since"])))
    return {
        "schema": 1,
        "generated_from": {
            "repo": REPO,
            "tag": tag,
            "commit": commit,
            "path": PATH,
            "sha256": hashlib.sha256(source).hexdigest(),
            "licence": "MIT",
        },
        "entries": entries,
    }


def key(v: str) -> tuple[int, ...]:
    return tuple(int(p) for p in v.split("."))


def render(table: dict) -> str:
    head = {k: v for k, v in table.items() if k != "entries"}
    lines = [json.dumps(e, separators=(",", ":"), sort_keys=True) for e in table["entries"]]
    return json.dumps(head, indent=1, sort_keys=True)[:-2] + ',\n "entries": [\n' + ",\n".join(lines) + "\n ]\n}\n"


def check() -> int:
    problems: list[str] = []
    t = json.loads(TABLE.read_text(encoding="utf-8"))
    src = t.get("generated_from") or {}
    if t.get("schema") != 1:
        problems.append("`schema` is not 1")
    if src.get("repo") != REPO or src.get("path") != PATH or src.get("licence") != "MIT":
        problems.append("`generated_from` does not name Ruff's source file and its MIT licence")
    if not re.fullmatch(r"[0-9a-f]{40}", str(src.get("commit", ""))) or not re.fullmatch(r"[0-9a-f]{64}", str(src.get("sha256", ""))) or not src.get("tag"):
        problems.append("`generated_from` needs `tag`, a 40-hex `commit` and a 64-hex `sha256`")
    entries = t.get("entries")
    if not isinstance(entries, list) or not entries:
        problems.append("`entries` is empty")
        entries = []
    seen = set()
    last = None
    for e in entries:
        ok = isinstance(e, dict) and {"module", "name", "target", "since"} <= set(e) and set(e) <= {"module", "name", "target", "since", "until", "rename"}
        if not ok:
            problems.append(f"malformed entry {e!r}")
            continue
        if not VERSION.match(e["since"]) or ("until" in e and not VERSION.match(e["until"])):
            problems.append(f"{e['module']}.{e['name']}: a version is not `3.N`")
            continue
        ident = (e["module"], e["name"], e["target"], e["since"])
        if ident in seen:
            problems.append(f"{e['module']}.{e['name']} -> {e['target']}: twice")
        seen.add(ident)
        order = (e["module"], e["name"], e["target"], key(e["since"]))
        if last is not None and order < last:
            problems.append(f"{e['module']}.{e['name']}: not in order")
        last = order
    if TABLE.read_text(encoding="utf-8") != render(t):
        problems.append("the file is not the script's rendering of itself; run --refresh")
    for p in problems[:50]:
        print(f"ruff-moved-names: {p}", file=sys.stderr)
    if problems:
        print(f"ruff-moved-names: {len(problems)} problem(s)", file=sys.stderr)
        return 1
    print(f"✔ ruff-moved-names: {len(entries)} entries, from Ruff {src['tag']} ({src['commit'][:10]})")
    return 0


def source_at(tag: str, commit: str | None) -> tuple[bytes, str]:
    if commit is None:
        meta = json.loads(fetch(f"https://api.github.com/repos/{REPO}/git/ref/tags/{tag}"))
        commit = meta["object"]["sha"]
        if meta["object"]["type"] == "tag":
            commit = json.loads(fetch(meta["object"]["url"]))["object"]["sha"]
    return fetch(f"https://raw.githubusercontent.com/{REPO}/{commit}/{PATH}"), commit


def refresh(tag: str, from_source: str | None) -> int:
    if from_source:
        raw, commit = Path(from_source).read_bytes(), PIN_COMMIT if tag == PIN_TAG else ""
        if not commit:
            raise SystemExit("ruff-moved-names: --from-source needs the pinned --tag (its commit is the pin's)")
    else:
        raw, commit = source_at(tag, PIN_COMMIT if tag == PIN_TAG else None)
    TABLE.write_text(render(build(raw, tag, commit)), encoding="utf-8")
    return check()


def verify(from_source: str | None) -> int:
    t = json.loads(TABLE.read_text(encoding="utf-8"))
    src = t["generated_from"]
    raw = Path(from_source).read_bytes() if from_source else source_at(src["tag"], src["commit"])[0]
    want = render(build(raw, src["tag"], src["commit"]))
    if TABLE.read_text(encoding="utf-8") != want:
        print("ruff-moved-names: the table is not what the pinned source gives; run --refresh", file=sys.stderr)
        return 1
    print(f"✔ ruff-moved-names: the table is what Ruff {src['tag']} gives")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--check", action="store_true")
    g.add_argument("--verify", action="store_true")
    g.add_argument("--refresh", action="store_true")
    ap.add_argument("--tag", default=PIN_TAG)
    ap.add_argument("--from-source", metavar="FILE")
    a = ap.parse_args()
    if a.check:
        return check()
    if a.verify:
        return verify(a.from_source)
    return refresh(a.tag, a.from_source)


if __name__ == "__main__":
    sys.exit(main())
