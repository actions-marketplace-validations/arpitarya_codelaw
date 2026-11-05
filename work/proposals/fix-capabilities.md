---
type: Proposal
description: "W-161's research (2026-11-05): the six fix capabilities the engine lacks, one design each, the build order for W-162, and the 37 waiting rules no capability will unblock."
item: W-161
date: 2026-11-05
author: "Cowork (Opus 5.5)"
---

# The fixes the engine can't make yet — six designs for W-162

**The takeaway.** 151 rules carry `fix_pending: "…; needs <capability>"` on
`main` (`d7ec95ae`), not the 119 the 2026-10-30 review counted. **112** of them
move once W-162 builds six things, **37** are waiting on an author's decision
that no capability can make, and should be re-judged `manual` with their
exception now (§3). **2** belong to other items. Build order (§4): computed
edits first (56 rules, no new dependency), cross-file edits last (33 rules, the
only part that changes L4 itself, and so the only part that needs a ruling,
§5).

## 1 · What is waiting, counted on `main`

Counted from every rule file's `fix_pending:` under `packs/`, 2026-11-05.

| capability (`needs …`) | waiting | unblocked by it | re-judge (§3) | elsewhere |
|---|---|---|---|---|
| `computed-edit` (+ `comment-preserving block dedent`) | 62 + 1 | 57 | 6 | — |
| `cross-file` (+ `byte-level-edit`, `type-info`'s `triple-slash-reference`) | 43 + 1 + 1 | 34 | 11 | — |
| `add-import` | 19 | 7 | 12 | — |
| `scope-rename` | 9 | 7 | 2 | — |
| `type-info` | 8 | 4 | 3 | 1 → cross-file |
| `purity` | 5 | 2 | 3 | — |
| `action-tag-SHAs-in-deps-snapshot` | 1 | 1 (§2.7) | — | — |
| `browser-targets` | 1 | — | — | W-163 |
| `TS language-service code fixes` | 1 | — | — | W-105 (TypeScript's API) |
| **all** | **151** | **112** | **37** | **2** |

*Unblocked* counts a rule once, under the capability that finishes it. Several
need two: `js/no-undef` and `react/jsx-no-undef` read other files *and* add an
import.

## 2 · The designs

Every design keeps three things fixed: no model and no network in the engine
(L5); every fix still passes the verifier (L4 decisions 9–11); a fix the
capability cannot prove drops a rung (L4 decision 2) rather than guessing.

### 2.1 · Computed edits — 57 rules

**What's missing.** A `fix:` template can copy captures, but it can't compute
new text from them. That covers swapping `==` for `===` but `!=` for `!==`,
stripping `.0` from a number, re-escaping a string, editing inside a comment,
or re-indenting a block it unwraps.

**The design, in three tiers, cheapest first:**

1. **ast-grep's own `transform:`**, beside `match:`. codelaw already runs on
   `ast-grep-config` (L11 D-03). Its `replace` (a regex over a capture),
   `substring` and `convert` (camelCase, kebab-case, upper and lower) give new
   captures to the template. That is no new dependency and no new syntax of
   codelaw's own. It covers `dom-node-dataset`, `prefer-string-starts-ends-with`,
   `no-console-spaces`, `escape-case` and the case changes the naming rules
   need (§2.2).
2. **A small, closed set of codelaw filters** for what `transform:` can't do,
   each a pure Rust function with its own tests ([SR-RULE-FORMAT](../../records/0112_rule-format.md)
   decision 3b already has `|trim` and `|lower-first`):
   - `|map(==:===, !=:!==)`: a token table. It covers `eqeqeq`, `operator-assignment`,
     `logical-assignment-operators`, `no-double-comparison`,
     `consistent-existence-index-check` and `no-implicit-coercion`.
   - `|number(strip-zero-fraction | group-digits | decimal | radix-literal)`:
     it covers `no-zero-fractions`, `numeric-separators-style`,
     `use-simple-number-keys` and `prefer-numeric-literals`.
   - `|escape(js-string | template | css-ident)` and `|unescape-octal`: they
     cover `prefer-template`, `no-useless-concat`, `prefer-query-selector` and
     `no-octal-escape`.
   - `|count` (`no-unreadable-array-destructuring`), `|lookup(<data table>)`
     (`prefer-keyboard-event-key`), and `$FILE.stem` / `$FILE.relative(<path>)`
     (`no-anonymous-default-export`, `import-no-absolute-path`).

   A filter that can't produce a value skips its step, and the finding drops a
   rung. Nothing is half-written.
3. **Three structural primitives**, added to decision 3c's list:
   - `edit-text: { pattern, replace }` makes regex edits *inside* one node's
     text, at byte offsets within it. Today an edit replaces whole nodes, and a
     comment is one node. This covers the eleven `jsdoc-*` rules,
     `no-div-regex` and `no-extra-label`.
   - `unwrap-block` replaces a block with its body, re-indented by the block's
     indent delta, comments kept. It covers `no-else-return` and `py/max-depth`
     (`comment-preserving block dedent`).
   - `move: { before | after: <capture> }` and `merge-into: <capture>` move or
     merge statements or members. They cover `grouped-accessor-pairs`,
     `assignment-in-assert`, `no-inline-comments`, `parameter-properties`,
     `django-unordered-body-content-in-model`, `no-duplicate-imports`,
     `prefer-single-call` and `no-useless-spread`.

   What remains are the dozen rewrites no declarative step expresses
   (`prefer-arrow-callback`, `no-for-each`, `no-array-from-fill`,
   `no-chained-comparison`, `style-prop-object`, …). These become **native
   fixes**: the rule's `NativeRule` already returns edits, so those rules gain
   a Rust fix, as `py/no-star-import` already has.

**Verifier.** Unchanged: single file, and decisions 9–11 as they stand.
**Records.** SR-RULE-FORMAT decisions 3b and 3c.
**Prior art.** ast-grep's `transform:` is the base. ESLint and Ruff compute
every fix in code (a JavaScript or Rust function per rule). The filters keep
codelaw's fixes declarative, and so readable by an agent.

### 2.2 · Cross-file edits — 34 rules

**What's missing.** A `TextEdit` has no path (`codelaw-core` `diagnostic.rs`),
so every fix lives in the finding's own file. Two different needs hide under
one label.

**2.2a · Read other files, edit this one — 11 rules.**
The fix stays local but needs facts from elsewhere:

- star imports become the names the module exports (`py/no-star-import`,
  `nested-star-import`, `star-import-usage`);
- a missing import becomes the nearest paths that exist (`node/no-missing-import`,
  `-require`, as choices);
- an undefined name becomes the project modules that export it, plus the
  import (`js/no-undef`, `react/jsx-no-undef`, as choices);
- a default import becomes the module's named export (`js/no-default-import`);
- whether an import has side effects (`py/module-import-not-at-top`);
- whether a class is used in markup (`scss/at-extend-no-missing-placeholder`);
- and what a `/// <reference>` provides (`ts/triple-slash-reference`).

The project graph already holds the facts: every file's decls, exports,
imports, CSS classes and class tokens (`codelaw-graph` `facts.rs`). Native
rules already get a read-only view of other files (`NativeContext.file`). The
change is to give fix steps the graph: `NativeContext` gains `graph`, and
`match:` fixes gain one read-only lookup, `$GRAPH.exports(<module>)`.
**Cache:** such a finding is keyed on the content hashes of the files it read,
as `scope: project` rules already are.

**2.2b · Edit several files — 22 rules, plus the BOM.**
These are the renames:

- CSS classes, ids, custom properties, keyframes, layers, containers and
  custom media;
- SCSS variables, mixins and placeholders;
- CSS-module classes, Stencil tags, `js/new-cap`, the React hook-prefix rules
  and `jsx-pascal-case`;
- `js/no-default-export` and its importers, `spell/known-typo-key` and
  `html/valid-id`;
- plus the two `node/no-extraneous-*` rules, which add a dependency to
  `package.json`.

`html/no-utf8-bom` is single-file, but it needs the same edit model: its
mark sits in bytes the text never sees.

- **The edit model.** A fix becomes a list of `FileEdit { path, edits }`, plus
  file-level operations (`strip-bom`; create and delete wait until a rule needs
  them). LSP's `WorkspaceEdit` and SARIF's `artifactChanges` already carry
  several files. JSON and MCP gain `path` on each edit set.
- **`rename` driven by the graph's references, per kind.** An export goes to
  its importers. A CSS class goes to markup and to class tokens in scripts. A
  custom property goes to `var()` and `setProperty` strings. A keyframe goes to
  `animation-name`. An SCSS member goes to its `@use` users. An id goes to
  `#id`, `href`, `for` and `aria-*`. The new name comes from the rule's case
  convention, with 2.1's `convert`. A pattern that isn't a case convention
  makes the name a `value`, so the rule stays `manual`.
- **The rung.** `auto` only when every reference is resolved statically: SCSS
  `$var` and `@mixin` through `@use`, and a JS export whose importers are all
  static (no `dynamic` module, SR-PROJECT-GRAPH decision 4). A rename that
  touches a class token in a string is **never** `auto`: those tokens are
  words that *may* name a class. Everything else is `suggest`.
- **The verifier, across files.** Every file touched still parses. A second
  run changes nothing. The rule no longer fires. Bytes outside the edits are
  identical in every file. The project graph has no new unresolved import or
  reference. The write is all-or-nothing: every file, or none.
- **Surfaces.** A multi-file fix never runs on save or from the agent's
  per-edit hook, because it rewrites files nobody has open (W-163's `on_edit`,
  forced `false`). `codelaw fix` applies it, and LSP offers it as a code
  action.

**Records.** SR-LAW-4 decisions 8, 9, 10 and 12 (needs a ruling, §5);
SR-PROJECT-GRAPH; SR-CLI-AND-MCP; SR-CACHE.
**Prior art.** LSP rename and `WorkspaceEdit`; TypeScript's language service
(`getEditsForFileRename`, rename returning `FileTextChanges`). ESLint and Ruff
stay single-file by design: a rule there sees one file.

### 2.3 · Add an import — 7 rules

**What's missing.** A fix that uses a name the file hasn't imported:
`field(default_factory=list)`, `ClassVar`, `math.isclose`, `next/head`, `<Link>`.

**The design.** A primitive, `ensure-import: { module, name, as? }`:

- If the file already imports the module, add the specifier, in place if the
  block is sorted.
- Otherwise add a new import after the last one. In Python, that comes after
  `from __future__` and the docstring. In JavaScript, it comes after the
  directives.
- If the name is already bound to something else, import it under an alias
  from 2.4's fresh-name generator, or drop a rung.
- Order is left to the import-sort rules: the fix loop runs to a fixpoint
  (decision 9).

It builds on `collect-into-block` and `move-to: after-imports`, which already
exist.
**Records.** SR-RULE-FORMAT decision 3c.
**Prior art.** Ruff's `Importer` (`get_or_import_symbol`), which reuses an
existing import before adding one; TypeScript's "Add missing import" code fix.

### 2.4 · Scope rename and a fresh name — 7 rules

**What's missing.** `rename-symbol-local` renames across a whole file and
refuses exports. These rules need a rename limited to one binding's scope (a
parameter or a catch parameter). Several also need a **new name the code
doesn't contain**, such as `prefer-for-of`'s loop variable or
`prefer-rest-params`' `...args`.

**The design.**

- `rename-symbol: { scope: binding }` uses the semantic model's scopes
  (`codelaw-semantic`).
- `$FRESH(<base>)` gives the first of `base`, `base2`, … that no enclosing or
  inner scope declares or references, and that isn't a global.
- If any name would collide, the step is refused and the finding drops to
  `suggest`.

**Records.** SR-RULE-FORMAT decision 3c.
**Prior art.** Ruff's `Renamer`, which renames a binding and its references
and handles shadowing; Biome's `rename_node_declaration`.

### 2.5 · Purity — 2 rules

**What's missing.** `js/no-unused-vars` and `no-unused-private-class-members`
may delete an initialiser only if evaluating it has no effect.

**The design.** A `pure` predicate, conservative by construction. These are
pure:

- literals and names;
- function, arrow and class expressions with no computed keys and no static
  blocks;
- array and object literals of pure parts, with no spread;
- template literals whose parts are primitive literals;
- operators applied to literals.

Anything else is impure, because a member access can run a getter and an
operator on an object can run `valueOf`. An `/*#__PURE__*/` annotation is the
author's claim, so it gives `suggest`, never `auto`.

There's a better fallback for the impure case: `const x = f()` becomes
`f();`. It keeps the effect, drops the binding, and works as `suggest`.
**Records.** SR-RULE-FORMAT decision 3a (a new predicate).
**Prior art.** Ruff's `contains_effect`, which gates F841's fix the same way;
Oxc's `may_have_side_effects`; the `__PURE__` annotation honoured by bundlers.

### 2.6 · Type information — 4 rules

**What's missing.** The type questions answer yes or no
([SR-TYPES](../../records/0117_types.md) decision 4). A fix that writes a type
needs the type's *text*.

**The design.** Two new questions, under the 2026-10-29 ruling that agents may
add questions:

- `inferred-type` gives the checker's `typeToString` of the node's type, or of
  the function's return type, for use in a template as `$TYPE.<capture>`. It is
  offered only when the text names nothing the file would have to import, and
  contains no `any`, no anonymous shape and no `typeof import(…)`. Otherwise
  the finding drops a rung. This covers `explicit-function-return-type`,
  `explicit-module-boundary-types` and `no-implicit-any-let`, all as `suggest`:
  a public API's type is still the author's to accept.
- `suppresses-error` asks whether a TypeScript diagnostic starts on the line
  after a `@ts-ignore`. The helper already returns diagnostics. If none does,
  the comment is removed (`auto`); otherwise `@ts-expect-error` replaces it
  (`suggest`). That covers `ts/ban-ts-comment`.

**Depends on** W-105: TypeScript 7.1's RC is due 2026-11-10 and GA on
2026-11-24.
**Records.** SR-TYPES decision 4.
**Prior art.** typescript-eslint fixers use `checker.typeToString`, and
TypeScript has its own "Infer return type" refactor. TypeScript 7.1's API has
no code fixes yet, which is why `ts/type-error` waits on W-105, not here.

### 2.7 · Action tags pinned to SHAs — 1 rule

`data/gha-pin-sha` needs the commit a tag points to. That takes the network,
and the engine never uses it (L5). The fix: `codelaw deps refresh`, the one
networked command, records each action tag's SHA in the deps snapshot. The
rule then reads the snapshot offline and offers `suggest`. If the snapshot has
no entry, the finding stays `manual`.
**Records.** SR-LAW-5 decision (the snapshot's contents) and SR-RULE-PACKS
decision 15.

## 3 · 37 rules no capability unblocks — re-judge now

Their own `fix_reason` names a decision, not a missing edit: "the allowed
destinations depend on the application", "what to do with the error is a
decision". Building the capability wouldn't move them. L4 decision 5 says such
a rule is `manual` with its exception, so **W-162's first step marks them**.
Each move is a one-line change.

| capability they were filed under | rules | exception |
|---|---|---|
| `add-import` | `sec-js/no-child-process-shell`, `no-dangerously-set-inner-html`, `no-nosql-injection`, `no-ldap-injection`; `sec-py/no-ldap-injection`, `no-open-redirect`, `urlopen-scheme`, `no-silent-except`, `no-exec-eval`, `no-stack-trace-response`, `no-mark-safe`, `no-wildcard-injection` | `security` (a suggested edit would read as "made safe") |
| `cross-file` | `sec-ts/trpc-public-mutation`, `nest-missing-guard`, `nest-throttler`, `sec-html/form-csrf-token` | `security` |
| `cross-file` | `css/no-empty-source`, `spell/suspicious-word`, `html/deprecated`, `node/no-unpublished-import`, `node/no-unpublished-require` | `value` |
| `cross-file` | `sec-py/no-xmlrpc`, `sec-py/no-pycrypto` | `restructure` |
| `computed-edit` | `js/no-control-regex`, `js/no-octal`, `nextjs/no-duplicate-head`, `react/hook-use-state` | `value` |
| `computed-edit` | `react/function-component-definition` | `restructure` |
| `computed-edit` | `js/prefer-add-event-listener` | `value` (its reason is behaviour, not computation) |
| `scope-rename` | `js/no-dupe-args`, `django/django-locals-in-render-function` | `value` |
| `type-info` | `ts/no-misused-new`, `ts/no-unsafe-function-type` | `value` |
| `type-info` | `sec-ts/angular-bypass-security-trust` | `security` |
| `purity` | `js/no-constant-condition`, `js/no-dupe-else-if`, `js/promise-valid-params` | `value` |

These are judgments, as every exception is. The W-162 agent re-reads each rule
before marking it. A rule it finds a real edit for keeps its `fix_pending:`.

## 4 · Build order for W-162

Ranked by rules unlocked for the work, with dependencies first:

| # | capability | rules | size | why here |
|---|---|---|---|---|
| 0 | re-judge §3 | 37 | XS | no engine work; the waiting count falls by a quarter on day one |
| 1 | computed edits (2.1) | 57 | M | the most rules; no dependency, no law change; tier 1 is configuration of what's already there |
| 2 | scope rename + `$FRESH` (2.4) | 7 | S | the fresh-name generator is reused by 2.1 (`prefer-for-of`) and 2.3 (aliases) |
| 3 | add an import (2.3) | 7 | S–M | needed by 2.2a's `no-undef` choices |
| 4 | purity (2.5) | 2 | S | small; generally useful beyond these two |
| 5 | type information (2.6) | 4 | S–M | after TypeScript 7.1's RC (W-105, 2026-11-10) |
| 6 | cross-file read (2.2a) | 11 | M | the graph is already built; needs 3 |
| 7 | cross-file write (2.2b) | 23 | L | the only part that amends L4; after the ruling in §5 |
| 8 | action-tag SHAs (2.7) | 1 | S | any time; it touches `deps refresh`, not the engine |

## 5 · One question for the decider

**Multi-file fixes amend L4.** Decision 10 says "fixes touch only their own
range", and decision 12 lets the agent hook apply `auto` fixes. 2.2b needs:

- fixes that edit several files, verified per file plus a graph check, written
  all-or-nothing;
- `auto` only when every reference resolves statically;
- never on save or from the per-edit hook.

**Recommended: allow it as written.** The alternative is to keep fixes
single-file and mark the 22 multi-file rules `manual` with `restructure`. That's honest,
but it leaves every rename to the agent by hand: the most error-prone edit it
makes.

## 6 · Hazards

- **Class tokens are guesses.** The graph's `class_tokens` are words that may
  name a class. A rename built on them is `suggest`, always.
- **Type information sits on W-105's path** (the project's own TypeScript via
  `codelaw-types`). Build on it, not beside it.
- **L8.** 2.2a makes some single-file rules read the graph. Measure the
  `check` budget before and after.
- **Two capabilities in one rule.** `no-undef` and `jsx-no-undef` move only
  when both 2.2a and 2.3 have landed.

## Reference

[W-161](../../archive/open/W-161-fix-capabilities-research.md) ·
[W-162](../open/W-162-fix-capabilities-build.md) ·
[W-156](../open/W-156-fix-ladder.md) B4 ·
[SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) ·
[SR-RULE-FORMAT](../../records/0112_rule-format.md) 3a–3c ·
[SR-TYPES](../../records/0117_types.md) 4 ·
[SR-PROJECT-GRAPH](../../records/0103_project-graph.md) ·
[`fix-ladder.md`](fix-ladder.md) — the 2026-10-30 review ·
[ast-grep `transform`](https://ast-grep.github.io/guide/rewrite/transform.html)
