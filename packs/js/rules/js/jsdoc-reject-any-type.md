---
id: js/jsdoc-reject-any-type
title: "No any or * types in JSDoc"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the real type is the author's to name"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsdoc/reject-any-type]
since: 0.5.0
---

A JSDoc type is not `any` or `*`, alone or inside a union or generic.

**Why:** `any` switches type checking off for the value and everything it flows into, so the documented API promises nothing.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '@[A-Za-z]+[ \t]*\{[^}\n]*(\bany\b|\*)[^}\n]*\}'
message: "Name the real type instead of `any` or `*`; use `unknown` if it really can be anything"
```

## How to fix

1. Write the type the value actually has, or `unknown` if it really can be anything and is checked before use.

## Examples

### JavaScript

#### Don't

```js
/**
 * Logs a value.
 * @param {*} value the value
 */
function log(value) {}
```

#### Do

```js
/**
 * Logs a value.
 * @param {string} value the value
 */
function log(value) {}
```

### JSX

#### Don't

```jsx
/**
 * Logs a value.
 * @param {*} value the value
 */
function log(value) {}
```

#### Do

```jsx
/**
 * Logs a value.
 * @param {string} value the value
 */
function log(value) {}
```

### TypeScript

#### Don't

```ts
/**
 * Logs a value.
 * @param {*} value the value
 */
function log(value) {}
```

#### Do

```ts
/**
 * Logs a value.
 * @param {string} value the value
 */
function log(value) {}
```

### TSX

#### Don't

```tsx
/**
 * Logs a value.
 * @param {*} value the value
 */
function log(value) {}
```

#### Do

```tsx
/**
 * Logs a value.
 * @param {string} value the value
 */
function log(value) {}
```
