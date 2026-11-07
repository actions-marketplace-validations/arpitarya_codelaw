---
type: Handoff
name: W-169
description: "`codelaw rules lint packs/<pack>/` segfaults in CI now and then, with nothing printed first — on linux-arm64-musl (js) and x64 (react); the same commit passes on another run."
item: W-169
filed: 2026-11-05
filed_by: "Claude Code (session 0a172e1f)"
updated: 2026-11-07
updated_by: "Claude Code (session 0a172e1f)"
ball: arpit
---

# W-169 — `rules lint` segfaults in CI, intermittently

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Capture it: run the build legs' `rules lint` under `ulimit -c unlimited` and upload the core and a backtrace (or `RUST_BACKTRACE=full` plus a SIGSEGV handler) as an artifact on failure | the crashing frame, not a guess | 🔨 landed 11-07 (`8ad936ec`), unproven: `main` is not pushed, so no CI run has used it |
| 2 | Fix the cause the backtrace shows; a regression test where it can be reproduced | release builds that do not crash | ⏳ local stress and sanitizers found nothing (11-07); waits on step 1's backtrace |
| 3 | Five green runs of every build leg in a row | proof it is gone, not hidden | ⏳ |

## What is known

- Run 42053908300 (main `3b53d1a8`, `workflow_dispatch`): `build linux-arm64-musl`, `codelaw rules lint packs/js/` → `Segmentation fault (core dumped)`, exit 139, right after js's `codelaw test` passed (job 127337006685).
- The same commit **passed** that leg in push run 42053750244; an x64 leg crashed on `react` once, again with no output first.
- Ruled out by reading (the main-green fix, 2026-11-05): the new regex parser (`regex_ast.rs`) is not on `rules lint`'s path; `check`'s workers already run on 64 MB stacks; the work needs under 1 MB of stack. So a stack overflow is unlikely; a data race or an FFI fault (tree-sitter grammars, the allocator) is the next suspect.
- **Not reproduced locally (2026-11-07, macOS arm64; not CI's Linux, which this machine cannot run — no container runtime):** no crash in 19,000+ runs across release, ASan and TSan builds — §Investigation.
- Effect: a red build leg skips `perf` (until W-126's `aed60b27` is on GitHub) and the corpus jobs, and would fail a release.

## Capture (step 1, built 2026-11-05)

`scripts/crash-capture.sh`, wired into `ci.yml`'s `build <key>`:

- **`arm`**, a step before the packs loop: every core goes to `$RUNNER_TEMP/crash` (`kernel.core_pattern` on Linux, `kern.corefile` on macOS, WER LocalDumps for `codelaw.exe` on Windows). The loop's step says `ulimit -c unlimited`.
- **`collect`**, `if: failure()`: copies the core and the shipped binary, builds an **unstripped twin** — the same `maturin build --release --locked --target … <flags>` with `strip = false`, never shipped — and writes `backtrace.txt`: `$_siginfo` (the fault address), registers, the faulting instructions and `thread apply all bt` from gdb (installed by apt if missing; a static musl core reads the same as a glibc one, so the musl legs need nothing else), or lldb plus macOS's `DiagnosticReports` crash report. On Linux it says whether the twin's `.text` is byte-identical to the shipped binary's — if it is, the names are the crash's. Windows keeps the minidump only.
- The artifact is **`crash-<key>`** (14 days); `backtrace.txt`'s first 60 KB also go to the job summary. The job's timeout went from 20 to 35 minutes for the twin's rebuild.
- **Unproven:** `main` is not on GitHub yet (the decider's push), so no run has armed it or caught a crash. The first red build leg after the push is the proof; until then the `arm` step's `sudo sysctl` on each runner image is untested.

## Investigation (step 2, 2026-11-07)

Every run is `codelaw <cmd> packs/<pack>/` over all 27 packs, four at a time, on this Mac. Packs frozen at the binaries' commit (`86c102e6`, by `git archive`): a first ASan `test` pass ran in the worktree while auto-land merged newer `packs/` under it (W-162's `edit-text`), and its 51 exit-1/2 failures were that skew — each printed its load error; none was a signal.

| binary | `rules lint` | `test` | result |
|---|---|---|---|
| release (`--release`, thin LTO) | 200 passes, 5,400 runs | 20 passes, 540 runs | 0 failures |
| ASan, Rust code (`-Zsanitizer=address`, nightly 2026-11-04) | 5,400 | 540 | 0 failures, no report |
| ASan, Rust **and** C (`CFLAGS=-fsanitize=address`: tree-sitter's runtime and every grammar's parser and scanner) | 5,400 | 540 | 0 failures, no report |
| TSan (`-Zsanitizer=thread -Zbuild-std`) | 50 passes, 1,350 runs | 5 passes, 135 runs | 0 failures, no race report |

What was read, and why each suspect is unlikely:

- **Parallel rule loading — there is none on this path.** `rules lint` → `rules_at` → `load_pack` → `compile_check`: the rule set is built in a plain loop, `compile_all` is a `for` loop, and `Project::from_parts` asks for `threads: Some(1)`. The only other thread is the one rayon worker `pool()` makes (64 MB stack) and drops unused. `providers.rs`'s two threads run only for a Tailwind provider.
- **`unsafe` in codelaw:** `libc::kill` (`codelaw-core/src/process.rs`), the watch's signal handler (`watch_cmd.rs`), `wait4` in `bench`, two `LanguageFn::from_raw` in the in-tree grammars, one `set_var` in a test. None is on `rules lint`'s path.
- **Tree-sitter FFI:** compiling parses ast-grep patterns and looks up node kinds through tree-sitter's C runtime and the grammars' external scanners — the one place C runs. The C-instrumented ASan build covers exactly that.
- **The allocator:** the system one (no `#[global_allocator]`; no mimalloc or jemalloc in `Cargo.lock`); on musl that is mallocng.

What is left: a fault that needs Linux — the musl leg (mallocng, musl's `exit`, a static binary) or the runner (memory pressure, the kernel); the x64 crash says it is not musl alone. Step 1's backtrace is the way on. No fix without it, so no regression test yet. The sanitizer builds lived in the session scratchpad and were deleted.

## Record

[SR-PACKAGES](../../records/0107_packages.md) (the build legs); the crash's own record once the cause is known.
