---
id: js/jsdoc-multiline-blocks
title: "/** and */ on their own lines"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "codelaw's edits replace whole nodes, and a comment is one node, so the edit inside its text is the author's (the text moves to a line of its own)"
stability: preview
ported_from: [eslint-plugin-jsdoc/multiline-blocks]
since: 0.5.0
---

In a JSDoc block that spans lines, nothing follows `/**` on the first line and nothing precedes `*/` on the last. Single-line blocks (`/** @type {number} */`) are fine.

**Why:** Text on the delimiter lines is easy to miss and is parsed inconsistently by documentation tools.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - any:
        - regex: '^/\*\*[ \t]*[^\s*][^\n]*\n'
        - regex: '\n[ \t]*(\*[ \t]*)?[^\s*][^\n]*\*/$'
message: "In a multi-line JSDoc block, keep `/**` and `*/` on lines of their own"
```

## How to fix

1. Move the text after `/**` to the next line, prefixed with ` * `.
2. Move the text before `*/` to a line of its own, and leave ` */` alone on the last line.

## Examples

### JavaScript

#### Don't

```js
/** Adds two numbers.
 * @param {number} a the first
 * @param {number} b the second */
function add(a, b) {
  return a + b;
}
```

#### Do

```js
/**
 * Adds two numbers.
 * @param {number} a the first
 * @param {number} b the second
 */
function add(a, b) {
  return a + b;
}
```

### JSX

#### Don't

```jsx
/** Adds two numbers.
 * @param {number} a the first
 * @param {number} b the second */
function add(a, b) {
  return a + b;
}
```

#### Do

```jsx
/**
 * Adds two numbers.
 * @param {number} a the first
 * @param {number} b the second
 */
function add(a, b) {
  return a + b;
}
```

### TypeScript

#### Don't

```ts
/** Adds two numbers.
 * @param {number} a the first
 * @param {number} b the second */
function add(a, b) {
  return a + b;
}
```

#### Do

```ts
/**
 * Adds two numbers.
 * @param {number} a the first
 * @param {number} b the second
 */
function add(a, b) {
  return a + b;
}
```

### TSX

#### Don't

```tsx
/** Adds two numbers.
 * @param {number} a the first
 * @param {number} b the second */
function add(a, b) {
  return a + b;
}
```

#### Do

```tsx
/**
 * Adds two numbers.
 * @param {number} a the first
 * @param {number} b the second
 */
function add(a, b) {
  return a + b;
}
```
