---
id: js/jsdoc-match-name
title: "JSDoc tag names the project allows"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
options:
  match: { type: "object", default: {}, description: "named checks, each `{ allowName, disallowName, replacement, tags, context, message }`" }
ported_from: [eslint-plugin-jsdoc/match-name]
since: 0.5.2
---

The name a JSDoc tag gives (`@param {string} user_name`: `user_name`) is reported when it does not match `allowName` or does match `disallowName`, for the tags in `tags` (default every tag). Each check is an entry of `match` under a name of the project's choosing; an entry with neither `allowName` nor `disallowName` reports the first tag it applies to as a prohibited context, and `context` confines an entry to blocks documenting one kind of code. A name pattern is a regular expression, optionally written `/body/flags` (`i` is the only flag read). With `disallowName` and `replacement`, the disallowed part of the name is rewritten (`^_` with the empty string drops a leading underscore) and the finding is an automatic fix; otherwise it has none. `message` replaces the default. A `context` is a tree-sitter node kind of the code a block documents (`function_declaration`, `class_declaration`, `lexical_declaration`, `method_definition` …; the declaration under an `export`, and the function or class a variable holds, count too) or `any`. Upstream names these with esquery selectors over ESTree and the JSDoc AST, which have no equivalent here; a context here is a node kind, and what a block must hold is a `tag` it carries and a `matches` regular expression its text matches. eslint-plugin-jsdoc 65.4.2's `match-name`, read and reimplemented; where upstream reports that its `match` option is missing, this rule says nothing.

With no `match` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** A project that names its parameters one way wants the documentation to say the same.

```codelaw
native: js/jsdoc-match-name
```

## Examples

### JavaScript

#### Don't

```js options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} _id
 */
```

#### Do

```js options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} id
 */
```

#### Fixed

```js options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} id
 */
```

### JSX

#### Don't

```jsx options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} _id
 */
```

#### Do

```jsx options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} id
 */
```

#### Fixed

```jsx options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} id
 */
```

### TypeScript

#### Don't

```ts options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} _id
 */
```

#### Do

```ts options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} id
 */
```

#### Fixed

```ts options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} id
 */
```

### TSX

#### Don't

```tsx options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} _id
 */
```

#### Do

```tsx options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} id
 */
```

#### Fixed

```tsx options={"match":{"prop":{"disallowName":"^_","replacement":"","tags":["property"]}}}
/**
 * @typedef {object} User
 * @property {string} id
 */
```
