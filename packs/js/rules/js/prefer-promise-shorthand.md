---
id: js/prefer-promise-shorthand
title: "Promise.resolve rather than new Promise(r => r(x))"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Promise.resolve` returns a promise given as `$X` unchanged, where the constructor wraps it"
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-promise-shorthand]
since: 0.5.0
---

A promise that only resolves with a value is written `Promise.resolve(value)`. Upstream also covers `reject`; this port covers `resolve`.

**Why:** The shorthand says the same without a callback.

```codelaw

match:
  any:
    - pattern: "new Promise(($R) => $R($X))"
    - pattern: "new Promise(($R) => { $R($X); })"
message: "Write `Promise.resolve($X)`"
fix: "Promise.resolve($X)"
```

## Examples

### JavaScript

#### Don't

```js
const p = new Promise((resolve) => resolve(42));
```

#### Do

```js
const p = Promise.resolve(42);
```

### JSX

#### Don't

```jsx
const p = new Promise((resolve) => resolve(42));
```

#### Do

```jsx
const p = Promise.resolve(42);
```

### TypeScript

#### Don't

```ts
const p = new Promise((resolve) => resolve(42));
```

#### Do

```ts
const p = Promise.resolve(42);
```

### TSX

#### Don't

```tsx
const p = new Promise((resolve) => resolve(42));
```

#### Do

```tsx
const p = Promise.resolve(42);
```
