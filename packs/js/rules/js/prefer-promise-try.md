---
id: js/prefer-promise-try
title: "Promise.try() rather than a promise that resolves to a call"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-promise-try]
since: 0.5.0
---

A promise made only to run a function and settle with its result, `new Promise((resolve) => resolve(fn()))`, is `Promise.try(fn)`.

**Why:** `Promise.try` runs the function now, turns a throw into a rejection and says so in its name.

```codelaw
match:
  any:
    - pattern: "new Promise(($R) => $R($F()))"
    - pattern: "new Promise(($R) => { $R($F()); })"
constraints:
  F: { kind: identifier }
message: "Use `Promise.try($F)`"
fix: "Promise.try($F)"
```

## Examples

### JavaScript

#### Don't

```js
const result = new Promise((resolve) => resolve(load()));
```

#### Do

```js
const result = Promise.try(load);
```


### JSX

#### Don't

```jsx
const result = new Promise((resolve) => resolve(load()));
```

#### Do

```jsx
const result = Promise.try(load);
```


### TypeScript

#### Don't

```ts
const result = new Promise((resolve) => resolve(load()));
```

#### Do

```ts
const result = Promise.try(load);
```


### TSX

#### Don't

```tsx
const result = new Promise((resolve) => resolve(load()));
```

#### Do

```tsx
const result = Promise.try(load);
```
