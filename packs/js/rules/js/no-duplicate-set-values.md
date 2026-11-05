---
id: js/no-duplicate-set-values
title: "No repeated literal in new Set([...])"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the repeat is a typo for another value is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-duplicate-set-values]
since: 0.5.0
---

The array a `Set` is created from does not repeat a string or number literal.

**Why:** A set keeps one copy, so the repeat is a typo or a value that was meant to be different.

```codelaw

match:
  kind: array
  inside: { kind: arguments, inside: { kind: new_expression, has: { field: constructor, regex: '^Set$' } } }
  has: { any: [{kind: string}, {kind: number}], pattern: $V, precedes: { stopBy: end, pattern: $V } }
message: "`$V` is listed twice in the set's initial values"
```

## How to fix

1. Remove the repeated value, or replace it with the one that was meant.

## Examples

### JavaScript

#### Don't

```js
const tags = new Set(["a", "b", "a"]);
```

#### Do

```js
const tags = new Set(["a", "b"]);
```

### JSX

#### Don't

```jsx
const tags = new Set(["a", "b", "a"]);
```

#### Do

```jsx
const tags = new Set(["a", "b"]);
```

### TypeScript

#### Don't

```ts
const tags = new Set(["a", "b", "a"]);
```

#### Do

```ts
const tags = new Set(["a", "b"]);
```

### TSX

#### Don't

```tsx
const tags = new Set(["a", "b", "a"]);
```

#### Do

```tsx
const tags = new Set(["a", "b"]);
```
