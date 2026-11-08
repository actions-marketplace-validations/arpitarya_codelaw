---
id: js/no-useless-recursion
title: "No function that only returns a call to itself"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the loop that replaces the recursion is a rewrite of the function's control flow"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-recursion]
since: 0.5.0
---

A named function — a declaration or a named function expression, async ones included — that returns a direct call to itself (`return walk(node.next)`) is reported at the call. Only the plain tail call counts: `return n + f(n - 1)`, `return c ? f(x) : x`, an optional call `f?.()`, a method, an arrow, a generator, and a return inside a `try` are left alone.

**Why:** Each tail call adds a stack frame, so deep input overflows the stack; the same logic as a loop does not.

```codelaw
native: js/no-useless-recursion
```

## How to fix

1. Turn the function into a loop: reassign the parameters the call passes and go round again; the base cases become the loop's exits.

## Examples

### JavaScript

#### Don't

```js
function last(node) {
  if (node.next) {
    return last(node.next);
  }
  return node;
}
```

#### Do

```js
function last(node) {
  while (node.next) {
    node = node.next;
  }
  return node;
}
```


### JSX

#### Don't

```jsx
function last(node) {
  if (node.next) {
    return last(node.next);
  }
  return node;
}
```

#### Do

```jsx
function last(node) {
  while (node.next) {
    node = node.next;
  }
  return node;
}
```


### TypeScript

#### Don't

```ts
function last(node) {
  if (node.next) {
    return last(node.next);
  }
  return node;
}
```

#### Do

```ts
function last(node) {
  while (node.next) {
    node = node.next;
  }
  return node;
}
```


### TSX

#### Don't

```tsx
function last(node) {
  if (node.next) {
    return last(node.next);
  }
  return node;
}
```

#### Do

```tsx
function last(node) {
  while (node.next) {
    node = node.next;
  }
  return node;
}
```
