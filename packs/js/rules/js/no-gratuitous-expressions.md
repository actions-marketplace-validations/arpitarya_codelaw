---
id: js/no-gratuitous-expressions
title: "No condition already decided by an enclosing `if`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the inner test or the outer one is wrong is the author's knowledge"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-gratuitous-expressions]
since: 0.5.0
---

`if (true)` and `if (false)` are reported. Inside the braced branch of `if (a && !b)`, the names the condition settles (`a` truthy, `b` falsy; for the `else` branch of `if (a)`, `a` falsy) are reported where they are tested again — in `&&`, the left of `||`, an `if` condition, or under `!` — unless the variable is written in the branch or from another function. A name on the right of `&&` directly inside JSX is left alone.

**Why:** A test whose answer is already known is dead logic, often a sign that a different variable was meant.

```codelaw
native: js/no-gratuitous-expressions
```

## How to fix

1. Remove the redundant test, or test the variable that was meant.

## Examples

### JavaScript

#### Don't

```js
function f(a, b) {
  if (a && b) {
    if (a) {
      go();
    }
  }
}
```

#### Do

```js
function f(a, b) {
  if (a && b) {
    go();
  }
}
```


### JSX

#### Don't

```jsx
function f(a, b) {
  if (a && b) {
    if (a) {
      go();
    }
  }
}
```

#### Do

```jsx
function f(a, b) {
  if (a && b) {
    go();
  }
}
```


### TypeScript

#### Don't

```ts
function f(a, b) {
  if (a && b) {
    if (a) {
      go();
    }
  }
}
```

#### Do

```ts
function f(a, b) {
  if (a && b) {
    go();
  }
}
```


### TSX

#### Don't

```tsx
function f(a, b) {
  if (a && b) {
    if (a) {
      go();
    }
  }
}
```

#### Do

```tsx
function f(a, b) {
  if (a && b) {
    go();
  }
}
```
