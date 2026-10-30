#!/usr/bin/env python3
"""typescript-watch — the TypeScript codelaw's type rules are tested on, kept current (SR-TYPES decision 7; W-105, was W-106).

    python3 scripts/typescript-watch.py latest [--version V] [--below 7.2]   # "newer <v>" or "current <pinned>"
    python3 scripts/typescript-watch.py bump <v>               # pin <v>; widen the tested range if <v> is past it

`latest` reads npm's dist-tags for `typescript` (one small read-only GET) and
takes the newest of `latest`, `rc` and `beta` that has the 7.1 API (>= 7.1,
its pre-releases included) and is newer than the version `fixtures/types`
pins. `next` (the nightly, `7.x.0-dev.<date>`) is never proposed: ci.yml's
nightly `types-next` job reports on it and blocks nothing. `--below 7.2` keeps
to the 7.1 line — ci.yml's `test` job asks it for the newest 7.1 release (a
range on npm would pick a nightly: `dev` sorts after `beta` and `rc`).

`bump` writes the exact pin into `fixtures/types/package.json` (the lock is
rewritten by `npm install` after it), and, when the version is past
`codelaw_types::TESTED`'s upper bound, moves that bound to the next minor —
in `crates/codelaw-types/src/lib.rs` and in SR-TYPES decision 7 — and names
the new pin in L11 §3 D-45. It prints one line saying which it did.
typescript-bump.yml runs both; the standard library only (L11).
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PACKAGE = ROOT / "fixtures/types/package.json"
LIB = ROOT / "crates/codelaw-types/src/lib.rs"
SR_TYPES = ROOT / "records/0117_types.md"
L11 = ROOT / "records/0013_LAW-11-dependencies.md"
DIST_TAGS = "https://registry.npmjs.org/-/package/typescript/dist-tags"
TAGS = ("latest", "rc", "beta")
TESTED_RE = re.compile(r'pub const TESTED: &str = "(>=[^,"]+), <(\d+)\.(\d+)\.0";')


def key(v: str) -> tuple:
    """A semver sort key: a release sorts after its pre-releases."""
    core, _, pre = v.partition("+")[0].partition("-")
    major, minor, patch = (int(x) for x in core.split("."))
    ids = tuple((0, int(p), "") if p.isdigit() else (1, 0, p) for p in pre.split(".")) if pre else ()
    return (major, minor, patch, 0 if pre else 1, ids)


def has_api(v: str) -> bool:
    return key(v)[:2] >= (7, 1)


def pinned() -> str:
    return json.loads(PACKAGE.read_text())["devDependencies"]["typescript"]


def latest(override: str | None, tags_json: str | None, below: str | None = None) -> None:
    have = pinned()
    if override:
        want = override
    else:
        if tags_json:
            tags = json.loads(Path(tags_json).read_text())
        else:
            import urllib.request

            with urllib.request.urlopen(DIST_TAGS, timeout=30) as r:
                tags = json.load(r)
        found = [tags[t] for t in TAGS if isinstance(tags.get(t), str) and has_api(tags[t])]
        if below:
            found = [v for v in found if key(v)[:2] < tuple(int(x) for x in below.split("."))[:2]]
        want = max(found, key=key) if found else have
    if key(want) > key(have) or (override and want != have):
        print(f"newer {want}")
    else:
        print(f"current {have}")


def bump(v: str) -> None:
    if not has_api(v):
        sys.exit(f"typescript-watch: {v} has no 7.1 API; the type rules cannot run on it (SR-TYPES decision 6)")
    old = pinned()
    pkg = json.loads(PACKAGE.read_text())
    pkg["devDependencies"]["typescript"] = v
    PACKAGE.write_text(json.dumps(pkg, indent=2, ensure_ascii=False) + "\n")

    lib = LIB.read_text()
    m = TESTED_RE.search(lib)
    if not m:
        sys.exit(f"typescript-watch: no TESTED range in {LIB.relative_to(ROOT)}")
    low, up_major, up_minor = m.group(1), int(m.group(2)), int(m.group(3))
    major, minor = key(v)[:2]
    old_range = f"{low}, <{up_major}.{up_minor}.0"
    widened = (major, minor) >= (up_major, up_minor)
    if widened:
        new_range = f"{low}, <{major}.{minor + 1}.0"
        LIB.write_text(lib.replace(f'"{old_range}"', f'"{new_range}"'))
        sr = SR_TYPES.read_text()
        if f"`{old_range}`" not in sr:
            sys.exit(f"typescript-watch: SR-TYPES decision 7 does not name `{old_range}`")
        SR_TYPES.write_text(sr.replace(f"`{old_range}`", f"`{new_range}`"))

    # L11 §3 D-45 names the exact pinned tree.
    law = L11.read_text().splitlines(keepends=True)
    for i, line in enumerate(law):
        if line.startswith("| D-45 |"):
            law[i] = line.replace(f"@{old}`", f"@{v}`")
    L11.write_text("".join(law))

    if widened:
        print(f"widened: typescript {old} → {v}; the tested range {old_range} → {new_range}")
    else:
        print(f"re-pinned: typescript {old} → {v}, inside the tested range {old_range}")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("latest", help="print `newer <v>` or `current <pinned>`")
    a.add_argument("--version", help="an exact release to propose instead of npm's newest")
    a.add_argument("--dist-tags", help="read the dist-tags from this JSON file instead of npm (tests)")
    a.add_argument("--below", help="only versions below this major.minor (`7.2`: the 7.1 line)")
    b = sub.add_parser("bump", help="pin <v>, widening the tested range when <v> is past it")
    b.add_argument("version")
    args = ap.parse_args()
    if args.cmd == "latest":
        latest(args.version or None, args.dist_tags, args.below)
    else:
        bump(args.version)


if __name__ == "__main__":
    main()
