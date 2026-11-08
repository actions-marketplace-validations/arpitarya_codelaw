---
id: js/no-invariant-returns
title: "No function that returns the same value on every path"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the function should return nothing or return what differs is the author's design"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-invariant-returns]
since: 0.5.0
---

A function with two or more `return`s, all with a value, that cannot fall off its end and has no `throw`, is reported at its head when every return gives the same literal (also through a local `const`), or the same local variable that is never reassigned and never passed on or changed through a property. The guard-and-log shape — a branch that does something and returns the same value as the rest — is left alone, as upstream does. Unlike upstream, the end of the function is judged by its syntax, not a code-path graph.

**Why:** A return value that never varies carries no information, and callers that test it are dead code.

```codelaw
native: js/no-invariant-returns
```

## How to fix

1. Return nothing if the value means nothing, or return the values that actually differ.

## Examples

### JavaScript

#### Don't

```js
function check(x) {
  if (x) {
    return 0;
  }
  return 0;
}
```

#### Do

```js
function check(x) {
  if (x) {
    return 1;
  }
  return 0;
}
```


### JSX

#### Don't

```jsx
function check(x) {
  if (x) {
    return 0;
  }
  return 0;
}
```

#### Do

```jsx
function check(x) {
  if (x) {
    return 1;
  }
  return 0;
}
```


### TypeScript

#### Don't

```ts
function check(x) {
  if (x) {
    return 0;
  }
  return 0;
}
```

#### Do

```ts
function check(x) {
  if (x) {
    return 1;
  }
  return 0;
}
```


### TSX

#### Don't

```tsx
function check(x) {
  if (x) {
    return 0;
  }
  return 0;
}
```

#### Do

```tsx
function check(x) {
  if (x) {
    return 1;
  }
  return 0;
}
```
