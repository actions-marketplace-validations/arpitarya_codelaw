---
id: js/no-useless-spread
title: "No spread of an array literal"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-spread]
since: 0.5.0
---

An array literal is not spread into another array literal. Upstream also covers spreading into calls, `new Set([...x])` and more; this port covers the literal case.

**Why:** The spread builds an array only to copy it.

```codelaw

match:
  kind: spread_element
  has: { kind: array, pattern: "[$$$A]" }
  inside: { kind: array }
  any:
    - { has: { kind: array, has: { nthChild: 1 } }, pattern: $SPREAD }
    - { regex: '' }
message: "Spreading an array literal into another array adds nothing; inline its items"
fix:
  - replace: { target: $SPREAD, with: "$$$A" }
```

## How to fix

1. Write the items in place: `[...[a, b], c]` → `[a, b, c]`.

## Examples

### JavaScript

#### Don't

```js
const all = [...[a, b], c];
```

#### Do

```js
const all = [a, b, c];
```

### JSX

#### Don't

```jsx
const all = [...[a, b], c];
```

#### Do

```jsx
const all = [a, b, c];
```

### TypeScript

#### Don't

```ts
const all = [...[a, b], c];
```

#### Do

```ts
const all = [a, b, c];
```

### TSX

#### Don't

```tsx
const all = [...[a, b], c];
```

#### Do

```tsx
const all = [a, b, c];
```
