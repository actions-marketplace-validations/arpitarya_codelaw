---
type: Handoff
name: W-184
description: "Arpit, 2026-11-09 (voice docket): add a file's mode bits to what a rule can read, and build Ruff EXE001 and EXE002 on it."
item: W-184
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-184 — Read a file's executable bit: EXE001 and EXE002

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Add the file's executable bit to what a rule can read | rules can check scripts | ✅ |
| 2 | Build EXE001 and EXE002 | a shebang without the bit, the bit without a shebang | ✅ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On the recommendation (build EXE001 and EXE002, backlog PLE2514): *"Perfect. Let's go with what you recommend."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): one field on the file context and two rules.

## Goal

`FileContext` carries the file's executable bit; `EXE001` (a shebang in a file that is not executable) and `EXE002` (an executable file with no shebang) read it.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · engine | the bit on `FileContext` (`is_executable`), read only when a rule asks, recorded as a cache question | ✅ |
| 2 · rules | EXE001, EXE002 with cases | ✅ |
| 3 · docs | what the rules do where there is no such bit | ✅ |

## Blockers

None.

## Record affected

SR-ARCHITECTURE decision 15b (the file context): a new bullet, *the file's executable bit is a question too* (2026-11-15).

## Hazards

- Windows has no executable bit: read git's recorded mode or report nothing, and say which (L5: the same answer everywhere). **Built: report nothing** off Unix, and git's mode is not consulted; stated in both rule files and in 15b.

## Built (2026-11-15)

`FileContext::is_executable` (`crates/codelaw-native/src/lib.rs`; the engine's answer in `crates/codelaw-engine/src/files.rs`, `Question::Executable`), `py/shebang-not-executable` (EXE001) and `py/shebang-missing-executable-file` (EXE002) in `crates/codelaw-native/src/py_exec.rs`. Rule-tester snippets give a file the bit with `+x` at the end of its header (`# run.py +x`); a snippet with no header has no bit to read.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Strand W-139-ruff, round 4*) · `work/proposals/rule-parity/ruff.csv`
