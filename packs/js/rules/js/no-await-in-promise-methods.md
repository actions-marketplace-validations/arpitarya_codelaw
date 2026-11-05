---
id: js/no-await-in-promise-methods
title: "No await inside Promise.all([...])"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the awaited promise no longer finishes before the others start"
stability: preview
ported_from: [eslint-plugin-unicorn/no-await-in-promise-methods]
since: 0.5.0
---

An element of the array given to `Promise.all`, `allSettled`, `any` or `race` is not `await`ed.

**Why:** Awaiting an element serialises it before the others start, which defeats the point of running them together.

```codelaw

match:
  kind: await_expression
  pattern: await $X
  inside:
    kind: array
    inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^Promise\.(all|allSettled|any|race)$' } } }
message: "`await` here makes the promises run one after another; pass the promise itself"
fix: "$X"
```

## Examples

### JavaScript

#### Don't

```js
const [a, b] = await Promise.all([await loadA(), loadB()]);
```

#### Do

```js
const [a, b] = await Promise.all([loadA(), loadB()]);
```

### JSX

#### Don't

```jsx
const [a, b] = await Promise.all([await loadA(), loadB()]);
```

#### Do

```jsx
const [a, b] = await Promise.all([loadA(), loadB()]);
```

### TypeScript

#### Don't

```ts
const [a, b] = await Promise.all([await loadA(), loadB()]);
```

#### Do

```ts
const [a, b] = await Promise.all([loadA(), loadB()]);
```

### TSX

#### Don't

```tsx
const [a, b] = await Promise.all([await loadA(), loadB()]);
```

#### Do

```tsx
const [a, b] = await Promise.all([loadA(), loadB()]);
```
