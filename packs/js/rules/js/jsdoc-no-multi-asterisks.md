---
id: js/jsdoc-no-multi-asterisks
title: "One asterisk per JSDoc line"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "codelaw's edits replace whole nodes, and a comment is one node, so the edit inside its text is the author's (the extra `*` is removed)"
fix_pending: "auto; needs computed-edit"
stability: preview
ported_from: [eslint-plugin-jsdoc/no-multi-asterisks]
since: 0.5.0
---

Each line of a JSDoc block starts with a single `*`, and the block ends `*/`, not `**/`. Markdown bold after the `* ` (` * **Note**`) is fine.

**Why:** An extra asterisk becomes part of the description text (or bold markup) in rendered docs.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - any:
        - regex: '\n[ \t]*\*\*'
        - regex: '[ \t]\*\*+/$'
message: "Start each JSDoc line with one `*`, and close the block with `*/`"
```

## How to fix

1. Remove the extra `*` at the start of the line, or before the closing `/`.

## Examples

### JavaScript

#### Don't

```js
/**
 ** Starts the server.
 */
function start() {}
```

#### Do

```js
/**
 * Starts the server.
 */
function start() {}
```

### JSX

#### Don't

```jsx
/**
 ** Starts the server.
 */
function start() {}
```

#### Do

```jsx
/**
 * Starts the server.
 */
function start() {}
```

### TypeScript

#### Don't

```ts
/**
 ** Starts the server.
 */
function start() {}
```

#### Do

```ts
/**
 * Starts the server.
 */
function start() {}
```

### TSX

#### Don't

```tsx
/**
 ** Starts the server.
 */
function start() {}
```

#### Do

```tsx
/**
 * Starts the server.
 */
function start() {}
```
