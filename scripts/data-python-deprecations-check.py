#!/usr/bin/env python3
"""The Python deprecation table's checks and its generator (L6 decision 15;
L11 D-54; W-187). Standard library only (L11).

    python3 scripts/data-python-deprecations-check.py --check
        Offline: data-packages/python-deprecations/data/python-deprecations.json
        is schema 1, its version is YYYY.M.D, `generated_from` names a CPython
        release tag and the commit it points at, and the rows are well-formed:
        every table's names are dotted and sorted, every row is
        [deprecated-in, removed-in-or-null] with versions `3.N` and the
        removal after the deprecation, every argument row is
        [name, position-or-null, deprecated-in, removed-in-or-null]. CI's lint
        job runs it.

    python3 scripts/data-python-deprecations-check.py --refresh [--tag T] [--from-tarball FILE]
        Online (CI's weekly job; by hand with --from-tarball, offline): reads
        the `Doc/library/*.rst` files of CPython's release tag (the newest
        stable one, or T) from GitHub's tarball of it and writes the table
        from their `.. deprecated::` / `.. deprecated-removed::` directives
        and `:deprecated:` module options. Nothing of CPython is installed or
        executed, and no prose is copied: only a name and two version numbers
        per row. The table's version is today's date when its rows changed,
        and stays when they did not. Exit 0.

What is read, and what is not (the rules in `classify`): a directive counts
when it stands in the block of the name it deprecates AND its text says the
whole name is deprecated. A directive that deprecates one way of calling
(`Passing *x* positionally is deprecated`, `Deprecation warning is emitted
if …`), or that belongs to a note about another name, is not a row; one that
deprecates a single named parameter is an argument row. The names the PEP 585
notes in `typing.rst` list (`typing.Dict` …) are left to `py/deprecated-
typing-alias`.
"""

from __future__ import annotations

import argparse
import datetime
import io
import json
import re
import sys
import tarfile
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TABLE = ROOT / "data-packages" / "python-deprecations" / "data" / "python-deprecations.json"
VERSION = re.compile(r"^3\.\d+$")
TAG = re.compile(r"^v3\.\d+\.\d+$")
SOURCE = "CPython documentation"
REPO = "python/cpython"
TABLES = ("modules", "classes", "functions", "attributes", "decorators")

DIRECTIVE = re.compile(
    r"^(?P<ind> *)\.\. (?:py:)?(?P<kind>module|currentmodule|function|class|exception|method|classmethod|staticmethod|attribute|data|decorator|decoratormethod|deprecated|deprecated-removed)::[ \t]*(?P<arg>.*)$"
)
KIND_TABLE = {
    "function": "functions",
    "method": "functions",
    "classmethod": "functions",
    "staticmethod": "functions",
    "class": "classes",
    "exception": "classes",
    "attribute": "attributes",
    "data": "attributes",
    "decorator": "decorators",
    "decoratormethod": "decorators",
}

# A directive that deprecates one way of using a name, not the name.
PARTIAL = re.compile(
    r"\b(?:if|when|without|Passing|Accepting|Calling|Opening|Setting|Invalid|emitted|positional|keyword|instances?)\b"
    r"|\bAll\b[^.]*\b(?:members|options)\b|\bIt is deprecated to\b|\bwill not be called\b|\bparameters?\b|\barguments?\b"
)
# "`:class:`X` is deprecated`": the name the sentence is about.
SUBJECT = re.compile(r":(?:class|func|meth|data|attr|exc|mod):`[!~.]*([\w.]+)`[^.]*?\b(?:is|are|has been|have been)\b (?:now |also )?deprecated")
PARAM_SENTENCE = (
    re.compile(r"^The \*(\w+)\* (?:parameter|argument)s?\b"),
    re.compile(r"^Passing \*?`{0,2}(\w+?)_?\*?`{0,2} to :\w+:`[^`]+` is (?:now )?deprecated"),
)
# The names `py/deprecated-typing-alias` owns: PEP 585 aliases.
PEP585 = re.compile(r"now supports? (?:subscripting|`*\[\]`*)")
# Rows the reading above gets wrong, each with the reason (read against the
# 3.15 documentation). Keep this list short: a row that needs an entry here
# is a sign the reading needs a rule.
EXCLUDE: dict[str, str] = {}


def load() -> dict:
    return json.loads(TABLE.read_text(encoding="utf-8"))


def ver_key(v: str) -> tuple[int, int]:
    a, b = v.split(".")
    return int(a), int(b)


def check() -> int:
    problems: list[str] = []
    t = load()
    if t.get("schema") != 1:
        problems.append("`schema` is not 1")
    if not re.fullmatch(r"\d{4}\.\d{1,2}\.\d{1,2}", str(t.get("version", ""))):
        problems.append(f"`version` {t.get('version')!r} is not YYYY.M.D")
    src = t.get("generated_from") or {}
    if src.get("source") != SOURCE or not TAG.match(str(src.get("tag", ""))) or not re.fullmatch(r"[0-9a-f]{40}", str(src.get("commit", ""))):
        problems.append("`generated_from` is not {source: CPython documentation, tag: vX.Y.Z, commit: <40 hex>}")
    counts = {}
    for name in TABLES:
        rows = t.get(name)
        if not isinstance(rows, dict) or not rows:
            problems.append(f"`{name}` is empty")
            continue
        counts[name] = len(rows)
        if list(rows) != sorted(rows):
            problems.append(f"`{name}` is not sorted by name")
        for key, row in rows.items():
            if not re.fullmatch(r"[A-Za-z_][\w]*(\.[A-Za-z_][\w]*)*", key):
                problems.append(f"{name}: {key!r} is not a dotted name")
            if not (isinstance(row, list) and len(row) == 2 and VERSION.match(str(row[0])) and (row[1] is None or VERSION.match(str(row[1])))):
                problems.append(f"{name}: {key}: not [deprecated-in, removed-in-or-null]")
            elif row[1] is not None and ver_key(row[1]) < ver_key(row[0]):
                problems.append(f"{name}: {key}: removed before it was deprecated")
    args = t.get("arguments")
    if not isinstance(args, dict) or not args:
        problems.append("`arguments` is empty")
        args = {}
    if list(args) != sorted(args):
        problems.append("`arguments` is not sorted by name")
    for key, rows in args.items():
        if not re.fullmatch(r"[A-Za-z_][\w]*(\.[A-Za-z_][\w]*)*", key):
            problems.append(f"arguments: {key!r} is not a dotted name")
        for row in rows if isinstance(rows, list) else []:
            if not (
                isinstance(row, list)
                and len(row) == 4
                and re.fullmatch(r"[A-Za-z_]\w*", str(row[0]))
                and (row[1] is None or (isinstance(row[1], int) and row[1] >= 0))
                and VERSION.match(str(row[2]))
                and (row[3] is None or VERSION.match(str(row[3])))
            ):
                problems.append(f"arguments: {key}: {row!r} is not [name, position-or-null, deprecated-in, removed-in-or-null]")
        if not isinstance(rows, list) or not rows:
            problems.append(f"arguments: {key}: no rows")
    for p in problems[:50]:
        print(f"data-python-deprecations: {p}", file=sys.stderr)
    if problems:
        print(f"data-python-deprecations: {len(problems)} problem(s)", file=sys.stderr)
        return 1
    shown = ", ".join(f"{n} {c}" for n, c in counts.items())
    print(f"✔ data-python-deprecations {t['version']}: {shown}, arguments {len(args)}, from {SOURCE} {src['tag']}")
    return 0


def fetch(url: str) -> bytes:
    req = urllib.request.Request(url, headers={"User-Agent": "codelaw-data-python-deprecations"})
    with urllib.request.urlopen(req, timeout=120) as r:  # noqa: S310 — GitHub, CI only (L5)
        return r.read()


# ---------------------------------------------------------------- reading

def join_signature(lines: list[str], i: int) -> str:
    """The directive's argument, with a backslash continuation joined."""
    arg = DIRECTIVE.match(lines[i]).group("arg").rstrip()  # type: ignore[union-attr]
    while arg.endswith("\\") and i + 1 < len(lines):
        i += 1
        arg = arg[:-1].rstrip() + " " + lines[i].strip().rstrip()
    return arg


def split_params(sig: str) -> list[str] | None:
    """The parameter names of `name(a, b=1, *, c)`, positional ones first: a
    name after a bare `*` or `*args` is keyword-only, marked by a `*` prefix.
    `None` when there is no parameter list."""
    open_ = sig.find("(")
    if open_ < 0:
        return None
    depth, cur, parts = 0, "", []
    for ch in sig[open_ + 1 :]:
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            if depth == 0:
                break
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    out, kwonly = [], False
    for p in parts:
        if p in ("/",):
            continue
        if p == "*" or (p.startswith("*") and not p.startswith("**")):
            kwonly = True
            continue
        if p.startswith("**"):
            continue
        m = re.match(r"[A-Za-z_]\w*", p)
        if m:
            out.append(("*" if kwonly else "") + m.group(0))
    return out


def body_of(lines: list[str], i: int, indent: int) -> str:
    """The first paragraph of the directive's text, whitespace joined."""
    body: list[str] = []
    for line in lines[i + 1 : i + 16]:
        if not line.strip():
            if body:
                break
            continue
        if len(line) - len(line.lstrip(" ")) <= indent:
            break
        body.append(line.strip())
    return " ".join(body)


def parse(name: str, text: str, out: dict) -> None:
    lines = text.split("\n")
    module: str | None = None
    stack: list[tuple[int, str, str]] = []  # (indent, kind, argument)
    pending_module: str | None = None  # a `:deprecated:` module, until its notice
    classes = {re.match(r"[A-Za-z_]\w*", m.group("arg")).group(0) for m in map(DIRECTIVE.match, lines) if m and m.group("kind") in ("class", "exception") and re.match(r"[A-Za-z_]", m.group("arg"))}  # type: ignore[union-attr]
    for i, line in enumerate(lines):
        if not line.strip():
            continue
        k = len(line) - len(line.lstrip(" "))
        m = DIRECTIVE.match(line)
        kind = m.group("kind") if m else None
        if kind in ("deprecated", "deprecated-removed"):
            vers = m.group("arg").split()  # type: ignore[union-attr]
            if not vers or not all(VERSION.match(v) for v in vers):
                continue
            since, removed = vers[0], (vers[1] if kind == "deprecated-removed" and len(vers) > 1 else None)
            if k == 0 and pending_module:
                put(out["modules"], pending_module, since, removed)
                pending_module = None
                continue
            while stack and stack[-1][0] >= k:
                stack.pop()
            if not stack or module is None:
                continue
            ind, okind, oarg = stack[-1]
            if okind == "module" or okind not in KIND_TABLE:
                continue
            owner = qualify(module, stack, oarg, classes)
            if owner is None or owner in EXCLUDE:
                continue
            body = body_of(lines, i, k)
            for pat in PARAM_SENTENCE:
                pm = pat.match(body)
                if pm and okind in ("function", "method", "classmethod", "staticmethod"):
                    sig = split_params(join_signature(lines, line_of(lines, ind, okind, oarg, i)))
                    pname = pm.group(1)
                    pos = None
                    if sig is not None and pname in sig:
                        pos = sig.index(pname)
                    out["arguments"].setdefault(owner, {})[pname] = (pname, pos, since, removed)
                    break
            else:
                if classify(owner, body):
                    put(out[KIND_TABLE[okind]], owner, since, removed)
            continue
        while stack and stack[-1][0] >= k:
            stack.pop()
        if not m:
            continue
        arg = m.group("arg").strip()
        if kind in ("module", "currentmodule"):
            module = arg
            pending_module = None
            if kind == "module":
                opts = lines[i + 1 : i + 8]
                if any(re.match(r"^\s+:deprecated:\s*$", o) for o in opts):
                    pending_module = arg
                stack.append((k, "module", arg))
            continue
        stack.append((k, kind, arg))  # type: ignore[arg-type]


def line_of(lines: list[str], ind: int, kind: str, arg: str, before: int) -> int:
    """The line of the directive that opened `arg`, searching upward."""
    for j in range(before, -1, -1):
        m = DIRECTIVE.match(lines[j])
        if m and m.group("kind") == kind and len(m.group("ind")) == ind and m.group("arg").strip() == arg:
            return j
    return before


def qualify(module: str, stack: list[tuple[int, str, str]], arg: str, classes: set[str]) -> str | None:
    nm = re.match(r"[A-Za-z_][\w.]*", arg)
    if not nm:
        return None
    name = nm.group(0)
    chain = []
    for _, kind, a in stack[:-1]:
        if kind in ("class", "exception"):
            cm = re.match(r"[A-Za-z_][\w.]*", a)
            if cm:
                chain.append(cm.group(0))
    if chain and name.startswith(chain[-1] + "."):
        name = name[len(chain[-1]) + 1 :]
    if name.split(".")[0] in classes:  # `datetime.utcnow` under `.. class:: datetime`
        return ".".join([module, *chain, name])
    if name.startswith(module + "."):
        return name
    return ".".join([module, *chain, name])


def classify(owner: str, body: str) -> bool:
    """Whether a directive's text deprecates the whole of `owner`."""
    if PEP585.search(body) and owner.startswith("typing."):
        return False
    if PARTIAL.search(body):
        return False
    sm = SUBJECT.search(body)
    if sm and sm.group(1).split(".")[-1] != owner.split(".")[-1]:
        return False
    return True


def put(table: dict, name: str, since: str, removed: str | None) -> None:
    old = table.get(name)
    if old is None or ver_key(since) < ver_key(old[0]):
        table[name] = [since, removed]


def build(tarball: bytes) -> tuple[dict, str]:
    out: dict = {n: {} for n in TABLES}
    out["arguments"] = {}
    with tarfile.open(fileobj=io.BytesIO(tarball), mode="r:gz") as tar:
        commit = str(tar.pax_headers.get("comment", ""))
        files = sorted((m for m in tar if re.search(r"/Doc/library/[^/]+\.rst$", m.name)), key=lambda m: m.name)
        for m in files:
            f = tar.extractfile(m)
            if f is not None:
                parse(m.name.rsplit("/", 1)[-1], f.read().decode("utf-8"), out)
    if not files:
        raise SystemExit("data-python-deprecations: the tarball has no Doc/library")
    out["arguments"] = {k: [list(v[n]) for n in sorted(v)] for k, v in sorted(out["arguments"].items())}
    for n in TABLES:
        out[n] = dict(sorted(out[n].items()))
    return out, commit


def newest_tag() -> str:
    tags = json.loads(fetch(f"https://api.github.com/repos/{REPO}/tags?per_page=100"))
    stable = [t["name"] for t in tags if TAG.match(t["name"])]
    if not stable:
        raise SystemExit("data-python-deprecations: no stable CPython tag in the newest hundred")
    return max(stable, key=lambda t: tuple(int(x) for x in t[1:].split(".")))


def refresh(tag: str | None, tarball_path: str | None) -> int:
    if tarball_path:
        raw = Path(tarball_path).read_bytes()
        m = re.search(r"cpython-(3\.\d+\.\d+)\.tar\.gz$", tarball_path)
        if not m:
            raise SystemExit("data-python-deprecations: --from-tarball wants a file named cpython-<version>.tar.gz")
        tag = "v" + m.group(1)
    else:
        tag = tag or newest_tag()
        if not TAG.match(tag):
            raise SystemExit(f"data-python-deprecations: {tag!r} is not a stable CPython tag (vX.Y.Z)")
        raw = fetch(f"https://codeload.github.com/{REPO}/tar.gz/refs/tags/{tag}")
    rows, commit = build(raw)
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        raise SystemExit("data-python-deprecations: the tarball names no commit")
    table = {"schema": 1, "version": "", "generated_from": {"source": SOURCE, "tag": tag, "commit": commit}, **rows}
    old = load() if TABLE.exists() else {}
    keys = ("generated_from", *TABLES, "arguments")
    same = all(old.get(k) == table[k] for k in keys)
    today = datetime.date.today()
    table["version"] = old["version"] if same and old.get("version") else f"{today.year}.{today.month}.{today.day}"
    TABLE.parent.mkdir(parents=True, exist_ok=True)
    TABLE.write_text(json.dumps(table, separators=(",", ":"), ensure_ascii=False) + "\n", encoding="utf-8")
    sizes = ", ".join(f"{n} {len(table[n])}" for n in (*TABLES, "arguments"))
    print(f"data-python-deprecations {table['version']}: {sizes} from {SOURCE} {tag}" + (" (unchanged)" if same else ""))
    return 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--refresh", action="store_true")
    ap.add_argument("--tag")
    ap.add_argument("--from-tarball")
    a = ap.parse_args()
    if a.refresh:
        return refresh(a.tag, a.from_tarball)
    return check()


if __name__ == "__main__":
    sys.exit(main())
