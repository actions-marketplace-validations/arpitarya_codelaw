---
id: js/no-unthrown-error
title: "Throw the error you create"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "throwing changes control flow from here on"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-unthrown-error]
since: 0.5.0
---

A `new …Error(…)` expression is not a statement on its own.

**Why:** Creating an error does nothing; the `throw` was forgotten.

```codelaw

match:
  kind: expression_statement
  has: { kind: new_expression, has: { field: constructor, regex: '^([A-Z][A-Za-z]*)?Error$' }, pattern: $E }
message: "This error is created and dropped; `throw` it"
fix: "throw $E;"
```

## Examples

### JavaScript

#### Don't

```js
if (!ok) {
  new Error("failed");
}
```

#### Do

```js
if (!ok) {
  throw new Error("failed");
}
```

### JSX

#### Don't

```jsx
if (!ok) {
  new Error("failed");
}
```

#### Do

```jsx
if (!ok) {
  throw new Error("failed");
}
```

### TypeScript

#### Don't

```ts
if (!ok) {
  new Error("failed");
}
```

#### Do

```ts
if (!ok) {
  throw new Error("failed");
}
```

### TSX

#### Don't

```tsx
if (!ok) {
  new Error("failed");
}
```

#### Do

```tsx
if (!ok) {
  throw new Error("failed");
}
```
