---
id: js/jsdoc-ts-no-empty-object-type
title: "No {} type in JSDoc"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether `object`, `unknown` or a real shape is meant is the author's call"
stability: preview
ported_from: [eslint-plugin-jsdoc/ts-no-empty-object-type]
since: 0.5.0
---

A JSDoc type does not use the empty object type `{}` (as `{{}}` or inside a generic such as `{Array<{}>}`); `object`, `unknown` or a written-out shape is used instead.

**Why:** `{}` does not mean "an empty object": it accepts every value except `null` and `undefined`, so it checks nothing.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@\w+[ \t]*\{[^{}\n]*\{[ \t]*\}
message: The `{}` type accepts almost anything; write `object`, `unknown` or the shape you mean
```

## How to fix

1. Replace `{}` with `object` for any non-primitive, `unknown` for any value, or the object's real shape.

## Examples

### JavaScript

#### Don't

```js
/**
 * Merges options.
 * @param {{}} opts The options.
 */
function merge(opts) {}
```

#### Do

```js
/**
 * Merges options.
 * @param {object} opts The options.
 */
function merge(opts) {}
```

### JSX

#### Don't

```jsx
/**
 * Merges options.
 * @param {{}} opts The options.
 */
function merge(opts) {}
```

#### Do

```jsx
/**
 * Merges options.
 * @param {object} opts The options.
 */
function merge(opts) {}
```

### TypeScript

#### Don't

```ts
/**
 * Merges options.
 * @param {{}} opts The options.
 */
function merge(opts) {}
```

#### Do

```ts
/**
 * Merges options.
 * @param {object} opts The options.
 */
function merge(opts) {}
```

### TSX

#### Don't

```tsx
/**
 * Merges options.
 * @param {{}} opts The options.
 */
function merge(opts) {}
```

#### Do

```tsx
/**
 * Merges options.
 * @param {object} opts The options.
 */
function merge(opts) {}
```
