---
id: js/no-implicit-coercion
title: "Explicit conversions, not !!x or +x"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Number(x)` and `+x` differ for a BigInt (`+1n` throws, `Number(1n)` is `1`), so the author confirms what the value holds"
stability: preview
ported_from: [eslint/no-implicit-coercion]
since: 0.5.0
---

`!!x` is written `Boolean(x)` and `+x` is written `Number(x)` (the upstream defaults; `"" + x` is left alone).

**Why:** The explicit call says which conversion is meant; `+x` in particular is easy to misread.

```codelaw

match:
  any:
    - pattern: "!!$X"
    - kind: unary_expression
      regex: '^\+'
      has: { field: argument, not: { any: [{kind: number}, {kind: unary_expression}] }, pattern: $X }
  all: [{ has: { field: operator, pattern: $$OP } }]
message: "Convert explicitly with `Boolean(…)` or `Number(…)`"
fix: "$OP|map(!:Boolean, +:Number)($X)"
```

## How to fix

1. Write `Boolean(x)` for `!!x` and `Number(x)` for `+x`.

## Examples

### JavaScript

#### Don't

```js
const has = !!items.length;
```

#### Do

```js
const has = Boolean(items.length);
```

### JSX

#### Don't

```jsx
const has = !!items.length;
```

#### Do

```jsx
const has = Boolean(items.length);
```

### TypeScript

#### Don't

```ts
const has = !!items.length;
```

#### Do

```ts
const has = Boolean(items.length);
```

### TSX

#### Don't

```tsx
const has = !!items.length;
```

#### Do

```tsx
const has = Boolean(items.length);
```
