---
id: js/no-equals-in-for-termination
title: "No == or != as a for loop end test"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which relational operator is right depends on the direction"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-equals-in-for-termination]
since: 0.5.0
---

A `for` loop that steps its counter by more than one does not end on `!=` or `==`. `i++` and `i--` are left alone.

**Why:** A step can jump over the exact value, so the loop never ends.

```codelaw
match:
  kind: for_statement
  has:
    field: condition
    kind: binary_expression
    has:
      field: operator
      regex: ^(!=|!==|==|===)$
    all:
    - has:
        field: left
        kind: identifier
        pattern: $I
  all:
  - has:
      field: increment
      any:
      - pattern: $I += $X
      - pattern: $I -= $X
message: An equality test as the end condition can be stepped over; use `<` or `>`
```

## How to fix

1. Use `<`, `<=`, `>` or `>=` in the direction the counter moves.

## Examples

### JavaScript

#### Don't

```js
for (let i = 0; i != 10; i += 3) {
  use(i);
}
```

#### Do

```js
for (let i = 0; i < 10; i += 3) {
  use(i);
}
```

### JSX

#### Don't

```jsx
for (let i = 0; i != 10; i += 3) {
  use(i);
}
```

#### Do

```jsx
for (let i = 0; i < 10; i += 3) {
  use(i);
}
```

### TypeScript

#### Don't

```ts
for (let i = 0; i != 10; i += 3) {
  use(i);
}
```

#### Do

```ts
for (let i = 0; i < 10; i += 3) {
  use(i);
}
```

### TSX

#### Don't

```tsx
for (let i = 0; i != 10; i += 3) {
  use(i);
}
```

#### Do

```tsx
for (let i = 0; i < 10; i += 3) {
  use(i);
}
```
