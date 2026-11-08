---
id: js/no-unsafe-promise-all-settled-values
title: "No `.value` of a `Promise.allSettled()` result without checking its status"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what to do with a rejected result (skip it, use a fallback, collect the reason) is the author's choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-unsafe-promise-all-settled-values]
since: 0.5.0
---

In a `.map(r => …)` over the results of `await Promise.allSettled(…)` (directly, through `.filter()`, a `const`, or the parameter of `Promise.allSettled(…).then(results => …)`), reading `r.value` or destructuring `value` is reported unless it is guarded: `r.status === "fulfilled" ? r.value : …`, `&&`, `||`, an `if`, an earlier `if (r.status !== "fulfilled") return;`, or a `.filter()` that keeps only fulfilled results. A callback passed by name is not read.

**Why:** A rejected result has no `value`, so the map silently produces `undefined` for every failure.

```codelaw
native: js/no-unsafe-promise-all-settled-values
```

## How to fix

1. Check `status` first: `r.status === "fulfilled" ? r.value : fallback`.
2. Or filter the fulfilled results before mapping.

## Examples

### JavaScript

#### Don't

```js
const values = (await Promise.allSettled(tasks)).map((r) => r.value);
```

#### Do

```js
const values = (await Promise.allSettled(tasks)).map((r) => (r.status === "fulfilled" ? r.value : null));
```


### JSX

#### Don't

```jsx
const values = (await Promise.allSettled(tasks)).map((r) => r.value);
```

#### Do

```jsx
const values = (await Promise.allSettled(tasks)).map((r) => (r.status === "fulfilled" ? r.value : null));
```


### TypeScript

#### Don't

```ts
const values = (await Promise.allSettled(tasks)).map((r) => r.value);
```

#### Do

```ts
const values = (await Promise.allSettled(tasks)).map((r) => (r.status === "fulfilled" ? r.value : null));
```


### TSX

#### Don't

```tsx
const values = (await Promise.allSettled(tasks)).map((r) => r.value);
```

#### Do

```tsx
const values = (await Promise.allSettled(tasks)).map((r) => (r.status === "fulfilled" ? r.value : null));
```
