---
id: js/no-boolean-sort-comparator
title: "A sort comparator returns a number"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the right comparator depends on the values compared (numbers, strings)"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-unicorn/no-boolean-sort-comparator]
since: 0.5.0
---

The comparator given to `sort` or `toSorted` does not return the result of a comparison operator.

**Why:** A boolean becomes 1 or 0, never negative, so the sort order is wrong in engines that rely on it.

```codelaw

match:
  kind: arrow_function
  has: { field: body, kind: binary_expression, has: { field: operator, regex: '^(<|>|<=|>=|===|!==)$' } }
  inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(sort|toSorted)$' } } } }
message: "A comparator must return a negative, zero or positive number, not a boolean"
```

## How to fix

1. Return `a - b` for numbers, `a.localeCompare(b)` for strings.

## Examples

### JavaScript

#### Don't

```js
items.sort((a, b) => a > b);
```

#### Do

```js
items.sort((a, b) => a - b);
```

### JSX

#### Don't

```jsx
items.sort((a, b) => a > b);
```

#### Do

```jsx
items.sort((a, b) => a - b);
```

### TypeScript

#### Don't

```ts
items.sort((a, b) => a > b);
```

#### Do

```ts
items.sort((a, b) => a - b);
```

### TSX

#### Don't

```tsx
items.sort((a, b) => a > b);
```

#### Do

```tsx
items.sort((a, b) => a - b);
```
