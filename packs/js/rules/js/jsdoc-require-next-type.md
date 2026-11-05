---
id: js/jsdoc-require-next-type
title: "@next has a type"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the type is the author's to name; codelaw cannot infer it"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsdoc/require-next-type]
since: 0.5.0
---

A JSDoc `@next` tag starts with the `{type}` of the value sent in with `next()`.

**Why:** Without the type, tools and readers cannot check what callers may pass to `next()`.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@next(?:[ \t]+[^{\s]|[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/)))
message: Give the `@next` tag a `{type}`
```

## How to fix

1. Write the type of the value passed to `next()` in braces right after `@next`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Counts by a step the caller sends.
 * @next The step to add.
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
 * @next The step to add.
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
 * @next The step to add.
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
 * @next The step to add.
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
