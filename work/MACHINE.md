---
type: Reference
description: "Environment and tooling quirks of wherever this runs — why a command fails on THIS surface."
---

# MACHINE — the surfaces this runs on

One section per surface (a laptop, a CI runner, a Cowork bridge shell). A row
is added when something fails for a reason the code did not cause.

| surface | symptom | cause | what to type instead |
|---|---|---|---|
| macOS laptop (zsh) | `timeout: command not found` | GNU coreutils is not installed | leave `timeout` out; use the tool's own timeout |
| macOS laptop (zsh) | a script using 3.10+ syntax fails | `python3` is 3.9.6 here (the L6 floor of 3.11 is for consumers) | keep repo scripts to Python 3.9 syntax |
| GitHub Windows runner | a repo script printing `✔` dies with `UnicodeEncodeError` | the console is cp1252 | `PYTHONUTF8=1` (set in `ci.yml`); the scripts also reconfigure stdout to UTF-8 |
| Claude Code cloud session | `gh api …/jobs/<id>/logs` and `…/actions/caches` refused | the session's proxy blocks Actions log blobs and the cache API | read a failure from the job's check-run annotations (`gh api repos/<r>/check-runs/<id>/annotations`); steps print their own `::error::` |
| macOS laptop (zsh) | disk 99 % full (2.8 GB free) while `disk-budget.py` reported ~24 GB | parallel agents: the shared `target/` held one build per worktree (23.7 GB) and agents' scratch cargo targets under `/private/tmp/claude-501/…/scratchpad/` (17 GB) — outside the repo, so the hook does not count them | stop the builds, `cargo clean`, `cargo clean --target-dir` each scratch target — § Where the disk goes |
| Claude Code cloud session | `No space left on device` with several agents building at once | each cargo target dir is ~3 GB with debug info; the session's disk allowance is ~40 GB | one target dir per lane with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0`; delete a lane's target dir when it finishes |

## Disk budget — law L9: this section is always followed

**This section binds** ([SR-LAW-9](../records/0011_LAW-9-machine-budget.md)): every session and every
agent follows it — Claude Code or Cowork, main agent or sub-agent. A number here
changes only by the decider's ruling. `.claude/hooks/disk-budget.py` reads the
`budget` row below; keep its `<n> GB` spelling.

| what | value |
|---|---|
| budget | 5 GB for the whole project — main session + every sub-agent + every worktree + build output, combined |
| past the budget | every further GB needs the decider's approval, asked before it is used: Claude Code raises a permission prompt; a Cowork session asks in chat. One yes approves one GB |
| new sub-agent or worktree | asks from 90 % of the approved ceiling |
| approvals reset | when the project is back under the budget |
| not counted | shared caches outside the repo: `~/.cargo`, the pnpm store, pip's cache |
| see it | `python3 .claude/hooks/disk-budget.py fresh` |
| codelaw's cargo | one shared `CARGO_TARGET_DIR` (`.claude/settings.local.json`) and no debug info for dev and test |
| at filing (2026-10-24) | codelaw is 15 GB — past the budget; `target/` alone is 15 GB, rebuildable with `cargo clean` (the installer offers it) |

### Where the disk goes — measured 2026-11-08, when the laptop ran out

The laptop's disk is **228 GB**; on 2026-11-08 it was at **99 % — 2.8 GB free** — and Arpit could not work. Facts, not rules (the budget above is the rule):

| what | size at the time | where | counted by `disk-budget.py`? | rebuildable? |
|---|---:|---|---|---|
| codelaw's shared cargo target | **23.7 GB** | `target/` (the `CARGO_TARGET_DIR` in `.claude/settings.local.json`) | yes | yes — `cargo clean` |
| agents' scratch cargo targets (sanitizer, bisect, perf builds) | **17.2 GB** (5.6 + 2.3 + 9.3) | the session scratchpads under `/private/tmp/claude-501/-Users-arpitarya-my-programs-codelaw/<session>/scratchpad/` | **no** — outside the repo | yes — `cargo clean --target-dir <dir>` |
| copied `codelaw` binaries, generated bench workloads | ~1.5 GB | the same scratchpads (`bin/`, `cl*`, `wl*`) | no | yes |
| each agent worktree | ~100–110 MB | `.claude/worktrees/W-nn` | yes | only once landed |
| `~/.cargo/registry`, pnpm store | 0.6 GB, 0.2 GB | home | no (shared caches) | yes |

- **What fills it:** several agents building at once. Each worktree's paths hash differently, so the one shared `target/` holds a full build **per worktree** (5 agents ≈ 20 GB+), and every agent that bisects or runs a sanitizer adds its own scratch target dir on top — which the budget hook does not see.
- **What freed it (2026-11-08):** all agents stopped; `cargo clean` (23.7 GB); `cargo clean --target-dir` on three scratch targets (17.2 GB); scratch binaries and workloads deleted — **2.8 GB → 43 GB free**. Nothing that was not build output was deleted; unlanded work stayed in its worktree.
- **To check it:** `df -h ~` (the real free space) and `python3 .claude/hooks/disk-budget.py fresh` (the project) and `du -sh /private/tmp/claude-501/-Users-arpitarya-my-programs-codelaw/*/scratchpad/* | sort -h | tail` (what the hook misses).
- **To free it fast, safely:** stop the builds first; then `cargo clean`; then, for every scratchpad directory that holds a `CACHEDIR.TAG`, `cargo clean --target-dir <dir>`. Never delete a worktree or a scratchpad spec folder without checking it is landed.
