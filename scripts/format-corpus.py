#!/usr/bin/env python3
"""format-corpus — fetch W-141's pinned real-world corpus for `format-parity.py`.

    python3 scripts/format-corpus.py [--dir ~/.cache/codelaw-format-corpus]
    python3 scripts/format-parity.py --binary target/release/codelaw --corpus <dir>/files

The corpus is real projects' files, not codelaw's own: every source below is
pinned (an npm version or a git commit), fetched once into a cache directory
outside the repository and never vendored (their licences stay with them).
Up to `PER_LANGUAGE` files of each language are taken, evenly spaced through
the sorted list, so a second run picks the same files. Minified files and
files over 200 KB are left out. The fetch needs the network and `npm` and
`git`; `format-parity.py` then needs only the cache.
"""
import argparse
import pathlib
import shutil
import subprocess
import sys
import tarfile

PER_LANGUAGE = 150

# name -> (kind, where, pin, licence)
SOURCES = {
    "bootstrap": ("npm", "bootstrap", "5.3.8", "MIT"),
    "bulma": ("npm", "bulma", "1.0.4", "MIT"),
    "primer": ("npm", "@primer/css", "22.3.3", "MIT"),
    "learning-area": ("git", "https://github.com/mdn/learning-area", "21973263ba859ff3477919087b488157f9d618a6", "CC0-1.0"),
    "starter-workflows": ("git", "https://github.com/actions/starter-workflows", "e3c451d60f119b71caebf13c98ac45da6e15b4b7", "MIT"),
    "eslint": ("git", "https://github.com/eslint/eslint", "8781e6f063e56438dc22346504ff637df3f84daf", "MIT"),
    "typescript-eslint": ("git", "https://github.com/typescript-eslint/typescript-eslint", "4666ed4e0c4b6d4d57aee85f4f1962f914dd53b3", "MIT"),
}
# Only these paths are checked out of each git source.
SPARSE = {
    "learning-area": ["*.html", "*.css"],
    "starter-workflows": ["*.yml", "*.yaml"],
    "eslint": ["/lib/rules/*.js", "/docs/src/rules/*.md"],
    "typescript-eslint": ["/packages/eslint-plugin/src/rules/*.ts", "/packages/*/README.md", "/docs/**/*.md"],
}
LANGUAGES = {"css": "css", "scss": "scss", "html": "html", "yml": "yaml", "yaml": "yaml", "js": "js", "ts": "ts", "md": "md"}


def run(cmd, cwd=None):
    subprocess.run(cmd, cwd=cwd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def fetch(name, kind, where, pin, dest: pathlib.Path) -> None:
    if dest.exists():
        return
    tmp = dest.with_name(dest.name + ".part")
    shutil.rmtree(tmp, ignore_errors=True)
    tmp.mkdir(parents=True)
    if kind == "npm":
        out = subprocess.run(["npm", "pack", f"{where}@{pin}", "--silent"], cwd=tmp, check=True, capture_output=True, text=True).stdout.split()[-1]
        with tarfile.open(tmp / out) as t:
            t.extractall(tmp, filter="data")
        (tmp / out).unlink()
    else:
        run(["git", "init", "-q"], tmp)
        run(["git", "remote", "add", "origin", where], tmp)
        run(["git", "sparse-checkout", "set", "--no-cone", *SPARSE[name]], tmp)
        run(["git", "fetch", "-q", "--depth", "1", "--filter=blob:none", "origin", pin], tmp)
        run(["git", "checkout", "-q", "FETCH_HEAD"], tmp)
        shutil.rmtree(tmp / ".git")
    tmp.rename(dest)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--dir", default=str(pathlib.Path.home() / ".cache" / "codelaw-format-corpus"))
    args = ap.parse_args()
    root = pathlib.Path(args.dir).expanduser()
    for name, (kind, where, pin, _) in SOURCES.items():
        print(f"fetching {name} {pin}", file=sys.stderr)
        fetch(name, kind, where, pin, root / "sources" / name)
    by_language: dict[str, list[pathlib.Path]] = {}
    for p in sorted((root / "sources").rglob("*")):
        ext = p.suffix.lstrip(".")
        if not p.is_file() or ext not in LANGUAGES or ".min." in p.name or p.stat().st_size > 200_000:
            continue
        if any(part in ("node_modules", "fixtures", "tests") for part in p.parts):
            continue
        by_language.setdefault(LANGUAGES[ext], []).append(p)
    files = root / "files"
    shutil.rmtree(files, ignore_errors=True)
    files.mkdir()
    manifest = []
    for lang, paths in sorted(by_language.items()):
        step = max(1, len(paths) / PER_LANGUAGE)
        chosen = [paths[int(i * step)] for i in range(min(PER_LANGUAGE, len(paths)))]
        for i, p in enumerate(chosen):
            rel = p.relative_to(root / "sources")
            shutil.copyfile(p, files / f"{lang}-{i:03d}{p.suffix}")
            manifest.append(f"{lang}-{i:03d}{p.suffix}\t{rel}")
        print(f"{lang}: {len(chosen)} of {len(paths)}", file=sys.stderr)
    (root / "MANIFEST.tsv").write_text("\n".join(manifest) + "\n", encoding="utf-8")
    print(files)
    return 0


if __name__ == "__main__":
    sys.exit(main())
