---
id: js/jsdoc-require-yields-type
title: "@yields has a type"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the type is the author's to name; codelaw cannot infer it"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsdoc/require-yields-type]
since: 0.5.0
---

A JSDoc `@yields` tag gives the `{type}` of the yielded values.

**Why:** Without the type, consumers of the generator cannot check what they receive.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@yields?(?:[ \t]+[^{\s]|[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/)))
message: Give the `@yields` tag a `{type}`
```

## How to fix

1. Write the yielded values' type in braces right after `@yields`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Counts from zero.
 * @yields The next id.
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
 * @yields The next id.
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
 * @yields The next id.
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
 * @yields The next id.
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
