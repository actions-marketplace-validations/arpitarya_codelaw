---
id: js/jsdoc-require-next-description
title: "@next says what is sent in"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's words"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-next-description]
since: 0.5.0
---

A JSDoc `@next` tag carries a description of the value the generator receives from `next()`.

**Why:** The type alone rarely says what the value sent in means to the generator.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@next\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\}[ \t]*)?[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
message: Describe what `next()` sends in after the `@next` type
```

## How to fix

1. After the `@next` type, write what the caller sends in with `next()` and how the generator uses it.

## Examples

### JavaScript

#### Don't

```js
/**
 * Counts by a step the caller sends.
 * @next {number}
 */
function* counter() {
  let n = 0;
  while (true) n += yield n;
}
```

#### Do

```js
/**
 * Counts by a step the caller sends.
 * @next {number} The step to add.
 */
function* counter() {
  let n = 0;
  while (true) n += yield n;
}
```

### JSX

#### Don't

```jsx
/**
 * Counts by a step the caller sends.
 * @next {number}
 */
function* counter() {
  let n = 0;
  while (true) n += yield n;
}
```

#### Do

```jsx
/**
 * Counts by a step the caller sends.
 * @next {number} The step to add.
 */
function* counter() {
  let n = 0;
  while (true) n += yield n;
}
```

### TypeScript

#### Don't

```ts
/**
 * Counts by a step the caller sends.
 * @next {number}
 */
function* counter() {
  let n = 0;
  while (true) n += yield n;
}
```

#### Do

```ts
/**
 * Counts by a step the caller sends.
 * @next {number} The step to add.
 */
function* counter() {
  let n = 0;
  while (true) n += yield n;
}
```

### TSX

#### Don't

```tsx
/**
 * Counts by a step the caller sends.
 * @next {number}
 */
function* counter() {
  let n = 0;
  while (true) n += yield n;
}
```

#### Do

```tsx
/**
 * Counts by a step the caller sends.
 * @next {number} The step to add.
 */
function* counter() {
  let n = 0;
  while (true) n += yield n;
}
```
