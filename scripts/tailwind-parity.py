#!/usr/bin/env python3
"""tailwind-parity.py — W-145's DoD 3 (SR-FORMATTER decision 9): codelaw's
Tailwind class sorting against prettier-plugin-tailwindcss, file by file.

The instrument is L11's D-49 (`prettier-plugin-tailwindcss@0.8.1` with
`prettier@3.9.10`) and the Tailwind is D-48's (`tailwindcss@4.3.3`); they are
used by hand or in CI only, never by the engine. `--tools` is a folder whose
`node_modules` holds those three; this script installs nothing.

    python3 scripts/tailwind-parity.py --binary target/debug/codelaw \\
        --tools <folder with node_modules/{prettier,prettier-plugin-tailwindcss,tailwindcss}> \\
        --corpus <checkout> [--corpus <checkout> …]

Each corpus file (`.js .jsx .ts .tsx .html`, `node_modules` left out) is
formatted by Prettier without the plugin (P0) and with it (P1). Two numbers:

- **sorting** — codelaw sorts the class lists of P0 and nothing else
  (`codelaw debug sort-classes`); identical to P1 is the sorter's parity, the
  layouts being the same by construction. This is DoD 3's measure.
- **format** — `codelaw format` with sorting on the original, against P1: the
  whole formatter, so the engines' own layout gaps (SR-FORMATTER's tables)
  count too. Printed for context.

Both tools use Tailwind's default theme (`theme.css`), the stylesheet the
plugin uses when it is given none, so they ask one design system. The class
functions are codelaw's defaults (`clsx`, `cn`, `cva`, `tw`), given to the
plugin as `tailwindFunctions`. Standard library only.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import shutil
import subprocess
import sys
import tempfile

EXTS = (".js", ".jsx", ".ts", ".tsx", ".html")
FUNCTIONS = ["clsx", "cn", "cva", "tw"]
ROOT = pathlib.Path(__file__).resolve().parent.parent

PRETTIER = r"""
import fs from "node:fs";
import path from "node:path";
import * as prettier from "prettier";
const [list, out0, out1, stylesheet, functions] = process.argv.slice(2);
const plugin = await import("prettier-plugin-tailwindcss");
let failed = 0;
for (const rel of fs.readFileSync(list, "utf8").split("\n").filter(Boolean)) {
  const src = path.join(out0, rel);
  const text = fs.readFileSync(src, "utf8");
  try {
    const p0 = await prettier.format(text, { filepath: src });
    const p1 = await prettier.format(text, { filepath: src, plugins: [plugin.default ?? plugin], tailwindStylesheet: stylesheet, tailwindFunctions: JSON.parse(functions) });
    fs.writeFileSync(src, p0);
    fs.mkdirSync(path.dirname(path.join(out1, rel)), { recursive: true });
    fs.writeFileSync(path.join(out1, rel), p1);
  } catch {
    fs.rmSync(src);
    failed++;
  }
}
console.log(JSON.stringify({ failed }));
"""


def files_of(corpus: pathlib.Path) -> list[pathlib.Path]:
    out = []
    for dirpath, dirnames, filenames in os.walk(corpus):
        dirnames[:] = sorted(d for d in dirnames if d != "node_modules" and not d.startswith("."))
        for f in sorted(filenames):
            if f.endswith(EXTS) and not f.endswith(".d.ts") and not f.endswith(".min.js"):
                out.append(pathlib.Path(dirpath, f).relative_to(corpus))
    return out


def link_tools(work: pathlib.Path, tools: pathlib.Path) -> None:
    nm = work / "node_modules"
    (nm / "@codelaw").mkdir(parents=True)
    for name in ("prettier", "prettier-plugin-tailwindcss", "tailwindcss"):
        (nm / name).symlink_to(tools / "node_modules" / name)
    (nm / "@codelaw" / "provider-tailwind").symlink_to(ROOT / "packaging" / "npm" / "provider-tailwind")
    (nm / "@codelaw" / "rules-tailwind").symlink_to(ROOT / "packs" / "tailwind")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--binary", required=True)
    ap.add_argument("--tools", required=True)
    ap.add_argument("--corpus", action="append", required=True)
    ap.add_argument("--keep", help="keep the work folder here, to read the differences")
    a = ap.parse_args()
    binary = pathlib.Path(a.binary).resolve()
    tools = pathlib.Path(a.tools).resolve()
    work = pathlib.Path(a.keep).resolve() if a.keep else pathlib.Path(tempfile.mkdtemp(prefix="tailwind-parity-"))
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True)
    link_tools(work, tools)
    stylesheet = str(work / "node_modules" / "tailwindcss" / "theme.css")
    toml = 'extends = ["codelaw:tailwind/recommended"]\npreview = true\n[providers.tailwind]\nstylesheet = "node_modules/tailwindcss/theme.css"\n'
    rows = []
    for c in a.corpus:
        corpus = pathlib.Path(c).resolve()
        name = corpus.name
        p0, p1, orig = work / name / "p0", work / name / "p1", work / name / "orig"
        rels = files_of(corpus)
        for rel in rels:
            for d in (p0, orig):
                (d / rel).parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(corpus / rel, d / rel)
        for d in (p0, orig):
            (d / "codelaw.toml").write_text(toml.replace("node_modules/", "../../node_modules/"))
        lst = work / name / "files.txt"
        lst.write_text("\n".join(r.as_posix() for r in rels) + "\n")
        script = work / "prettier.mjs"
        script.write_text(PRETTIER)
        r = subprocess.run(["node", str(script), str(lst), str(p0), str(p1), stylesheet, json.dumps(FUNCTIONS)], cwd=work, capture_output=True, text=True)
        if r.returncode != 0:
            print(r.stderr, file=sys.stderr)
            return 1
        failed = json.loads(r.stdout.strip().splitlines()[-1])["failed"]
        kept = [rel for rel in rels if (p1 / rel).is_file()]
        # how many files the plugin's sorting changes at all
        moved = sum(1 for rel in kept if (p0 / rel).read_bytes() != (p1 / rel).read_bytes())
        # sorting: P0 sorted by codelaw, against P1
        subprocess.run([str(binary), "debug", "sort-classes", "--write", *[str(p0 / rel) for rel in kept]], cwd=p0, check=True)
        # format: the original through codelaw format, against P1
        subprocess.run([str(binary), "format", "."], cwd=orig, capture_output=True)
        same_sort = [rel for rel in kept if (p0 / rel).read_bytes() == (p1 / rel).read_bytes()]
        same_fmt = [rel for rel in kept if (orig / rel).read_bytes() == (p1 / rel).read_bytes()]
        rows.append((name, len(kept), failed, moved, len(same_sort), len(same_fmt)))
        (work / name / "sort-differs.txt").write_text("\n".join(r.as_posix() for r in kept if r not in set(same_sort)) + "\n")
    print("| corpus | files | skipped (Prettier fails) | files the plugin re-sorts | sorting identical | format identical |")
    print("|---|--:|--:|--:|--:|--:|")
    for name, n, failed, moved, s, f in rows:
        pct = lambda k: f"{100.0 * k / n:.1f} %" if n else "—"
        print(f"| {name} | {n} | {failed} | {moved} | {s} ({pct(s)}) | {f} ({pct(f)}) |")
    print(f"\nwork folder: {work}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
