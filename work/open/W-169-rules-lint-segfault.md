---
type: Handoff
name: W-169
description: "`codelaw rules lint packs/<pack>/` segfaults in CI now and then, with nothing printed first — on linux-arm64-musl (js) and x64 (react); the same commit passes on another run."
item: W-169
filed: 2026-11-05
filed_by: "Claude Code (session 0a172e1f)"
updated: 2026-11-05
updated_by: "Claude Code (session 0a172e1f)"
ball: agent
---

# W-169 — `rules lint` segfaults in CI, intermittently

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Capture it: run the build legs' `rules lint` under `ulimit -c unlimited` and upload the core and a backtrace (or `RUST_BACKTRACE=full` plus a SIGSEGV handler) as an artifact on failure | the crashing frame, not a guess | ⏳ |
| 2 | Fix the cause the backtrace shows; a regression test where it can be reproduced | release builds that do not crash | ⏳ |
| 3 | Five green runs of every build leg in a row | proof it is gone, not hidden | ⏳ |

## What is known

- Run 42053908300 (main `3b53d1a8`, `workflow_dispatch`): `build linux-arm64-musl`, `codelaw rules lint packs/js/` → `Segmentation fault (core dumped)`, exit 139, right after js's `codelaw test` passed (job 127337006685).
- The same commit **passed** that leg in push run 42053750244; an x64 leg crashed on `react` once, again with no output first.
- Ruled out by reading (the main-green fix, 2026-11-05): the new regex parser (`regex_ast.rs`) is not on `rules lint`'s path; `check`'s workers already run on 64 MB stacks; the work needs under 1 MB of stack. So a stack overflow is unlikely; a data race or an FFI fault (tree-sitter grammars, the allocator) is the next suspect.
- Effect: a red build leg skips `perf` (until W-126's `aed60b27` is on GitHub) and the corpus jobs, and would fail a release.

## Record

[SR-PACKAGES](../../records/0107_packages.md) (the build legs); the crash's own record once the cause is known.
