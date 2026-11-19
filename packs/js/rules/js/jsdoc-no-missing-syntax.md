---
id: js/jsdoc-no-missing-syntax
title: "JSDoc blocks the project requires"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the missing documentation is the author's to write"
fix_exception: authored
stability: preview
options:
  contexts: { type: "string[]", default: [], description: "node kinds of code that must have a JSDoc block (`function_declaration`, `class_declaration` \u2026, or `any`)" }
  entries: { type: "object", default: {}, description: "named requirements, each `{ context, tag, matches, message, minimum }`" }
ported_from: [eslint-plugin-jsdoc/no-missing-syntax]
since: 0.5.2
---

The file must hold a JSDoc block for each requirement the project lists: at least `minimum` (default 1) blocks documenting code of the `context` kind (default `any`) that carry the `tag` and whose text matches `matches`. The first requirement not met is reported, once, at the start of the file; `message` replaces `Syntax is required: <context> with <tag>`, and may use `{{context}}` and `{{comment}}`. `contexts` is the short form, node kinds only; `entries` names each requirement. A `context` is a tree-sitter node kind of the code a block documents (`function_declaration`, `class_declaration`, `lexical_declaration`, `method_definition` …; the declaration under an `export`, and the function or class a variable holds, count too) or `any`. Upstream names these with esquery selectors over ESTree and the JSDoc AST, which have no equivalent here; a context here is a node kind, and what a block must hold is a `tag` it carries and a `matches` regular expression its text matches. eslint-plugin-jsdoc 65.4.2's `no-missing-syntax`, read and reimplemented; where upstream reports that its `contexts` option is missing, this rule says nothing.

With no `contexts` or `entries` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

In `codelaw.toml`, under `[rules]`: `"js/jsdoc-no-missing-syntax" = { severity = "error", options = { entries = { file = { tag = "file" } } } }`.

**Why:** A project asks that some code be documented (a `@file` block, a `@module`, every exported class) and wants the file that skips it named.

```codelaw
native: js/jsdoc-no-missing-syntax
```

## How to fix

1. Add the JSDoc block the message names.

## Examples

### JavaScript

#### Don't

```js options={"entries":{"file":{"tag":"file"}}}
export function run() {}
```

#### Do

```js options={"entries":{"file":{"tag":"file"}}}
/** @file Runs the job. */
export function run() {}
```

### JSX

#### Don't

```jsx options={"entries":{"file":{"tag":"file"}}}
export function run() {}
```

#### Do

```jsx options={"entries":{"file":{"tag":"file"}}}
/** @file Runs the job. */
export function run() {}
```

### TypeScript

#### Don't

```ts options={"entries":{"file":{"tag":"file"}}}
export function run() {}
```

#### Do

```ts options={"entries":{"file":{"tag":"file"}}}
/** @file Runs the job. */
export function run() {}
```

### TSX

#### Don't

```tsx options={"entries":{"file":{"tag":"file"}}}
export function run() {}
```

#### Do

```tsx options={"entries":{"file":{"tag":"file"}}}
/** @file Runs the job. */
export function run() {}
```
