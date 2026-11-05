---
id: js/jsdoc-require-yields-description
title: "@yields says what is yielded"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's words"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/require-yields-description]
since: 0.5.0
---

A JSDoc `@yields` tag carries a description of the yielded values.

**Why:** A type alone rarely says what each yielded value means.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@yields?\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\}[ \t]*)?[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
message: Describe the yielded values after the `@yields` type
```

## How to fix

1. After the `@yields` type, write what each yielded value is.

## Examples

### JavaScript

#### Don't

```js
/**
 * Counts from zero.
 * @yields {number}
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

#### Do

```js
/**
 * Counts from zero.
 * @yields {number} The next id.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

### JSX

#### Don't

```jsx
/**
 * Counts from zero.
 * @yields {number}
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

#### Do

```jsx
/**
 * Counts from zero.
 * @yields {number} The next id.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

### TypeScript

#### Don't

```ts
/**
 * Counts from zero.
 * @yields {number}
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

#### Do

```ts
/**
 * Counts from zero.
 * @yields {number} The next id.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

### TSX

#### Don't

```tsx
/**
 * Counts from zero.
 * @yields {number}
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

#### Do

```tsx
/**
 * Counts from zero.
 * @yields {number} The next id.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```
