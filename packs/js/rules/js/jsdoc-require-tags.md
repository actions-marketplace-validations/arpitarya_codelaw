---
id: js/jsdoc-require-tags
title: "JSDoc blocks carry the tags the project requires"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the content of the missing tag is the author's to write"
fix_exception: authored
stability: preview
options:
  tags: { type: "string[]", default: [], description: "tags every JSDoc block must carry" }
  tagContexts: { type: "object", default: {}, description: "a tag to the node kind of the code whose block must carry it (`function_declaration` \u2026)" }
ported_from: [eslint-plugin-jsdoc/require-tags]
since: 0.5.2
---

A JSDoc block that lacks a tag the project lists is reported, once per missing tag, on the block's first line: `tags = ["since"]` asks every block for `@since`. `tagContexts` asks for a tag only on blocks documenting one kind of code (`tagContexts = { returns = "function_declaration" }`). A block is checked wherever it stands, a file header's too, unless the tag has a context. The message is `Missing required tag "<tag>"`. A `context` is a tree-sitter node kind of the code a block documents (`function_declaration`, `class_declaration`, `lexical_declaration`, `method_definition` …; the declaration under an `export`, and the function or class a variable holds, count too) or `any`. eslint-plugin-jsdoc 65.4.2's `require-tags`, read and reimplemented; where upstream reports that its `tags` option is missing, this rule says nothing.

With no `tags` or `tagContexts` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

In `codelaw.toml`, under `[rules]`: `"js/jsdoc-require-tags" = { severity = "error", options = { tags = ["since"] } }`.

**Why:** A project asks for a tag on every block (`@since`, `@public`) so that its documentation tooling finds it.

```codelaw
native: js/jsdoc-require-tags
```

## How to fix

1. Add the tag the message names, with its content.

## Examples

### JavaScript

#### Don't

```js options={"tags":["since"]}
/** Runs the job. */
function run() {}
```

#### Do

```js options={"tags":["since"]}
/**
 * Runs the job.
 * @since 1.2.0
 */
function run() {}
```

### JSX

#### Don't

```jsx options={"tags":["since"]}
/** Runs the job. */
function run() {}
```

#### Do

```jsx options={"tags":["since"]}
/**
 * Runs the job.
 * @since 1.2.0
 */
function run() {}
```

### TypeScript

#### Don't

```ts options={"tags":["since"]}
/** Runs the job. */
function run() {}
```

#### Do

```ts options={"tags":["since"]}
/**
 * Runs the job.
 * @since 1.2.0
 */
function run() {}
```

### TSX

#### Don't

```tsx options={"tags":["since"]}
/** Runs the job. */
function run() {}
```

#### Do

```tsx options={"tags":["since"]}
/**
 * Runs the job.
 * @since 1.2.0
 */
function run() {}
```
