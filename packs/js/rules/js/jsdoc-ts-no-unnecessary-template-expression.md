---
id: js/jsdoc-ts-no-unnecessary-template-expression
title: "No unnecessary template expression in a JSDoc template-literal type"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { enable_fixer: { type: "boolean", default: true, description: "offer the rewrite" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/ts-no-unnecessary-template-expression]
since: 0.5.0
---

A template-literal type has no `${'text'}` with a string literal in it (the text belongs in the literal part) and is not a lone `${T}` (the type itself is meant). The fix writes the text into the template, or the type for the lone expression; the first string expression of a template is rewritten at a time, and one with an escape or a backtick in it is reported without a fix.

**Why:** An interpolated literal says in a longer way what the template already says, and a template with one placeholder is just that placeholder.

```codelaw
native: js/jsdoc-ts-no-unnecessary-template-expression
```

## How to fix

1. Move the literal's text into the template, or drop the template around a lone `${T}` (the fix does).

## Examples

### JavaScript

#### Don't

```js
/**
 * @param {`prefix-${'a'}-suffix`} x
 */
function f(x) {}
```

#### Do

```js
/**
 * @param {`prefix-a-suffix`} x
 */
function f(x) {}
```

#### Fixed

```js
/**
 * @param {`prefix-a-suffix`} x
 */
function f(x) {}
```

### JSX

#### Don't

```jsx
/**
 * @param {`prefix-${'a'}-suffix`} x
 */
function f(x) {}
```

#### Do

```jsx
/**
 * @param {`prefix-a-suffix`} x
 */
function f(x) {}
```

#### Fixed

```jsx
/**
 * @param {`prefix-a-suffix`} x
 */
function f(x) {}
```

### TypeScript

#### Don't

```ts
/**
 * @param {`prefix-${'a'}-suffix`} x
 */
function f(x) {}
```

#### Do

```ts
/**
 * @param {`prefix-a-suffix`} x
 */
function f(x) {}
```

#### Fixed

```ts
/**
 * @param {`prefix-a-suffix`} x
 */
function f(x) {}
```

### TSX

#### Don't

```tsx
/**
 * @param {`prefix-${'a'}-suffix`} x
 */
function f(x) {}
```

#### Do

```tsx
/**
 * @param {`prefix-a-suffix`} x
 */
function f(x) {}
```

#### Fixed

```tsx
/**
 * @param {`prefix-a-suffix`} x
 */
function f(x) {}
```
