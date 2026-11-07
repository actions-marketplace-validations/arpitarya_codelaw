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
| 1 | Capture it: run the build legs' `rules lint` under `ulimit -c unlimited` and upload the core and a backtrace (or `RUST_BACKTRACE=full` plus a SIGSEGV handler) as an artifact on failure | the crashing frame, not a guess | 🔨 built 11-05, unproven: `main` is not pushed, so no CI run has used it |
| 2 | Fix the cause the backtrace shows; a regression test where it can be reproduced | release builds that do not crash | ⏳ |
| 3 | Five green runs of every build leg in a row | proof it is gone, not hidden | ⏳ |

## What is known

- Run 42053908300 (main `3b53d1a8`, `workflow_dispatch`): `build linux-arm64-musl`, `codelaw rules lint packs/js/` → `Segmentation fault (core dumped)`, exit 139, right after js's `codelaw test` passed (job 127337006685).
- The same commit **passed** that leg in push run 42053750244; an x64 leg crashed on `react` once, again with no output first.
- Ruled out by reading (the main-green fix, 2026-11-05): the new regex parser (`regex_ast.rs`) is not on `rules lint`'s path; `check`'s workers already run on 64 MB stacks; the work needs under 1 MB of stack. So a stack overflow is unlikely; a data race or an FFI fault (tree-sitter grammars, the allocator) is the next suspect.
- **Local reproduction failed (2026-11-05, macOS arm64, M-series; one machine, not CI's):** see §Investigation.
- Effect: a red build leg skips `perf` (until W-126's `aed60b27` is on GitHub) and the corpus jobs, and would fail a release.

## Capture (step 1, built 2026-11-05)

`scripts/crash-capture.sh`, wired into `ci.yml`'s `build <key>`:

- **`arm`**, a step before the packs loop: every core goes to `$RUNNER_TEMP/crash` (`kernel.core_pattern` on Linux, `kern.corefile` on macOS, WER LocalDumps for `codelaw.exe` on Windows). The loop's step says `ulimit -c unlimited`.
- **`collect`**, `if: failure()`: copies the core and the shipped binary, builds an **unstripped twin** — the same `maturin build --release --locked --target … <flags>` with `strip = false`, never shipped — and writes `backtrace.txt`: `$_siginfo` (the fault address), registers, the faulting instructions and `thread apply all bt` from gdb (installed by apt if missing; a static musl core reads the same as a glibc one, so the musl legs need nothing else), or lldb plus macOS's `DiagnosticReports` crash report. On Linux it says whether the twin's `.text` is byte-identical to the shipped binary's — if it is, the names are the crash's. Windows keeps the minidump only.
- The artifact is **`crash-<key>`** (14 days); `backtrace.txt`'s first 60 KB also go to the job summary. The job's timeout went from 20 to 35 minutes for the twin's rebuild.
- **Unproven:** `main` is not on GitHub yet (the decider's push), so no run has armed it or caught a crash. The first red build leg after the push is the proof; until then the `arm` step's `sudo sysctl` on each runner image is untested.

## Investigation (step 2, 2026-11-05)

INVESTIGATION_PLACEHOLDER

## Record

[SR-PACKAGES](../../records/0107_packages.md) (the build legs); the crash's own record once the cause is known.
