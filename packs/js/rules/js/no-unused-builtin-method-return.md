---
id: js/no-unused-builtin-method-return
title: "Use the result of a method that returns a new value"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where the result should go is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-unused-builtin-method-return]
since: 0.5.0
---

The result of a method that returns a new value without changing its receiver (`map`, `filter`, `slice`, `trim`, `toUpperCase`, …) is not discarded.

**Why:** The call has no effect; the author expected it to change the value in place.

```codelaw

match:
  kind: expression_statement
  has: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(map|filter|slice|concat|flat|flatMap|toSorted|toReversed|toSpliced|with|trim|trimStart|trimEnd|toUpperCase|toLowerCase|padStart|padEnd|replace|replaceAll|substring|repeat|normalize|at|includes|indexOf|some|every|find|findIndex)$' } } }
message: "This method changes nothing and its result is thrown away"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
name.trim();
```

#### Do

```js
name = name.trim();
```

### JSX

#### Don't

```jsx
name.trim();
```

#### Do

```jsx
name = name.trim();
```

### TypeScript

#### Don't

```ts
name.trim();
```

#### Do

```ts
name = name.trim();
```

### TSX

#### Don't

```tsx
name.trim();
```

#### Do

```tsx
name = name.trim();
```
