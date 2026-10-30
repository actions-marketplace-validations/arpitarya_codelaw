---
type: Handoff
name: W-146
description: "Arpit, 2026-10-30 (voice): pull-request hygiene — commit messages (Conventional Commits by default, fully configurable), branch names, PR size and PR description — locally, in CI and for agents."
item: W-146
filed: 2026-10-30
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-10-31
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-146 — Pull-request hygiene: commits, branches, size, description

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Widen SR-ARCHITECTURE decision 14: git may also be read for commit messages in a range and the current branch name — read-only, offline | the data to check | ✅ SR-ARCHITECTURE 14 |
| 2 | `git/commit-message` — Conventional Commits by default; `[commits]` configurable | readable history | ✅ |
| 3 | `git/branch-name` — `[branches]` pattern, ticket, length | findable branches | ✅ |
| 4 | `git/pr-size` — warn above 400 changed lines, lockfiles and generated files excluded | reviewable PRs | ✅ |
| 5 | `git/pr-description` — title, ticket link, template sections, from the CI's own event data | complete PRs | ✅ |
| 6 | A `commit-msg` hook (`init --git-hooks commit-msg`), CI over the PR's commits, the agent hook | checked before, during and after | ✅ (agent hook: every agent's shell tool, 10-31) |
| 7 | Share `[commits]`/`[branches]`/`[pr]` through a preset | one convention for many repos | ✅ 10-31 for the presets `extends` applies; team-pack presets → B-069 (the decider's) |

**Arpit, 2026-10-30 (voice):** *"Pull request hygiene is something I would go for."* · *"want the commit convention to be configurable."* · on the example `[commits]` and `[branches]` keys: *"All this looks good."* · on widening the git rule and the defaults (Conventional Commits, a 400-line warning): *"Yes."* → *"file it."*

**Model:** Opus (Claude Code).

## Goal — the configuration (the keys Arpit approved, final names settled in the record)

```toml
[commits]
preset = "conventional"          # conventional | angular | gitmoji | custom | off
types = ["feat", "fix", "docs", "refactor", "test", "chore", "perf", "ci", "build"]
scopes = []                      # empty = any scope
require_scope = false
subject_max = 72
subject_case = "lower"           # lower | sentence | any
require_body = false
breaking = ["!", "BREAKING CHANGE:"]
pattern = ""                     # a regex, only with preset = "custom"

[commits.ticket]
required = false
pattern = "[A-Z]+-[0-9]+"
place = "footer"                 # subject | footer | either

[commits.skip]
merges = true
reverts = true
fixups = true
authors = ["dependabot[bot]", "renovate[bot]"]

[branches]
pattern = ""                     # empty = branch names not checked
max_length = 60
protected = ["main", "develop", "release/*"]
skip_authors = ["dependabot[bot]", "renovate[bot]"]

[branches.ticket]
required = false
pattern = "[A-Z]+-[0-9]+"
match_commits = false

[pr]
max_lines = 400                  # warn above; 0 = off
exclude = ["**/*.lock", "**/package-lock.json", "**/pnpm-lock.yaml", "**/generated/**"]
title_pattern = ""
require_ticket = false
template_sections = []           # headings the description must fill
```

Defaults ruled: Conventional Commits on, a 400-line PR-size warning. Defaults chosen by the agent (say so in the record): ticket checks off, branch names unchecked until a pattern is set, PR description unchecked until a section or pattern is set.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · git | SR-ARCHITECTURE decision 14: from six uses to eight — `git log --format` over the `--changed-since` range, and the current branch (or the CI's branch variable) | ✅ |
| 2 · rules | the four rules, each `manual` with steps (a message is never rewritten silently); the error shows the message, what is wrong and a corrected example | ✅ |
| 3 · where | `init --git-hooks commit-msg` (template in `templates/git-hooks/`, L10); `codelaw check --changed-since` checks the PR's commits; the agent hook checks a commit an agent makes | ✅ `check --commit-msg`; the GitHub Action already runs `--changed-since` on a PR; the Gate on Claude Code `Bash` |
| 4 · PR data | GitHub (`GITHUB_EVENT_PATH`), GitLab (`CI_MERGE_REQUEST_TITLE`, `CI_MERGE_REQUEST_DESCRIPTION`); Bitbucket and Azure DevOps expose no description without an API call — title or branch only there, the rest reported as "not checked here", never fetched (L5) | ✅ |
| 5 · config | SR-CONFIG `[commits]`, `[branches]`, `[pr]`; shareable through a pack | ✅ SR-CONFIG 11b; a preset's `[rules]` sets the `git/*` rules (10-31). How a *team* pack names its preset in `extends` is unruled: B-069 |
| 6 · tests | each preset; skips; the hook; each CI's data, from fixtures | ✅ `tests/pr_hygiene.rs`, `hygiene` unit tests |

## Blockers

None. Closed 2026-10-31; the one fork left (how a team pack's preset is named in `extends`) is B-069 in the backlog, the decider's.

## Record affected

[SR-ARCHITECTURE](../../records/0108_architecture.md) decision 14 · SR-CONFIG · SR-CLI-MCP decision 11 (git hooks) · SR-RULE-PACKS · SR-AGENT-SURFACES (the hook).

## Hazards

- **A commit is not a file:** findings need a location — a logical one (`commit:<sha>`, SARIF `logicalLocations`; the hook's message file in hook mode). Decide in SR-ARCHITECTURE; no L3 change.
- Never network (L5): the PR description is read only from what the CI already put on disk or in the environment.
- Corrected at voice: the architecture record allows git for **six** things today, not three.

## Reference

W-50 (git hooks) · W-131 (ticket patterns, the same `[A-Z]+-\d+` shape)

## Progress — 2026-10-30 (Claude Code)

Landed: `crates/codelaw-engine/src/hygiene.rs` (the four rules), `git.rs` (commits, branch, numstat), `[commits]`/`[branches]`/`[pr]` in `codelaw-config` and `codelaw.schema.json`, `check --commit-msg`, `init --git-hooks commit-msg` (installed only when named; a bare `--git-hooks` stays pre-commit + post-commit), the Gate on `git commit -m` (Claude Code `Bash` matcher), logical locations in SARIF/GitHub output. Records: SR-ARCHITECTURE 14, SR-CONFIG 11b, SR-CLI-MCP 11, SR-AGENT-SURFACES 9, SR-RULE-PACKS 1, SR-RULE-FORMAT 1, SR-LAW-10 3.

**Left (agent):**
1. Pack sharing: let a preset's `[rules]` settings (severity, options) reach engine rules, so a team pack can carry `git/commit-message` options; today the tables are root-config only.
2. The Gate for Codex's shell tool and the other agents' (their payload shapes for shell commands are unconfirmed); the `commit-msg` git hook covers them meanwhile.
3. `init --help`'s `--git-hooks` line still names two hooks (`init.rs` is also described by SR-SPELL; left for a change that touches both).

Behaviour change to note in the changelog: with `--changed-since`, Conventional Commits is now enforced by default (`git/commit-message`, `error`) — a repository with free-form messages sets `[commits] preset = "off"` or turns the rule off.

## Closed — 2026-10-31 (Claude Code)

1. **Pack sharing.** A preset's `[rules]` may set the four `git/*` engine rules — severity, and options holding the table's keys — though no pack holds them (`pack.rs` allows `git/*` names in a preset; `RuleSet::engine_settings` carries them). Lowest first: the presets' in `extends` order, then the root's `[commits]`/`[branches]`/`[pr]`, then the project's `[rules]`, key by key (`hygiene.rs::git_options`). `git/pr-size` and `git/pr-description` now read their own options over `[pr]` too. Only presets `extends` applies count — the `codelaw:` ones; an `npm:`/`pypi:`/`path:` entry applies none of its pack's presets (SR-RULE-PACKS d11), so a team pack cannot carry them until the decider rules B-069. Test: `pr_hygiene.rs::a_preset_shares_the_git_rules_settings_and_the_project_table_wins`.
2. **Other agents' shell tools.** `hook --pre` reads a shell command from Codex `Bash` (and `exec_command`, which matches as `Bash`), Gemini `run_shell_command`, Copilot/VS Code `toolArgs.command`, Windsurf `pre_run_command` `tool_info.command_line` and Cursor `beforeShellExecution` `command`; a `["bash", "-lc", …]` list too; a payload that names a file or holds a patch stays a write. The Codex template's `PreToolUse` matcher gains `Bash` (from Codex's hooks docs, 2026-10-31). Test: `pre.rs::every_agents_shell_command_is_read`.
3. **`init --help`** names `commit-msg` (installed only when named) — e6a6be83.
4. **W-141's CHANGELOG line** corrected: the comment and the next selector are no longer joined on one line — e6a6be83.

Records: SR-CONFIG 11b, SR-RULE-PACKS 1, SR-RULE-FORMAT 1, SR-ARCHITECTURE 14, SR-AGENT-SURFACES 9, SR-CLI-MCP 11, SR-RULE-AUTHORING 4, SR-PACK-LOOKUP (Consequences).
