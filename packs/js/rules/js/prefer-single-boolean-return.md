---
id: js/prefer-single-boolean-return
title: "Return the condition, not true/false"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a non-boolean condition needs `Boolean(…)` or `!`, decided per pattern"
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-single-boolean-return]
since: 0.5.0
---

An `if … else` that only returns `true` in one branch and `false` in the other is a single `return`.

**Why:** The condition already is the boolean.

```codelaw

match:
  kind: if_statement
  any:
    - pattern: "if ($C) { return true; } else { return false; }"
    - pattern: "if ($C) { return false; } else { return true; }"
    - pattern: "if ($C) return true; else return false;"
    - pattern: "if ($C) return false; else return true;"
message: "Return the condition itself"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
function isAdult(age) {
  if (age >= 18) {
    return true;
  } else {
    return false;
  }
}
```

#### Do

```js
function isAdult(age) {
  return age >= 18;
}
```

### JSX

#### Don't

```jsx
function isAdult(age) {
  if (age >= 18) {
    return true;
  } else {
    return false;
  }
}
```

#### Do

```jsx
function isAdult(age) {
  return age >= 18;
}
```

### TypeScript

#### Don't

```ts
function isAdult(age) {
  if (age >= 18) {
    return true;
  } else {
    return false;
  }
}
```

#### Do

```ts
function isAdult(age) {
  return age >= 18;
}
```

### TSX

#### Don't

```tsx
function isAdult(age) {
  if (age >= 18) {
    return true;
  } else {
    return false;
  }
}
```

#### Do

```tsx
function isAdult(age) {
  return age >= 18;
}
```
