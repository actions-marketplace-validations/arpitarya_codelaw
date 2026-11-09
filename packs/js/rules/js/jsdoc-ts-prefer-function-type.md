---
id: js/jsdoc-ts-prefer-function-type
title: "A function type, not a lone call signature, in JSDoc"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { enable_fixer: { type: "boolean", default: true, description: "offer the rewrite" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/ts-prefer-function-type]
since: 0.5.0
---

An object type with one member, a call signature — `{(a: string): void}` — is written as the function type `(a: string) => void`, parenthesised where it sits in a union, an intersection or an array's element. An object with other members, and a construct signature (`{new (a): T}`), are not reported. A call signature with no return type is reported without a fix.

**Why:** A function type says the same thing in one line; the object form is for a function that also has properties.

```codelaw
native: js/jsdoc-ts-prefer-function-type
```

## How to fix

1. Write the signature as an arrow function type (the fix does).

## Examples

### JavaScript

#### Don't

```js
/**
 * @param {{(event: string): void}} listener
 */
function on(listener) {}
```

#### Do

```js
/**
 * @param {(event: string) => void} listener
 */
function on(listener) {}
```

#### Fixed

```js
/**
 * @param {(event: string) => void} listener
 */
function on(listener) {}
```

### JSX

#### Don't

```jsx
/**
 * @param {{(event: string): void}} listener
 */
function on(listener) {}
```

#### Do

```jsx
/**
 * @param {(event: string) => void} listener
 */
function on(listener) {}
```

#### Fixed

```jsx
/**
 * @param {(event: string) => void} listener
 */
function on(listener) {}
```

### TypeScript

#### Don't

```ts
/**
 * @param {{(event: string): void}} listener
 */
function on(listener) {}
```

#### Do

```ts
/**
 * @param {(event: string) => void} listener
 */
function on(listener) {}
```

#### Fixed

```ts
/**
 * @param {(event: string) => void} listener
 */
function on(listener) {}
```

### TSX

#### Don't

```tsx
/**
 * @param {{(event: string): void}} listener
 */
function on(listener) {}
```

#### Do

```tsx
/**
 * @param {(event: string) => void} listener
 */
function on(listener) {}
```

#### Fixed

```tsx
/**
 * @param {(event: string) => void} listener
 */
function on(listener) {}
```
