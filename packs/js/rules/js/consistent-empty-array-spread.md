---
id: js/consistent-empty-array-spread
title: "Spread an array, not a string, in the empty branch"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-empty-array-spread]
since: 0.5.0
---

In an array literal, `...(cond ? array : '')` uses `[]` for the empty branch.

**Why:** Spreading `''` happens to add nothing, but mixes a string into code that builds an array; `[]` says what is meant.

```codelaw

match:
  kind: spread_element
  inside: { kind: array }
  has:
    kind: parenthesized_expression
    has:
      kind: ternary_expression
      any:
        - all: [{ has: { field: consequence, kind: array } }, { has: { field: alternative, pattern: "''", kind: string } }]
        - all: [{ has: { field: consequence, pattern: "''", kind: string } }, { has: { field: alternative, kind: array } }]
      has: { kind: string, pattern: $E }
message: "Use `[]`, not `''`, as the empty branch of an array spread"
fix:
  - replace: { target: $E, with: "[]" }
```

## Examples

### JavaScript

#### Don't

```js
const all = [...base, ...(extra ? [x] : '')];
```

#### Do

```js
const all = [...base, ...(extra ? [x] : [])];
```

### JSX

#### Don't

```jsx
const all = [...base, ...(extra ? [x] : '')];
```

#### Do

```jsx
const all = [...base, ...(extra ? [x] : [])];
```

### TypeScript

#### Don't

```ts
const all = [...base, ...(extra ? [x] : '')];
```

#### Do

```ts
const all = [...base, ...(extra ? [x] : [])];
```

### TSX

#### Don't

```tsx
const all = [...base, ...(extra ? [x] : '')];
```

#### Do

```tsx
const all = [...base, ...(extra ? [x] : [])];
```
