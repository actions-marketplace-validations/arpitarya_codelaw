---
id: js/comma-or-logical-or-case
title: "No comma or || in a case"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "splitting the case is a structural edit"
stability: preview
ported_from: [eslint-plugin-sonarjs/comma-or-logical-or-case]
since: 0.5.0
---

A `case` value is not a comma or `||`/`&&` expression.

**Why:** The expression is evaluated to one value, so `case 1, 2:` matches only 2 and `case "a" || "b":` only "a".

```codelaw

match:
  kind: switch_case
  has: { field: value, any: [{kind: sequence_expression}, {kind: binary_expression, has: { field: operator, regex: '^(\|\||&&)$' }}] }
message: "`case a || b` tests one value; write one `case` per value"
```

## How to fix

1. Write one `case` label for each value, falling through to the same body.

## Examples

### JavaScript

#### Don't

```js
switch (k) {
  case "a" || "b":
    run();
}
```

#### Do

```js
switch (k) {
  case "a":
  case "b":
    run();
}
```

### JSX

#### Don't

```jsx
switch (k) {
  case "a" || "b":
    run();
}
```

#### Do

```jsx
switch (k) {
  case "a":
  case "b":
    run();
}
```

### TypeScript

#### Don't

```ts
switch (k) {
  case "a" || "b":
    run();
}
```

#### Do

```ts
switch (k) {
  case "a":
  case "b":
    run();
}
```

### TSX

#### Don't

```tsx
switch (k) {
  case "a" || "b":
    run();
}
```

#### Do

```tsx
switch (k) {
  case "a":
  case "b":
    run();
}
```
