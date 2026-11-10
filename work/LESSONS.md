---
type: Log
description: "Dated lessons — what this project learned the hard way, newest first. A log, not a record: the date is part of the lesson."
---

## 2026-11-10 — a p95 over ten samples is the max, and the max was the second call

`mcp_rules_for_path` failed its 65 ms budget with a 17 ms median and one 170 ms sample. The bench discards the first call; the slow one was the *second*, every time (a 1 s pause between calls did not move it — not a background thread). A memory-saving rule (a pack is kept only from its second load, so one-shot runs keep nothing) made the second request in a warm process re-parse every pack, a cost that grows with the rule count and so only crossed the budget once ~360 rules landed. When one sample is slow, print them in order and sample the process (`/usr/bin/sample`, 1 ms, a debug build for symbols) during that call before reading code; and a "kept from the Nth use" cache rule is wrong for a process that is known to serve many requests.

## 2026-11-10 — a fail-open hook turns a slow debug build into a silent allow

`codelaw hook --pre` allows the write when it takes over 5 s, printing the reason on stderr, which a test that reads only stdout never sees. In a debug build the per-process compile of every enabled rule's regexes (unoptimised `regex-automata`) grew with each rule slice until a loaded CI runner crossed 5 s, and `bench`'s hook_pre failed as "not refused" with no sign of a timeout. When a fail-open path's test goes red with an empty stdout, read the stderr first and time the command; the cheap cure for debug-only cost is an `opt-level` on the hot dependency in `[profile.dev.package.*]`, not a longer timeout.

## 2026-11-07 — a shared cargo target hands you another worktree's binary

`target/debug/codelaw` is shared by every lane (MACHINE.md). Between your `cargo build` and your test run, another worktree's build can replace it, and the test then runs code your tree doesn't hold — once it passed a rule whose `Fixed` blocks still read `XX`, once the whole pack died with exit 137. **Copy the binary right after your own build and test with the copy**; a result that contradicts the code you just wrote is the binary first, the code second.

---

# LESSONS — what this build learned the hard way

Newest first. A lesson whose remedy is a *rule* belongs in a record and is
named here only as the failure that produced it. A lesson whose remedy is
*judgment* lives here. ⚠ Nothing here is normative.

---

## 2026-11-07 — `git stash` is one stack for every worktree; `pop` takes whoever's is on top

W-172 stashed its eight files to merge `main`, and while the merge ran W-162,
in another worktree, stashed too. `git stash pop` took the top entry — W-162's
(`On lm/W-162: w162-change5`, 40 pack files and 19 untracked ones) — applied it
in W-172's worktree and dropped it. Caught at once by `git status`; put back
with `git stash store` of the dropped commit before W-162 popped, and W-172's
own entry applied by its hash. **In a repository with parallel worktrees, never
`pop` by position: `git stash push -m <item>` and `git stash apply <hash>`, or
commit to the branch and merge instead** — lockmaster's end-of-turn landing
commits the claim anyway.

## 2026-11-07 — a rule nobody turns on still costs memory and time, once per process

W-139 grew the packs a project extends from ~440 rules to ~1,600, most of them
in no preset the project uses. Every one was parsed, compiled for each of its
grammars — ast-grep builds each `regex:` once per grammar, about 16 KB apiece —
and kept, and every file asked each one for its settings: peak memory 60 →
124 MiB and `rules_for_path` 40 → 129 ms on the reference runner, with no
engine commit to bisect to. **A per-rule cost scales with the packs, not with
the code: measure one binary against packs with and without the rules, and
read a heap profile by call site (`heap --sortBySize`, `malloc_history
-callTree -invert`), before bisecting.** Two packs from different weeks do not
load into each other's engine, so a commit bisect mixes both arms.

---

## 2026-10-26 — a `skip_serializing_if` on a cached type silently turns every entry into a miss

`Diagnostic.data` (W-61) was `skip_serializing_if = "Option::is_none"`; the cache's codec (W-67) is positional, so a skipped field shifts every later one and the entry never decodes. A corrupt entry reads as a miss by design, so nothing failed: warm runs re-linted every file with a finding and `check_warm_10k` went from 220 ms to 1.4 s. A cached type must serialize the same fields every time; the cache's test now round-trips a `Diagnostic` with and without `data`. Look at the cache hit/miss line of `--timings` first when a warm run slows.

## 2026-10-25 — a shared `CARGO_TARGET_DIR` does not make parallel worktrees safe either

Six agents built six worktrees into the one shared `target/`. Disk held, but
cargo names a workspace crate's artifacts the same in every worktree and judges
freshness by mtime, so a build reused another branch's compiled crates, and
`target/debug/codelaw` was swapped mid-test. Agents saw "method not found",
unknown rules and tests failing on another branch's binary; one `cargo clean`
pulled the folder from under a third session's test run. Every branch's green
tests were suspect until one clean run on the merge (555 passed). The lesson
of 2026-10-24 stands, for a second reason: **only one may build** — let the
others edit, then build each branch in turn, or test only on the merge.

## 2026-10-24 — seven parallel cargo builds fill this disk; one at a time

Seven agents, each in its own worktree with its own `target/`, built the
workspace at once. Each `target/` is 0.3–4 GB; with the main one they came to
about 27 GB and the disk hit 0 bytes free. Every agent stopped mid-commit, and
no shell command could run — not even `df` — until the decider freed space by
hand. Nothing was lost (the edits sat uncommitted in the worktrees), but six
of seven items needed a second pass to test and commit. **Parallel agents on
this machine may edit in parallel; only one may build.** Delete a finished
worktree's `target/` before starting the next build.

## 2026-10-24 — a `cfg`-split function is two functions; the other half is never compiled where you are

`bench/mod.rs` grew a `#[cfg(unix)]` `wait` that reads `rusage` and a
`#[cfg(not(unix))]` twin that returns the wrong type. Clippy, 435 tests and the
reviewer all passed it, because on a Mac the second function does not exist.
It landed on `main` and would have turned both Windows builds red on the first
push — no `dist`, no attestation, no release. **When a file gains a `cfg`
split, `cargo check --target x86_64-pc-windows-msvc` (or the other side of the
split) before merging is part of verification, not CI's job** — W-24 fixes
this one. The same shape hides anywhere one platform is the only one anyone
builds on: the review that found it read the code, not the test results.

## 2026-10-24 — two sessions, one checkout: a ruling can sit on a branch for days

The decider's 2026-10-22 rulings were built, verified and delivered as a
branch, unmerged, because another session had left uncommitted edits in the
main checkout; the next session found the queue still showing six inbox rows
and nearly re-did the templates. **Every session's first look is `git status`
and `git branch --no-merged main`, not just the queue** — an unmerged branch
named after a date or a ruling is work the queue does not know about.

## 2026-10-20 — a law with no gate gets walked past by the next release

L7 says the linter and formatter never ship apart, and L6 decision 3 says every
package carries the formatter, MCP server and LSP. The work was split sensibly —
lint first (W-02), packs (W-03), the pipeline (W-04) — and each item was green
on its own terms, while the formatter sat in the backlog as "L7 requires it".
Nothing mechanical stood between that and a publish, so the queue ended up
asking the decider to hand-publish 0.1.0. **Before a step that cannot be
undone (a publish), check every law against what will ship, and make the
check a gate** — `scripts/law-gate.py` is that gate for L6 and L7.

## 2026-10-20 — the slowest platform sets the pipeline's time; skip it, don't rush it

Eight platform builds run in parallel, so a push takes as long as Windows arm64
(~2½ min for one LTO build) whatever the other seven do. Faster runners were not
the lever: **not rebuilding was** — the wheel is cached under the hash of every
input that goes into it, so a push that changes no engine source (records,
docs, packs, `work/`) restores eight wheels instead of compiling them. The
binary's tests still run on every push; only the compile is skipped.

## 2026-10-19 — a rule's own examples don't find its false positives; a corpus does

All 58 first ports passed their examples, then a run over npm's source found
eight false-positive patterns (`this` inside an arrow passed to `super()`,
`[]` inside `[{[]`, ambient `declare` code, a tree-sitter misparse, `await`
read as an identifier). **Run a new rule over real code before trusting it**;
the examples only prove the cases the author thought of.

## 2026-10-19 — YAML keeps the last of two equal keys, silently

A rule written with two `has:` keys compiled with one of them gone, and its
broken half was never noticed until the duplicate was removed. Rule YAML now
refuses duplicate keys (SR-RULE-FORMAT decision 3d).

## 2026-10-19 — a benchmark that writes into the repo it checks measures itself

Timing `codelaw check > cold.json` inside the generated repo made the second
run check `cold.json` too, so "cold" and "warm" output differed and looked
like a cache bug. **Write a run's output outside the folder it checks** before
reading a difference as a finding.

## 2026-10-19 — rule YAML must be read as YAML 1.2

`serde-saphyr`'s default options read `message: n` as the boolean `false`
(YAML 1.1's "Norway problem"), so a one-letter message refused a rule. ast-grep
reads its rules as YAML 1.2; codelaw now does the same, and the rule is in
SR-RULE-FORMAT decision 3d. A test in `codelaw-rules` pins it.
