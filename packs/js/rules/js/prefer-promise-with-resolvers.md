---
id: js/prefer-promise-with-resolvers
title: "`Promise.withResolvers()` for a promise whose resolvers are taken out"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-promise-with-resolvers]
since: 0.5.0
---

A `new Promise((resolve, reject) => { … })` whose executor does nothing but copy its resolvers into outer variables (`res = resolve;`) is reported. When it is the only declarator of `const p = new Promise(…)`, directly after `let` declarations of exactly those variables (no values, no annotations) that nothing else writes, the fix replaces them all with `const { promise: p, resolve: res } = Promise.withResolvers();` (type arguments kept).

**Why:** `Promise.withResolvers()` says what the code does — get a promise and its resolvers — without a callback and `let`s.

```codelaw
native: js/prefer-promise-with-resolvers
```

## Examples

### JavaScript

#### Don't

```js
let resolveReady;
const ready = new Promise((resolve) => {
  resolveReady = resolve;
});
```

#### Do

```js
const { promise: ready, resolve: resolveReady } = Promise.withResolvers();
```

#### Fixed

```js
const { promise: ready, resolve: resolveReady } = Promise.withResolvers();
```


### JSX

#### Don't

```jsx
let resolveReady;
const ready = new Promise((resolve) => {
  resolveReady = resolve;
});
```

#### Do

```jsx
const { promise: ready, resolve: resolveReady } = Promise.withResolvers();
```

#### Fixed

```jsx
const { promise: ready, resolve: resolveReady } = Promise.withResolvers();
```


### TypeScript

#### Don't

```ts
let resolveReady;
const ready = new Promise((resolve) => {
  resolveReady = resolve;
});
```

#### Do

```ts
const { promise: ready, resolve: resolveReady } = Promise.withResolvers();
```

#### Fixed

```ts
const { promise: ready, resolve: resolveReady } = Promise.withResolvers();
```


### TSX

#### Don't

```tsx
let resolveReady;
const ready = new Promise((resolve) => {
  resolveReady = resolve;
});
```

#### Do

```tsx
const { promise: ready, resolve: resolveReady } = Promise.withResolvers();
```

#### Fixed

```tsx
const { promise: ready, resolve: resolveReady } = Promise.withResolvers();
```
