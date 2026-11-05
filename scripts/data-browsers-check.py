#!/usr/bin/env python3
"""The browser-support table's checks (L6 decision 15; SR-RULE-FORMAT
decision 2c; W-166). Standard library only (L11).

    python3 scripts/data-browsers-check.py --check
        Offline: data-packages/browsers/data/browsers.json is schema 1, its
        version is YYYY.M.D, every row names browsers of the closed set with
        versions, and cites at least one source, none of them caniuse or MDN's
        browser-compat-data (L2). CI's lint job runs it.

    python3 scripts/data-browsers-check.py --review <out.md>
        Online, CI's only: reads each browser's current stable release from
        its vendor (Chrome's release dashboard, Mozilla's product details,
        WebKit's blog feed) and writes the rows to review: every row that
        leaves a browser out, and every browser that has shipped a release
        since the table's version date. Exit 0 always; an empty review writes
        nothing. data-browsers.yml opens a pull request with it.
"""

from __future__ import annotations

import datetime
import json
import re
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TABLE = ROOT / "data-packages" / "browsers" / "data" / "browsers.json"
BROWSERS = ["chrome", "edge", "firefox", "safari", "ios_saf", "samsung"]
REFUSED = ("caniuse.com", "github.com/mdn/browser-compat-data", "browser-compat-data", "developer.mozilla.org/en-US/docs/Web/")
VERSION = re.compile(r"^\d+(\.\d+)*$")


def load() -> dict:
    return json.loads(TABLE.read_text(encoding="utf-8"))


def check() -> int:
    problems: list[str] = []
    t = load()
    if t.get("schema") != 1:
        problems.append("`schema` is not 1")
    if not re.fullmatch(r"\d{4}\.\d{1,2}\.\d{1,2}", str(t.get("version", ""))):
        problems.append(f"`version` {t.get('version')!r} is not YYYY.M.D")
    features = t.get("features")
    if not isinstance(features, dict) or not features:
        problems.append("`features` is empty")
        features = {}
    for fid, row in sorted(features.items()):
        if not re.fullmatch(r"[a-z0-9-]+:[^\s]+", fid):
            problems.append(f"{fid}: a feature id is `<kind>:<name>`")
        since = row.get("since") or {}
        if not since:
            problems.append(f"{fid}: no `since`")
        for b, v in since.items():
            if b not in BROWSERS:
                problems.append(f"{fid}: `{b}` is not one of {', '.join(BROWSERS)}")
            if not VERSION.match(str(v)):
                problems.append(f"{fid}: {b} `{v}` is not a version")
        sources = row.get("sources") or []
        if not sources:
            problems.append(f"{fid}: no `sources` — every row cites the vendor's notes or the spec")
        for s in sources:
            if not str(s).startswith("https://"):
                problems.append(f"{fid}: source {s!r} is not an https URL")
            if any(r in str(s) for r in REFUSED):
                problems.append(f"{fid}: source {s} is caniuse or MDN's compat data, which codelaw does not use (L2)")
    for p in problems:
        print(f"✘ data-browsers: {p}")
    if problems:
        return 1
    print(f"✔ data-browsers: {len(features)} rows, version {t['version']}")
    return 0


def fetch(url: str) -> str:
    req = urllib.request.Request(url, headers={"User-Agent": "codelaw-data-browsers-check"})
    with urllib.request.urlopen(req, timeout=30) as r:  # noqa: S310 - fixed vendor URLs
        return r.read().decode("utf-8", "replace")


def releases() -> dict[str, str]:
    """Each browser's current stable version, from its vendor; a browser
    whose source fails is left out and named in the review."""
    out: dict[str, str] = {}
    try:
        rel = json.loads(fetch("https://chromiumdash.appspot.com/fetch_releases?channel=Stable&platform=Windows&num=1"))
        out["chrome"] = rel[0]["version"].split(".")[0]
    except Exception as e:  # noqa: BLE001 - a review names what it could not read
        out["chrome?"] = str(e)
    try:
        out["firefox"] = json.loads(fetch("https://product-details.mozilla.org/1.0/firefox_versions.json"))["LATEST_FIREFOX_VERSION"].split(".")[0]
    except Exception as e:  # noqa: BLE001
        out["firefox?"] = str(e)
    try:
        titles = re.findall(r"<title>([^<]*Safari (\d+(?:\.\d+)?)[^<]*)</title>", fetch("https://webkit.org/blog/feed/"))
        if titles:
            out["safari"] = max((t[1] for t in titles), key=lambda v: [int(x) for x in v.split(".")])
    except Exception as e:  # noqa: BLE001
        out["safari?"] = str(e)
    return out


def review(path: Path) -> int:
    t = load()
    date = datetime.date(*[int(x) for x in t["version"].split(".")])
    rel = releases()
    lines: list[str] = []
    missing = [(fid, [b for b in BROWSERS if b not in row.get("since", {})]) for fid, row in sorted(t["features"].items())]
    missing = [(fid, bs) for fid, bs in missing if bs]
    if missing:
        lines += ["## Rows that leave a browser out", "", "A browser a row leaves out keeps that fix a suggestion. Check its release notes and add the version, with the source.", ""]
        lines += [f"- `{fid}`: {', '.join(bs)}" for fid, bs in missing]
        lines.append("")
    lines += ["## Current stable releases", ""]
    for b, v in sorted(rel.items()):
        lines.append(f"- {b.rstrip('?')}: {'could not read: ' + v if b.endswith('?') else v}")
    lines += ["", f"The table's data is dated {date.isoformat()}. Rows to review: any feature a release since then unprefixed — read each release's notes; never caniuse or MDN (L2)."]
    body = "# Browser-support table: rows to review\n\n" + "\n".join(lines) + "\n"
    old = path.read_text(encoding="utf-8") if path.exists() else None
    if old != body:
        path.write_text(body, encoding="utf-8")
        print(f"wrote {path}")
    return 0


def main(argv: list[str]) -> int:
    if argv[:1] == ["--check"]:
        return check()
    if argv[:1] == ["--review"] and len(argv) == 2:
        return review(Path(argv[1]))
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
