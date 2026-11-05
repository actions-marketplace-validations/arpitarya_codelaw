---
id: js/jsdoc-require-returns-description
title: "@returns says what is returned"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's words"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/require-returns-description]
since: 0.5.0
---

A JSDoc `@returns` tag carries a description, unless its type is `{void}`, `{undefined}` or `{Promise<void>}`.

**Why:** A type alone rarely says what the returned value means.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@returns?\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\}[ \t]*)?[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
  - not:
      regex: '@returns?[ \t]*\{(?:void|undefined|Promise<void>)\}'
message: Describe the return value after the `@returns` type
```

## How to fix

1. After the `@returns` type, write what the value is.

## Examples

### JavaScript

#### Don't

```js
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number}
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```js
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```

### JSX

#### Don't

```jsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number}
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```jsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```

### TypeScript

#### Don't

```ts
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number}
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```ts
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```

### TSX

#### Don't

```tsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number}
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```tsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```
