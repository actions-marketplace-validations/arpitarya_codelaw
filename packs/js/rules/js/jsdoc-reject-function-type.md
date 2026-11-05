---
id: js/jsdoc-reject-function-type
title: "No bare Function type in JSDoc"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the function's signature is the author's to write"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsdoc/reject-function-type]
since: 0.5.0
---

A JSDoc type is not the bare `Function`; it states the signature, `(a: string) => void`, or names a `@callback`.

**Why:** `Function` accepts any callable and returns `any`, so neither the arguments nor the result are checked.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '@[A-Za-z]+[ \t]*\{[^}\n]*\bFunction\b[^}\n]*\}'
message: "Write the function's signature, such as `(err: Error) => void`, instead of `Function`"
```

## How to fix

1. Replace `Function` with the signature the code calls it with, e.g. `{(err: Error) => void}`, or define a `@callback` and use its name.

## Examples

### JavaScript

#### Don't

```js
/**
 * Runs a job.
 * @param {Function} done called when finished
 */
function run(done) {}
```

#### Do

```js
/**
 * Runs a job.
 * @param {(err: Error) => void} done called when finished
 */
function run(done) {}
```

### JSX

#### Don't

```jsx
/**
 * Runs a job.
 * @param {Function} done called when finished
 */
function run(done) {}
```

#### Do

```jsx
/**
 * Runs a job.
 * @param {(err: Error) => void} done called when finished
 */
function run(done) {}
```

### TypeScript

#### Don't

```ts
/**
 * Runs a job.
 * @param {Function} done called when finished
 */
function run(done) {}
```

#### Do

```ts
/**
 * Runs a job.
 * @param {(err: Error) => void} done called when finished
 */
function run(done) {}
```

### TSX

#### Don't

```tsx
/**
 * Runs a job.
 * @param {Function} done called when finished
 */
function run(done) {}
```

#### Do

```tsx
/**
 * Runs a job.
 * @param {(err: Error) => void} done called when finished
 */
function run(done) {}
```
