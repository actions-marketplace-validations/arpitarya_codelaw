---
id: js/jsdoc-no-restricted-syntax
title: "No JSDoc block the project restricts"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what replaces the restricted block is the project's convention"
fix_exception: value
stability: preview
options:
  contexts: { type: "string[]", default: [], description: "node kinds of code whose JSDoc block is reported (`function_declaration` \u2026, or `any`)" }
  entries: { type: "object", default: {}, description: "named restrictions, each `{ context, tag, matches, message }`" }
ported_from: [eslint-plugin-jsdoc/no-restricted-syntax]
since: 0.5.2
---

A JSDoc block is reported when it documents code of an entry's `context` kind (default `any`), carries its `tag` and its text matches `matches`; `message` replaces `Syntax is restricted: <context> with <tag>`, and may use `{{context}}` and `{{comment}}`. `contexts` is the short form, node kinds only, reporting every block on that code; `entries` names each restriction. A `context` is a tree-sitter node kind of the code a block documents (`function_declaration`, `class_declaration`, `lexical_declaration`, `method_definition` …; the declaration under an `export`, and the function or class a variable holds, count too) or `any`. Upstream names these with esquery selectors over ESTree and the JSDoc AST, which have no equivalent here; a context here is a node kind, and what a block must hold is a `tag` it carries and a `matches` regular expression its text matches. eslint-plugin-jsdoc 65.4.2's `no-restricted-syntax`, read and reimplemented; where upstream reports that its `contexts` option is missing, this rule says nothing.

With no `contexts` or `entries` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** A TypeScript project keeps types out of its comments, or a project retires a tag; the rule finds the blocks that still carry it.

```codelaw
native: js/jsdoc-no-restricted-syntax
```

## How to fix

1. Remove or rewrite the block as the project's message says.

## Examples

### JavaScript

#### Don't

```js options={"entries":{"typeTag":{"tag":"type"}}}
/** @type {string} */
const name = "x";
```

#### Do

```js options={"entries":{"typeTag":{"tag":"type"}}}
/** The name. */
const name = "x";
```

### JSX

#### Don't

```jsx options={"entries":{"typeTag":{"tag":"type"}}}
/** @type {string} */
const name = "x";
```

#### Do

```jsx options={"entries":{"typeTag":{"tag":"type"}}}
/** The name. */
const name = "x";
```

### TypeScript

#### Don't

```ts options={"entries":{"typeTag":{"tag":"type"}}}
/** @type {string} */
const name = "x";
```

#### Do

```ts options={"entries":{"typeTag":{"tag":"type"}}}
/** The name. */
const name = "x";
```

### TSX

#### Don't

```tsx options={"entries":{"typeTag":{"tag":"type"}}}
/** @type {string} */
const name = "x";
```

#### Do

```tsx options={"entries":{"typeTag":{"tag":"type"}}}
/** The name. */
const name = "x";
```
