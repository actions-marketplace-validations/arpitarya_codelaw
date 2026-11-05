---
id: js/no-param-reassign
title: "Parameters are not reassigned"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new variable's name is the author's choice"
fix_exception: value
stability: preview
ported_from: [eslint/no-param-reassign]
since: 0.5.0
---

A function parameter is not assigned to (the upstream default does not check its properties).

**Why:** Reassigning a parameter hides the value the caller passed and, in sloppy mode, changes `arguments`.

```codelaw

match:
  any:
    - kind: assignment_expression
      has: { field: left, kind: identifier, pattern: $P }
    - kind: augmented_assignment_expression
      has: { field: left, kind: identifier, pattern: $P }
    - kind: update_expression
      has: { kind: identifier, pattern: $P }
where:
  $P: { binding: [param] }
message: "`$P` is a parameter; assign to a new variable instead"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
function greet(name) {
  name = name.trim();
  return name;
}
```

#### Do

```js
function greet(name) {
  const clean = name.trim();
  return clean;
}
```

### JSX

#### Don't

```jsx
function greet(name) {
  name = name.trim();
  return name;
}
```

#### Do

```jsx
function greet(name) {
  const clean = name.trim();
  return clean;
}
```

### TypeScript

#### Don't

```ts
function greet(name) {
  name = name.trim();
  return name;
}
```

#### Do

```ts
function greet(name) {
  const clean = name.trim();
  return clean;
}
```

### TSX

#### Don't

```tsx
function greet(name) {
  name = name.trim();
  return name;
}
```

#### Do

```tsx
function greet(name) {
  const clean = name.trim();
  return clean;
}
```
