---
id: js/no-element-overwrite
title: "No element written twice in a row"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which of the two values was meant is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-element-overwrite]
since: 0.5.0
---

An array element or a `Map` entry is not written and then, in the very next statement, written again at the same index or key: the first write is lost.

**Why:** Two writes in a row to one place mean one of them is a slip, usually a wrong index or key.

```codelaw
match:
  any:
    - pattern: "$A[$I] = $X;"
      follows: { pattern: "$A[$I] = $Y;" }
      not: { has: { stopBy: end, kind: subscript_expression, not: { inside: { kind: assignment_expression, field: left } } } }
    - pattern: "$M.set($K, $X);"
      follows: { pattern: "$M.set($K, $Y);" }
message: "This overwrites the value written on the line before"
```

## How to fix

1. Fix the index or key of one of the two writes, or delete the one that is not wanted.

## Examples

### JavaScript

#### Don't

```js
totals[0] = sum(a);
totals[0] = sum(b);
```

#### Do

```js
totals[0] = sum(a);
totals[1] = sum(b);
```


### JSX

#### Don't

```jsx
totals[0] = sum(a);
totals[0] = sum(b);
```

#### Do

```jsx
totals[0] = sum(a);
totals[1] = sum(b);
```


### TypeScript

#### Don't

```ts
totals[0] = sum(a);
totals[0] = sum(b);
```

#### Do

```ts
totals[0] = sum(a);
totals[1] = sum(b);
```


### TSX

#### Don't

```tsx
totals[0] = sum(a);
totals[0] = sum(b);
```

#### Do

```tsx
totals[0] = sum(a);
totals[1] = sum(b);
```
