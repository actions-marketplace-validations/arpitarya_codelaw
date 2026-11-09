---
type: Handoff
name: W-169
description: "`codelaw rules lint packs/<pack>/` segfaults in CI now and then, with nothing printed first — on linux-arm64-musl (js) and x64 (react); the same commit passes on another run."
item: W-169
filed: 2026-11-05
filed_by: "Claude Code (session 0a172e1f)"
updated: 2026-11-08
updated_by: "Claude Code (CI-green agent, session d73e9989)"
ball: agent
---

# W-169 — `rules lint` segfaults in CI, intermittently

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Capture it: run the build legs' `rules lint` under `ulimit -c unlimited` and upload the core and a backtrace (or `RUST_BACKTRACE=full` plus a SIGSEGV handler) as an artifact on failure | the crashing frame, not a guess | ✅ live since the push of 11-07 (`7cdb480b`); caught the crash on its third run (42348986474) |
| 2 | Fix the cause the backtrace shows; a regression test where it can be reproduced | release builds that do not crash | 🔨 fixed 11-07: name lookups read a NUL-terminated copy; regression tests in `codelaw-lang` |
| 3 | Five green runs of every build leg in a row | proof it is gone, not hidden | 🔄 **1 of 5** (2026-11-08): `ci` 42454141410 (push of `b20e1a1f`, which holds the fix) — all eight build legs green; the run itself is red for other jobs (§Green runs) |

## Green runs of every build leg, counted on the fixed code (2026-11-08)

The fix (`c466c98a`) is in `b20e1a1f`, the commit the decider pushed on 11-08. Every `ci` run on GitHub since the crash capture went live, newest first (`gh run list --workflow ci`; legs read with `gh run view <id> --json jobs`):

| run | sha | event | the eight build legs | on the fix? |
|---|---|---|---|---|
| 42454141410 | `b20e1a1f` | push | **all eight green** (linux-arm64-musl 258 s) | yes — **green run 1** |
| 42427010052 | `7cdb480b` | schedule | `linux-arm64-musl` **failed** (310 s) in "the binary runs, says its version, and passes every pack's tests"; `crash-linux-arm64-musl` was uploaded | no (before `c466c98a`) — the crash recurred, as the diagnosis predicts (its core was not read) |
| 42348986474 | `7cdb480b` | dispatch | `linux-arm64-musl` **failed** (311 s): the core that named the cause | no |
| 42348089566, 42347941992 | `7cdb480b` | dispatch, push | all eight green | no — green by luck of the allocator, not evidence |

So **one consecutive green run on the fixed code; four more are needed** (they come from scheduled runs, pushes and dispatches; an agent cannot trigger one). The item stays open until five exist. Run 42454141410 is red as a whole for reasons that are not this one (the SR guard replay, three `corpus` legs, the L8 perf gate); W-169's definition counts the build legs only.

## What is known

- Run 42053908300 (main `3b53d1a8`, `workflow_dispatch`): `build linux-arm64-musl`, `codelaw rules lint packs/js/` → `Segmentation fault (core dumped)`, exit 139, right after js's `codelaw test` passed (job 127337006685).
- The same commit **passed** that leg in push run 42053750244; an x64 leg crashed on `react` once, again with no output first.
- Ruled out by reading (the main-green fix, 2026-11-05): the new regex parser (`regex_ast.rs`) is not on `rules lint`'s path; `check`'s workers already run on 64 MB stacks; the work needs under 1 MB of stack. So a stack overflow is unlikely; a data race or an FFI fault (tree-sitter grammars, the allocator) is the next suspect.
- **Found (2026-11-07): an over-reading `strncmp` in the musl binaries, reached through tree-sitter's name lookup — §The cause.**
- **Not reproduced locally (2026-11-07, macOS arm64; not CI's Linux, which this machine cannot run — no container runtime):** no crash in 19,000+ runs across release, ASan and TSan builds — §Investigation.
- Effect: a red build leg skips `perf` (until W-126's `aed60b27` is on GitHub) and the corpus jobs, and would fail a release.

## The cause (2026-11-07, from the core)

CI run 42348986474 (`workflow_dispatch` on `7cdb480b`), `build linux-arm64-musl`: `codelaw test packs/js/` → SIGSEGV; the artifact `crash-linux-arm64-musl` held the core. Runs 42347941992 (push) and 42348089566 (dispatch) had all eight build legs green.

- **The faulting read:** `ldrb w12, [x9, x11]` at `0x2ccba6c`, address `0xff3d223c0000` — the first byte of an unmapped page. `x1` (the second string) is `0xff3d223bfff0` and `x2` (the length) is `0x10`: a 16-byte string ending exactly at the page boundary, read at index 16.
- **The function:** a `strncmp(l, r, n)` (zig's `c.string.strncmp`, `string.zig:106`, per the fixed twin) whose loop loads `l[i+1]`, `r[i+1]` while `i < n`, so when the first `n` bytes agree it reads index `n` before returning 0 — one byte past what `strncmp` may read. musl's own `strncmp` does not; this is the copy in the static musl binary (maturin links it with zig).
- **Its caller (`0x2caa918`):** tree-sitter's `ts_language_symbol_for_name` — `strncmp(name, "ERROR", …)` when named, then the loop over the symbols with `strncmp(symbol_names[i], string, length)` and `!symbol_name[length]`, returning `public_symbol_map[i]`. So: a node-kind lookup (`Language::id_for_node_kind`) of a 16-byte kind name, called from ast-grep's rule compiling (the recursive frames are the rule tree).
- **Why only sometimes, and only on musl:** the read faults only when the name's heap allocation ends at the end of a mapped page with nothing mapped after it. musl's allocator (mallocng) maps small-allocation groups on their own; glibc's heap is contiguous, and macOS's `strncmp` stays within `n` — which is why the local stress and the sanitizers (§Investigation) never saw it. The x64 crash on `react` was most likely the same lookup on `linux-x64-musl` (its leg was not recorded).
- **The twin came out stripped:** `backtrace.txt` said its `.text` was byte-identical — the whole file was. The frames above were named by disassembling the shipped binary (lldb). **Fixed 2026-11-07:** `strip = false` left Cargo's release default `-C strip=debuginfo` on the final link, and zig, the linker of the `--zig` legs, strips everything for either. The twin now builds with `--strip false` and `-C strip=none` on the final crate only. A local `aarch64-unknown-linux-musl --zig` build gave a twin with 120,192 symbol-table entries (`ts_language_symbol_for_name at language.c:283`, and zig's `c.string.strncmp at string.zig:106` — the over-reading `strncmp` named), relinked only `codelaw-cli`, and had a `.text` byte-identical to the shipped build's. A profile override (`profile.release.strip = "none"`) gave symbols too, but it rebuilt all 353 crates and moved the code, so it would have been useless for the core. `crash-capture.sh collect` ran locally end to end (twin built, `twin: 40795 symbols` in `backtrace.txt`).

**The fix:** `codelaw-lang`'s `Grammar::kind_to_id` and `field_to_id` — every kind and field lookup, ast-grep's and the rule compiler's — hand tree-sitter a copy of the name followed by a NUL (on the stack under 128 bytes), so a `strncmp` that reads `name[len]` reads that NUL. Recorded in [SR-LINTER](../../records/0106_linter.md) decision 1.

**Tests:** `grammar::tests` (the byte after every copy is a NUL, for 0-, 1-, 127-, 128- and 300-byte names; lookups still find kinds and fields) and `tests/name_lookup_guard_page.rs` (names written flush against a `PROT_NONE` page, looked up in six grammars). The guard-page test faults without the fix only where `strncmp` over-reads — the musl binaries; CI's `test` job runs on glibc, where it passes either way, so the unit test is the one that holds the line everywhere.

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

What is left: a fault that needs Linux — the musl leg (mallocng, musl's `exit`, a static binary) or the runner (memory pressure, the kernel); the x64 crash says it is not musl alone. Step 1's backtrace was the way on (§The cause). The sanitizer builds lived in the session scratchpad and were deleted.

## Record

[SR-PACKAGES](../../records/0107_packages.md) (the build legs); the crash's own record once the cause is known.
