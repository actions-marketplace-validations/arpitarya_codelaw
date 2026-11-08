---
id: js/block-scoped-var
title: "No `var` used outside the block that declares it"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where the variable belongs, and whether it should become `let`, is the author's choice"
fix_exception: restructure
stability: preview
ported_from: [eslint/block-scoped-var]
since: 0.5.0
---

A `var` is hoisted to its function, but code that uses it outside the block it is written in — after an `if`, a loop or a `switch`, or in a sibling block — reads as if it were block-scoped. Each such reference is reported, once for each declaration whose block it is outside.

**Why:** Code that relies on hoisting out of a block reads as a mistake and breaks when the `var` becomes `let`.

```codelaw
native: js/block-scoped-var
```

## How to fix

1. Declare the variable before the block, in the scope where it is used, or use `let` inside the block only.

## Examples

### JavaScript

#### Don't

```js
function f(ok) {
  if (ok) {
    var label = "yes";
  }
  return label;
}
```

#### Do

```js
function f(ok) {
  let label;
  if (ok) {
    label = "yes";
  }
  return label;
}
```


### JSX

#### Don't

```jsx
function f(ok) {
  if (ok) {
    var label = "yes";
  }
  return label;
}
```

#### Do

```jsx
function f(ok) {
  let label;
  if (ok) {
    label = "yes";
  }
  return label;
}
```


### TypeScript

#### Don't

```ts
function f(ok) {
  if (ok) {
    var label = "yes";
  }
  return label;
}
```

#### Do

```ts
function f(ok) {
  let label;
  if (ok) {
    label = "yes";
  }
  return label;
}
```


### TSX

#### Don't

```tsx
function f(ok) {
  if (ok) {
    var label = "yes";
  }
  return label;
}
```

#### Do

```tsx
function f(ok) {
  let label;
  if (ok) {
    label = "yes";
  }
  return label;
}
```
