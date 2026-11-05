---
id: js/no-return-array-push
title: "No return of push()"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the array or the length was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-unicorn/no-return-array-push]
since: 0.5.0
---

A function does not return the result of `push` or `unshift`.

**Why:** Those methods return the array's new length, which callers rarely expect.

```codelaw

match:
  kind: return_statement
  has: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(push|unshift)$' } } }
message: "`push` returns the new length, not the array"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
function add(list, x) {
  return list.push(x);
}
```

#### Do

```js
function add(list, x) {
  list.push(x);
  return list;
}
```

### JSX

#### Don't

```jsx
function add(list, x) {
  return list.push(x);
}
```

#### Do

```jsx
function add(list, x) {
  list.push(x);
  return list;
}
```

### TypeScript

#### Don't

```ts
function add(list, x) {
  return list.push(x);
}
```

#### Do

```ts
function add(list, x) {
  list.push(x);
  return list;
}
```

### TSX

#### Don't

```tsx
function add(list, x) {
  return list.push(x);
}
```

#### Do

```tsx
function add(list, x) {
  list.push(x);
  return list;
}
```
