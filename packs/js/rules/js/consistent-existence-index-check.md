---
id: js/consistent-existence-index-check
title: "Check indexOf against -1 the same way"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the rewrite depends on the operator (`< 0` becomes `=== -1`, the other two `!== -1`), and one rule carries one template"
fix_pending: "auto; needs computed-edit"
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-existence-index-check]
since: 0.5.0
---

An index from `indexOf`, `lastIndexOf`, `findIndex` or `findLastIndex` held in a variable is tested with `=== -1` or `!== -1`, not `< 0`, `>= 0` or `> -1`.

**Why:** These methods return -1 for "not found"; comparing with -1 says that, and reads the same everywhere.

```codelaw

match:
  kind: binary_expression
  any:
    - pattern: $I < 0
    - pattern: $I >= 0
    - pattern: $I > -1
  has: { field: left, kind: identifier, pattern: $I }
  inside:
    stopBy: end
    kind: statement_block
    has:
      stopBy: end
      kind: variable_declarator
      all:
        - has: { field: name, pattern: $I }
        - has: { field: value, kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(indexOf|lastIndexOf|findIndex|findLastIndex)$' } } }
message: "Compare the index with `=== -1` or `!== -1`"
```

## How to fix

1. Write `i === -1` for `i < 0`, and `i !== -1` for `i >= 0` or `i > -1`.

## Examples

### JavaScript

#### Don't

```js
function has(list, x) {
  const i = list.indexOf(x);
  return i >= 0;
}
```

#### Do

```js
function has(list, x) {
  const i = list.indexOf(x);
  return i !== -1;
}
```

### JSX

#### Don't

```jsx
function has(list, x) {
  const i = list.indexOf(x);
  return i >= 0;
}
```

#### Do

```jsx
function has(list, x) {
  const i = list.indexOf(x);
  return i !== -1;
}
```

### TypeScript

#### Don't

```ts
function has(list, x) {
  const i = list.indexOf(x);
  return i >= 0;
}
```

#### Do

```ts
function has(list, x) {
  const i = list.indexOf(x);
  return i !== -1;
}
```

### TSX

#### Don't

```tsx
function has(list, x) {
  const i = list.indexOf(x);
  return i >= 0;
}
```

#### Do

```tsx
function has(list, x) {
  const i = list.indexOf(x);
  return i !== -1;
}
```
