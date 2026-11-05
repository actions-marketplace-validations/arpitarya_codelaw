---
id: js/promise-spec-only
title: "Only standard Promise methods"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "library methods differ in concurrency and callback arguments from the standard ones, so the reviewer confirms the rewrite; a method with no standard equivalent has no edit"
stability: preview
ported_from: [eslint-plugin-promise/spec-only]
since: 0.5.0
---

Only the static methods the language defines are called on `Promise`.

**Why:** A method a promise library adds to the global (`Promise.map`, `Promise.delay`) breaks when that library is not loaded first, or is swapped out.

```codelaw
match:
  pattern: Promise.$M
  any:
    - inside: { all: [{ pattern: $CMAP }, { pattern: 'Promise.map($XS, $FN)' }] }
    - inside: { all: [{ pattern: $CDELAY }, { pattern: Promise.delay($MS) }] }
    - kind: member_expression
constraints:
  M: { not: { regex: '^(resolve|reject|all|allSettled|race|any|withResolvers|try|prototype)$' } }
message: "`Promise.$M` is not a standard method"
fix:
  - replace: { target: $CMAP, with: "Promise.all($XS.map($FN))" }
  - replace: { target: $CDELAY, with: "new Promise((resolve) => setTimeout(resolve, $MS))" }
```

## How to fix

1. Replace the library method with the standard ones (`Promise.all(items.map(f))` for `Promise.map`, a `setTimeout` promise for `Promise.delay`) or import the library explicitly; the fix writes those two.

## Examples

### JavaScript

#### Don't

```js
const out = Promise.map(items, load);
```

#### Do

```js
const out = Promise.all(items.map(load));
```

### JSX

#### Don't

```jsx
const out = Promise.map(items, load);
```

#### Do

```jsx
const out = Promise.all(items.map(load));
```

### TypeScript

#### Don't

```ts
const out = Promise.map(items, load);
```

#### Do

```ts
const out = Promise.all(items.map(load));
```

### TSX

#### Don't

```tsx
const out = Promise.map(items, load);
```

#### Do

```tsx
const out = Promise.all(items.map(load));
```
