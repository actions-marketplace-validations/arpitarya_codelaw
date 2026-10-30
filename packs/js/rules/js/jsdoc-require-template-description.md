---
id: js/jsdoc-require-template-description
title: "@template says what the type parameter is"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's words"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-template-description]
since: 0.5.0
---

A JSDoc `@template` tag carries a description after the type parameter's name.

**Why:** A bare type-parameter name says nothing about what the type stands for or what it is constrained to.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@template\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\}[ \t]*)?(?:\[[^\]\n]*\]|[\w$]+(?:[ \t]*,[ \t]*[\w$]+)*)[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
message: Describe this type parameter after its name
```

## How to fix

1. After the type parameter's name, write what it stands for.

## Examples

### JavaScript

#### Don't

```js
/**
 * Wraps a value.
 * @template T
 * @param {T} value The value.
 * @returns {T[]} The box.
 */
function box(value) {
  return [value];
}
```

#### Do

```js
/**
 * Wraps a value.
 * @template T The value's type.
 * @param {T} value The value.
 * @returns {T[]} The box.
 */
function box(value) {
  return [value];
}
```

### JSX

#### Don't

```jsx
/**
 * Wraps a value.
 * @template T
 * @param {T} value The value.
 * @returns {T[]} The box.
 */
function box(value) {
  return [value];
}
```

#### Do

```jsx
/**
 * Wraps a value.
 * @template T The value's type.
 * @param {T} value The value.
 * @returns {T[]} The box.
 */
function box(value) {
  return [value];
}
```

### TypeScript

#### Don't

```ts
/**
 * Wraps a value.
 * @template T
 * @param {T} value The value.
 * @returns {T[]} The box.
 */
function box(value) {
  return [value];
}
```

#### Do

```ts
/**
 * Wraps a value.
 * @template T The value's type.
 * @param {T} value The value.
 * @returns {T[]} The box.
 */
function box(value) {
  return [value];
}
```

### TSX

#### Don't

```tsx
/**
 * Wraps a value.
 * @template T
 * @param {T} value The value.
 * @returns {T[]} The box.
 */
function box(value) {
  return [value];
}
```

#### Do

```tsx
/**
 * Wraps a value.
 * @template T The value's type.
 * @param {T} value The value.
 * @returns {T[]} The box.
 */
function box(value) {
  return [value];
}
```
