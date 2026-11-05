---
type: Standing Record
kind: component
name: SR-MULTI-FILE-FIXES
title: "SR-MULTI-FILE-FIXES (0119) — a fix may edit several files: five safeguards that are never configurable, and settings that are"
description: "When a fix may edit more than one file — a rename across a project, a dependency added to package.json — the five safeguards every such fix passes, and the project and per-rule settings that turn it off or keep it a suggestion."
status: accepted
date: 2026-11-05
feature: "multi-file fixes (W-162)"
owns: []
laws: [L4, L5]
timestamp: 2026-11-05T14:30:00Z
content_sha: ""
ratifies: W-162
---

# SR-MULTI-FILE-FIXES: a fix may edit several files

## §1 — For humans

Some findings can only be fixed in more than one place:

- A CSS class renamed to the team's convention must change in every page and
  script that uses it.
- An export renamed must change in every file that imports it.
- A package imported but not declared must be added to `package.json`.

Done by hand, these are the edits an agent most often gets wrong: it misses
one file.

**Ruled by Arpit, 2026-11-05:** allowed (*"Allow multi-file (Recommended)"*),
then, by voice, in its own record and configurable: *"Create an SR for
this."* · *"and make it configurable"* · *"it does and that's all"*.

The five safeguards are what keep a multi-file fix from breaking the build,
so they are never configurable. The settings decide only whether the fix is
offered, and whether it may be applied automatically.

```mermaid
flowchart LR
    F["a finding whose fix<br/>edits several files"] --> S{"the five safeguards"}
    S -->|"any fails"| N["nothing written;<br/>the finding drops a rung"]
    S -->|"all pass"| C{"every reference<br/>certain?"}
    C -->|yes| A["auto — applied by codelaw fix"]
    C -->|no| G["suggest — a diff to accept"]
```

<details>
<summary><b>ASCII twin</b></summary>

```text
  finding (fix edits N files) --> five safeguards --any fails--> nothing written, a rung lower
                                        | all pass
                                        v
                         every reference certain? --yes--> auto (codelaw fix only)
                                        | no
                                        v
                                     suggest (a diff to accept)
```

</details>

---

## §2 — For agents

### Context

A `TextEdit` carries no path, so every fix lived in its own file. 22 rules
could not climb the fix ladder without editing other files
([the design](../work/proposals/fix-capabilities.md) §2.2b):

- renames of CSS classes, ids, custom properties, keyframes, layers,
  containers and custom media;
- SCSS variables, mixins and placeholders, and CSS-module classes;
- Stencil tags, `js/new-cap`, the React hook-prefix rules,
  `jsx-pascal-case`, `js/no-default-export`, `spell/known-typo-key` and
  `html/valid-id`;
- and the two `node/no-extraneous-*` rules, which add a dependency to
  `package.json`.

### Decision

1. **A fix may edit several files.** A fix is a list of edit sets, each
   naming its file by its project-relative path, plus file-level operations
   (`strip-bom`). Every output carries the path:
   - JSON and MCP group `edits` by `path`;
   - SARIF uses `artifactChanges`;
   - LSP uses a `WorkspaceEdit`;
   - the text and agent lines say how many files the fix edits.
2. **The five safeguards.** Every multi-file fix passes all five, every
   time, and none is configurable:
   1. **Every file still parses.** If any edited file gains a syntax error,
      the whole fix is refused. [L4](0006_LAW-4-every-rule-has-a-fix.md)
      decisions 9 and 10 hold in every file: a second run changes nothing,
      the rule no longer fires, and bytes outside the edits are identical.
   2. **Nothing is left dangling.** The project graph
      ([SR-PROJECT-GRAPH](0103_project-graph.md)) is rebuilt over the edited
      files. If an import, class, id or other reference that resolved before
      now resolves to nothing, the fix is refused.
   3. **All or nothing.** Every file is written, or none is. Each goes to a
      temporary file first and is then renamed into place, and any failure
      restores every file already renamed.
   4. **Automatic only when it is certain.** The fix is `auto` only when
      every use it edits is resolved statically: SCSS members through
      `@use`, or an export whose importers are all static (no module marked
      `dynamic`, SR-PROJECT-GRAPH decision 4). One use the graph only
      guesses at, such as a class token inside a string, makes the whole fix
      `suggest`.
   5. **Never in the background.** A multi-file fix is never applied on save
      or by an agent hook (L4 decision 12). It is applied by `codelaw fix`
      (`fix --unsafe` for a `suggest` one), by a pre-commit run of it, by
      choosing it as a quick fix in the editor, or by an agent accepting it
      through MCP.

   A refused fix leaves every file untouched, and the finding drops a rung
   (L4 decision 2).
3. **The settings.** One key, `multi_file`, takes `"auto"`, `"suggest"` or
   `"off"` ([SR-CONFIG](0110_config.md) decision 11g):
   - **For the project:** `[fix] multi_file`. The default is `"auto"`.
   - **Per rule:** `multi_file` in the rule's entry, in `[rules]`, a
     `[scope.<name>]` or a `[profile.<name>.rules]`, as `fix` is (SR-CONFIG
     decision 11d). The rule entry wins over the project key.
   - **`"suggest"`** keeps every multi-file fix a diff to accept, even when
     safeguard 4 would allow `auto`.
   - **`"off"`** offers no multi-file fix. The finding is `manual` with
     exception `restructure`, and its steps are the edits the fix would have
     made.
   - **No setting raises a fix.** `"auto"` is a fix's best case, still bound
     by safeguard 4.
   - `codelaw config --print` and `rules --why` show the setting in force and
     the line that set it.
4. **The cache.** A finding with a multi-file fix is keyed on the content
   hashes of every file the fix reads or edits, as `scope: project` rules
   are ([SR-CACHE](0109_cache.md)).
5. **No network, no model** ([L5](0007_LAW-5-no-model-in-the-engine.md)).
   Every fact a multi-file fix uses comes from the project's own files,
   through the graph.

### Consequences

- **Built: no.** W-162 builds it as the cross-file write step of its order
  ([the design](../work/proposals/fix-capabilities.md) §4).
- L4 decisions 9, 10 and 12 point here; single-file fixes are unchanged.
- As this is built, the 22 rules move from
  `fix_pending: "…; needs cross-file"` to `suggest`, and some to `auto`. In a
  project with `multi_file = "off"`, they are `manual`.

### Alternatives considered

- **Single-file fixes only:** rejected 2026-11-05. The 22 rules would stay
  `manual` `restructure`, and every rename would be done by hand.
- **Configurable safeguards:** rejected. They are what make the fix safe to
  offer at all.
- **Multi-file fixes on save:** rejected. They rewrite files nobody has
  open.

### Reference (required)

- [`work/proposals/fix-capabilities.md`](../work/proposals/fix-capabilities.md)
  §2.2b and §5: the design, and the question put.
- `crates/codelaw-core/src/diagnostic.rs` (`TextEdit`): the edit that gains a
  path.

### Veto condition

**Reopen this decision if:** a multi-file fix writes some of its files and
not others, or leaves a reference unresolved that resolved before it.

**How to check it:** the multi-file fix tests W-162 adds under
`crates/codelaw-cli/tests/`. Until they exist, `codelaw check` after
`codelaw fix` reports no new `graph/*` finding.

---

## References

*Every source this record cites, gathered in one place. An archived document is never listed here.*

**Records** — [SR-LAW-4](0006_LAW-4-every-rule-has-a-fix.md) · [SR-LAW-5](0007_LAW-5-no-model-in-the-engine.md) · [SR-PROJECT-GRAPH](0103_project-graph.md) · [SR-CONFIG](0110_config.md) · [SR-CACHE](0109_cache.md)

**Code** — `crates/codelaw-core/src/diagnostic.rs`

**Project docs** — [`work/proposals/fix-capabilities.md`](../work/proposals/fix-capabilities.md) · [W-162](../work/open/W-162-fix-capabilities-build.md)
