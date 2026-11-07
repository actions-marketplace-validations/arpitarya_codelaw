---
id: js/prefer-ternary
title: "A ternary for an if-else that only returns or assigns"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-ternary]
since: 0.5.0
---

An `if`-`else` whose two branches each only return a value, or each only assign to the same target, is written as one `return` or one assignment with a ternary.

**Why:** The ternary says once what both branches do, and leaves only the difference.

```codelaw
utils:
  not-ternary-operand:
    not:
      any:
        - kind: ternary_expression
        - kind: assignment_expression
        - kind: augmented_assignment_expression
        - kind: sequence_expression
        - kind: yield_expression
        - kind: arrow_function
match:
  any:
    - pattern: "if ($C) { return $A; } else { return $B; }"
      all: [{ pattern: $RET }]
    - pattern: "if ($C) { $X = $A; } else { $X = $B; }"
      all: [{ pattern: $SET }]
constraints:
  C: { matches: not-ternary-operand }
  A: { matches: not-ternary-operand }
  B: { matches: not-ternary-operand }
message: "Write this if-else as a ternary"
fix:
  - replace: { target: $RET, with: "return $C ? $A : $B;" }
  - replace: { target: $SET, with: "$X = $C ? $A : $B;" }
```

## Examples

### JavaScript

#### Don't

```js
function sign(n) {
  if (n < 0) {
    return "negative";
  } else {
    return "positive";
  }
}
```

#### Do

```js
function sign(n) {
  return n < 0 ? "negative" : "positive";
}
```


### JSX

#### Don't

```jsx
function sign(n) {
  if (n < 0) {
    return "negative";
  } else {
    return "positive";
  }
}
```

#### Do

```jsx
function sign(n) {
  return n < 0 ? "negative" : "positive";
}
```


### TypeScript

#### Don't

```ts
function sign(n) {
  if (n < 0) {
    return "negative";
  } else {
    return "positive";
  }
}
```

#### Do

```ts
function sign(n) {
  return n < 0 ? "negative" : "positive";
}
```


### TSX

#### Don't

```tsx
function sign(n) {
  if (n < 0) {
    return "negative";
  } else {
    return "positive";
  }
}
```

#### Do

```tsx
function sign(n) {
  return n < 0 ? "negative" : "positive";
}
```
