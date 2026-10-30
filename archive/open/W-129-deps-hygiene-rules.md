---
type: Handoff
name: W-129
description: "Arpit picked 2026-10-29 (Cowork, voice) four more dependency rules for the deps pack: licence, lockfile in sync, monorepo version drift, runtime version agreement."
item: W-129
filed: 2026-10-30
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-10-30
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-129 — Licence, lockfile sync, version drift and runtime agreement

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build `deps/licence` from the snapshot's licence metadata | copyleft or unknown licences caught | ✅ 2026-10-30 |
| 2 | Build `deps/lockfile-sync` | a manifest edited without its lockfile caught | ✅ 2026-10-30 |
| 3 | Build `deps/version-drift` | one version of a package across a monorepo | ✅ 2026-10-30 |
| 4 | Build `deps/runtime-agreement` | Node and Python versions agree everywhere they are named | ✅ 2026-10-30 |
| 5 | Report each text-file finding in both places, once W-136 amends L3 | the `.nvmrc` line and `package.json` both show it | ✅ 2026-10-30 — runtime pins and yarn classic licences |

**Arpit, 2026-10-29:** *"The license check, I like it. The lock file sync, I like it. Version drift in monorepo, I like that one as well. The runtime version agreement, I like that as well."*

**Model:** Sonnet (Claude Code): four contained rules on parsers that exist; SPDX expression parsing is the one new piece.

## Goal

[The proposal](../../work/proposals/deps-flags-markers-reviews.md) §2.5, in the `deps` pack W-128 creates.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · licence | `[deps.licence] allow / deny / unknown`, defaults as the proposal; SPDX `OR`/`AND`/`WITH`; direct and transitive | ✅ |
| 2 · lockfile-sync | package-lock, pnpm, yarn, uv, poetry (`content-hash` recomputed); manual fix names the lock command | ✅ |
| 3 · version-drift | npm/pnpm/yarn workspaces, uv members; `policy = "highest"\|"root"`; suggest fix | ✅ |
| 4 · runtime-agreement | every source in the proposal; `[deps.runtime] source`; suggest on L3 files | ✅ |
| 5 · records, tests, docs | SR-RULE-PACKS, SR-CONFIG; fixtures per lockfile kind; corpus; L8; rule docs | ✅ |

## Closed 2026-10-30 — what was decided where the proposal was silent

- **Licence lists:** the configured `deny`, then `allow`, are checked before the defaults, so `allow = ["LGPL-*"]` lifts a default denial. `[deps.licence] ignore` takes `[deps] ignore`'s shapes; `[deps] ignore`'s whole-package entries hold too. A closed alias table maps exact texts (PyPI classifiers such as `MIT License`, `Apache 2.0`) to one id; every other free text is `unknown`.
- **Transitive licences are one finding per lockfile** (up to ten named, the count, the worst grade): the engine keeps one finding per rule and place (`codelaw-engine` dedups by rule and span), so per-package findings on the first line collapsed to one. **The same collapse hits `deps/vulnerable`'s transitive findings today** — only the first survives; not changed here (W-128's rule).
- **Lockfile sync:** spec equality for package-lock and pnpm (what `npm ci` checks); yarn only "has an entry"; uv by clause set; Poetry's `content-hash` as 1.x, 2.x and 2.x-with-`[dependency-groups]` would write it, any match is in sync (verified against Python's `json.dumps` + SHA-256 on two pyprojects; not against a real Poetry install — none on this machine).
- **Drift:** `peerDependencies` not compared; a protocol range or a name declared twice in one manifest is skipped; ties go to the most common range, then the root's.
- **Runtime:** `[deps.runtime] source` is a table `{ node, python }`; only root files are read; the reference pin is the truth when a pin, else the first pin the range allows; pins agree on their common prefix. A text-file pin's finding sits on the root manifest's range (else line 1) with the `elsewhere` copy.
- **`codelaw-test`** accepts `uv.lock` / `poetry.lock` as snippet files so the TOML example can carry one.
- **L8:** debug build, 3,000-package lockfile, 0.64–0.68 s for the whole `deps/recommended` (no release build: L9 disk). **Corpus:** 556 files, 0 findings.

## Blockers

None. Closed 2026-10-30.

## Record affected

SR-RULE-PACKS decision 15 (as W-128 leaves it) · SR-CONFIG §`[deps]`.

## Hazards

- `.nvmrc`, `.python-version`, `.tool-versions`, Dockerfile, yarn classic `yarn.lock`: Q-A ruled B (2026-10-30, *"in both the places"*) — the finding goes on the L3 file **and** the text file's line, through W-136.
- A licence field is free text on old packages — `unknown`, never a guess.
- No new dependency (L11): write the SPDX parser.

## Reference

[Proposal](../../work/proposals/deps-flags-markers-reviews.md) §2.5, §8 · [W-128](W-128-deps-pack-and-vulnerabilities.md)
