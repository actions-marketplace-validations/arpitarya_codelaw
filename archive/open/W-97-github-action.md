---
type: Handoff
name: W-97
description: "Arpit, 2026-10-27 (W-36 gap G3): codelaw installed into other people's pipelines — the GitHub Action (listed as codelaw check) and the codelaw-pre-commit mirror repo (was W-64). Merged 2026-10-30."
item: W-97
filed: 2026-10-27
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5), W-97 closed"
ball: done
---

# W-97 — codelaw in other people's pipelines: the Action and pre-commit

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | `action.yml` (composite) at this repo's root: install codelaw at a pinned version (npm or PyPI), run `check --format github`, optionally `--format sarif` + upload to code scanning, with `--changed-since` on pull requests | `uses: arpitarya/codelaw@v0.x` in one line | ✅ 2026-10-27 |
| 2 | Only ratified actions inside (D-30); pinned by SHA (W-18) | L11 | ✅ 2026-10-27 — it uses **no** action; the SARIF upload is the REST API via `gh` (ruled 2026-10-27: keep `gh`) |
| 3 | `init --ci github` writes a workflow that uses it; docs in USAGE | adoption | ✅ 2026-10-27 |
| W-64 | `codelaw-pre-commit`: the pre-commit framework installs codelaw by version — merged in; its steps under [§ W-64](#merged-in-w-64) | one item, not two | ✅ 2026-11-05 — see § |
| 4 | **Arpit:** allow Claude Code to run `gh repo create --public` (or create the repos yourself and name them): `arpitarya/codelaw-action-test` for the Action's proof, `arpitarya/codelaw-pre-commit` for the mirror (was W-64) | both halves can move | ✅ 2026-11-05 |

**Repos created, 2026-11-05 (Cowork, built-in browser, signed in as `arpitarya`):** `arpitarya/codelaw-action-test` and `arpitarya/codelaw-pre-commit` — public, empty (no README, licence or description). **Arpit, 2026-11-05 (docket):** chose *"A · You create them now (Recommended)"*; the write-back *"Yes, write it (Recommended)"*, then *"file it."* **Next (agent):** the Action's proof in `codelaw-action-test` (Left, step 2), then the mirror's contents in `codelaw-pre-commit` (§ W-64).

**Merged 2026-10-30 (Arpit, text):** *"There are too many work items. See if we can review all the work items and merge them into few of them."* → *"go ahead, merge them."* Merged in: W-64.

**Arpit, 2026-10-27 (voice docket; written 2026-10-28):** *"Okay."* — to all three: (1) keep the SARIF upload on `gh`; (2) Claude runs the throwaway-repo proof after the next release tag; (3) publish to the GitHub Marketplace and keep a moving `v0` tag. Asked whether every CI has `gh` (GitHub-hosted runners do; self-hosted and slim images may not): *"Make sure it works with Bitbucket as well. I don't want any problems with it."* and *"File it as a work item and make it configurable. The serif part."* — filed as [W-109](W-109-bitbucket-pipelines.md).

**Arpit, 2026-10-27 (voice):** *"Yes to all four."* — on W-36's R1 (README fixes), R2 (file gaps G1–G8), R3 (reopen type-aware TS) and R4 (run the with/without-Fux experiment). [Proposal](../../work/proposals/linter-for-ai-positioning.md).

**Model:** Sonnet.

Why: Biome (`setup-biome`) and Ruff (`ruff-action`) ship one ([proposal](../../work/proposals/linter-for-ai-positioning.md) §3).

## Definition of done

- A workflow in a throwaway repo using the Action shows annotations and uploads SARIF; a version input pins the engine. ✅ 2026-11-05 — run 42002927893 (§ Closed). Was: waits on the decider: the released codelaw 0.2.0 has no `--format github`, so the proof needs the next release tag.

## Built (2026-10-27, Claude Code)

- **`action.yml`** — composite; inputs `version` (default written by `scripts/version.py`, so tag `v<x>` installs `<x>`), `registry` npm|pypi, `package`, `working-directory`, `args`, `changed-since`, `sarif`, `upload-sarif`, `fail-on-error`, `token`; outputs `exit-code`, `sarif-file`. Annotation and SARIF paths are rewritten from the project folder to the repo root. No `uses:` inside (L11; `crates/codelaw-cli/tests/ci_action.rs` holds it).
- **CI:** `ci.yml` job `action` runs `uses: ./` on `fixtures/github-action` (one local rule, one `debugger`), installed from the run's linux wheel; asserts exit `1` and one SARIF result at `fixtures/github-action/src/app.js`.
- **`init --ci github`** (no `--autofix`) writes `.github/workflows/codelaw.yml` from `templates/ci/github/codelaw.yml` (SR-CLI-MCP decision 12a); `--ci gitlab` alone is refused.
- Records: SR-PACKAGES decision 15, SR-CLI-MCP 12a; USAGE, RELEASING, CHANGELOG.

## Ruled 2026-10-30 — the listing's name and description

**Arpit, 2026-10-30 (voice), on the v0.4.0 edit page:** GitHub refused the listing — *"name codelaw must be unique, cannot match an existing action, user or organization name"* and *"description … must be less than 125 characters."* → *"Let's go with codelaw check."* · *"Whatever you are suggesting sounds good."*

- `action.yml`: `name: codelaw check`; the description shortened to 115 characters, keeping what the Action does (it checks and annotates; it does not fix). Written 2026-10-30 (Cowork). The repo path is unchanged, so `uses: arpitarya/codelaw@v0` still works.
- v0.4.0 carries the old `action.yml`, so the listing waits for **the next release**.

## Listed, 2026-10-30 (checked 10:15 IST, Cowork, in Chrome)

- **Live:** <https://github.com/marketplace/actions/codelaw-check> — **codelaw check**, by arpitarya, release **v0.4.0** (*Latest*), the 115-character description. Arpit published it on v0.4.0 (the Release was edited 2026-10-30 04:00 UTC), not on a later release as planned: the listing takes its name and description from `main`'s `action.yml` (`af9de8a5`), while `v0.4.0`'s own file still says `name: codelaw`. So step 1 below is done.
- **Found:** **no `v0` tag exists** (tags: `v0.2.0`, `v0.3.0`, `v0.4.0`) — `uses: arpitarya/codelaw@v0`, as USAGE and the ruling promise, fails today. `@v0.4.0` works (the listing's *Use latest version* button).
- **Found:** the listing shows the repo README, which still says *"0.3.0 is on npm and PyPI"* and *"The editor extension's Marketplace listing comes later"* — W-95's install-docs step.

## Left (ruled 2026-10-27; step 1 moved by the 2026-10-30 ruling)

1. ~~**Arpit:** accept the Marketplace Developer Agreement and tick *Publish this Action to the GitHub Marketplace*.~~ ✅ done on v0.4.0 (2026-10-30).
2. **Agent, now (0.4.0 has `--format github`):** ✅ 2026-11-05 (§ Closed). **2026-10-30: Claude Code's permission check refused `gh repo create --public`** (a new public surface), so the proof waits for Arpit to allow it or create the repo. Prepared: the fixture's `codelaw.toml`, `src/app.js` and `.codelaw/rules/fixture/no-debugger.md`, and the template's workflow with `@v0`, `sarif: true` and `security-events: write`. The DoD proof — `gh repo create arpitarya/codelaw-action-test --public --clone`, `.github/workflows/codelaw.yml` from `codelaw init --ci github` with `sarif: true` and `permissions: security-events: write`, one `debugger`, then the annotation and Security → Code scanning checked.
3. **Agent, each release:** `git tag -f v0 v<x> && git push -f origin v0` (RELEASING.md) — ✅ **`v0` created 2026-10-30 at `v0.4.0` (`d2a87c8`)**, pushed; a tag push triggers no workflow (`ci.yml` runs on branches, `publish.yml` on Releases).
4. **Offered, not ruled:** a clear error when `upload-sarif` is on and `gh` is absent (a self-hosted runner or slim image) — the plain check-and-annotate path needs no `gh`.

## Closed 2026-11-05 (Claude Code, Opus 5.5)

**The Action's proof** — [`arpitarya/codelaw-action-test`](https://github.com/arpitarya/codelaw-action-test), commit `7bc5975`: `.github/workflows/codelaw.yml` runs `uses: arpitarya/codelaw@v0` (v0 = v0.4.3) with `sarif: true` and `security-events: write` on this repo's `fixtures/github-action` (one `debugger`, local rule `fixture/no-debugger`). **Run [42002927893](https://github.com/arpitarya/codelaw-action-test/actions/runs/42002927893): success** — codelaw 0.4.3 installed from npm, the annotation *Remove the debugger statement [fix: auto]* at `src/app.js#2`, exit code 1 asserted, one SARIF result at `src/app.js`, uploaded; code scanning alert #1 `fixture/no-debugger` at `src/app.js:2`, open. The version input pins the engine (`version` defaults to the tag's). DoD met.

**The mirror** — [`arpitarya/codelaw-pre-commit`](https://github.com/arpitarya/codelaw-pre-commit), `main` = tag **`v0.4.3`** (`22bd59f`): `.pre-commit-hooks.yaml` with `codelaw-check` (`codelaw check --staged`), `codelaw-fix` (`codelaw fix --staged`), `codelaw-format` (`codelaw format`), each `language: python`, `additional_dependencies: ["codelaw==0.4.3"]`, `types_or` the nine L3 languages' file types, `require_serial`; a `pyproject.toml` with no code and no dependencies (pip needs a project to install a `language: python` hook repo — ruff-pre-commit's pattern); README; MIT `LICENSE`. **Verified** with `uvx pre-commit try-repo` in a fresh repo of the fixture: the environment installed `codelaw==0.4.3` from PyPI; `codelaw-check` failed on `src/app.js:2:3 fixture/no-debugger` (exit 1), `codelaw-format` reformatted a file, `codelaw-fix` applied the auto fix. DoD 1 met.

**Step 2, the bump** — the mirror's own `.github/workflows/bump.yml` (daily 03:17 UTC and `workflow_dispatch`; `actions/checkout` pinned by SHA, nothing else; `contents: write` on its own token): reads codelaw's latest version from PyPI, and when the pin differs rewrites it (hooks, README `rev`, pyproject), commits `codelaw <x>` and pushes tag `v<x>`; at an equal version it pushes the tag if missing. A scheduled job in the mirror rather than `publish.yml`: a cross-repo push from codelaw's workflows would need a token the decider creates. First run by hand, [42019788907](https://github.com/arpitarya/codelaw-pre-commit/actions/runs/42019788907): success, *PyPI: 0.4.3 · pinned: 0.4.3*, no bump. DoD 2 by mechanism; its first real bump comes with the next release (RELEASING.md says to check it).

**Step 3, the docs** — USAGE §*Without AI* (W-50's recipes): the mirror first, the main repo's `language: system` hooks for teams that already have codelaw; RELEASING.md: after each release move `v0`, the mirror follows by itself, re-run the proof; `.pre-commit-hooks.yaml`'s header points to the mirror; CHANGELOG *Unreleased*.

**Not ruled, carried:** Left step 4 — a clear error when `upload-sarif` is on and `gh` is absent — is B-073.

## Record affected

SR-PACKAGES (a new distribution surface) · SR-CLI-MCP (`init --ci`).

## Hazards

- An Action in the main repo is versioned by the repo's tags — the release tags must stay the Action's contract.

## Merged in: W-64

<a id="merged-in-w-64"></a>

**W-64 — `codelaw-pre-commit`: the pre-commit framework installs codelaw by version**, merged into W-97 on 2026-10-30; its file is archived at [`archive/open/W-64-codelaw-pre-commit-mirror.md`](W-64-codelaw-pre-commit-mirror.md). As it stood:

### At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Create the repo `codelaw-pre-commit` with a `.pre-commit-hooks.yaml`: hooks `codelaw-check`, `codelaw-fix`, `codelaw-format`, `language: python`, `additional_dependencies: ["codelaw==<version>"]` | `rev: v0.2.0` in a consumer's config installs the PyPI wheel — no Rust build, nothing on `PATH` | ✅ 2026-11-05 |
| 2 | Tag it per codelaw release; `publish.yml` (or a scheduled job) opens the bump | the mirror follows every release | ✅ 2026-11-05 |
| 3 | USAGE and W-50's recipes point to it; the `language: system` hooks stay for teams with codelaw already installed | both ways documented | ✅ 2026-11-05 |

**Arpit, 2026-10-26 (docket on W-50):** *"PATH now, mirror repo later"*.

**Model:** Sonnet — a small repo after Ruff's pattern, with a test install.

### Goal

A consumer's `.pre-commit-config.yaml`:
```yaml
- repo: https://github.com/arpitarya/codelaw-pre-commit
  rev: v0.2.0
  hooks: [{ id: codelaw-check }]
```

### Definition of done

| DoD | what | state |
|---|---|---|
| 1 · | `pre-commit run --all-files` in a fresh repo installs codelaw (0.4.0 today) from PyPI and runs it | ✅ 2026-11-05 (try-repo, 0.4.3) |
| 2 · | a codelaw release produces a matching mirror tag | ✅ 2026-11-05 (`bump.yml`; first real bump at the next release) |

### Blockers

W-38 is done: `codelaw` is on PyPI (checked 2026-10-29).

### Record affected

SR-CLI-MCP (`init --git-hooks` docs) · L6 is unaffected: the mirror publishes no package, it pins one.

### Hazards

- The mirror's version must equal codelaw's — a bump that lags gives users an old engine.

### Reference

[ruff-pre-commit](https://github.com/astral-sh/ruff-pre-commit) · [pre-commit: creating new hooks](https://pre-commit.com/#creating-new-hooks) · [W-50](W-50-git-hooks.md)
