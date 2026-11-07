---
id: js/no-anonymous-default-export
title: "Name the default export"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the name the function or class should have is the author's; the file name is only one guess at it"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-anonymous-default-export]
since: 0.5.0
---

A default-exported function or class has a name.

**Why:** A name shows in stack traces and lets editors import it by name; unicorn derives it from the file name.

```codelaw

match:
  kind: export_statement
  regex: '^export\s+default\b'
  has:
    field: value
    any:
      - kind: arrow_function
      - { kind: function_expression, not: { has: { field: name, regex: "." } } }
      - { kind: class, not: { has: { field: name, regex: "." } } }
message: "Give the default-exported function or class a name"
```

## How to fix

1. Name the function or class after the file (`user-card.js` → `UserCard`).

## Examples

### JavaScript

#### Don't

```js
export default function () {
  return 1;
}
```

#### Do

```js
export default function one() {
  return 1;
}
```

### JSX

#### Don't

```jsx
export default function () {
  return 1;
}
```

#### Do

```jsx
export default function one() {
  return 1;
}
```

### TypeScript

#### Don't

```ts
export default function () {
  return 1;
}
```

#### Do

```ts
export default function one() {
  return 1;
}
```

### TSX

#### Don't

```tsx
export default function () {
  return 1;
}
```

#### Do

```tsx
export default function one() {
  return 1;
}
```
