---
id: js/no-ignored-exceptions
title: "No empty catch"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what to do with the error is the author's call"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/no-ignored-exceptions]
since: 0.5.0
---

A `catch` block is not empty (a comment inside counts as handling).

**Why:** An empty `catch` hides failures that someone will later spend hours looking for.

```codelaw

match:
  kind: catch_clause
  has: { field: body, regex: '^\{\s*\}$' }
message: "This error is swallowed; handle it, or say in a comment why it is safe to ignore"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
try {
  save();
} catch (error) {}
```

#### Do

```js
try {
  save();
} catch (error) {
  log(error);
}
```

### JSX

#### Don't

```jsx
try {
  save();
} catch (error) {}
```

#### Do

```jsx
try {
  save();
} catch (error) {
  log(error);
}
```

### TypeScript

#### Don't

```ts
try {
  save();
} catch (error) {}
```

#### Do

```ts
try {
  save();
} catch (error) {
  log(error);
}
```

### TSX

#### Don't

```tsx
try {
  save();
} catch (error) {}
```

#### Do

```tsx
try {
  save();
} catch (error) {
  log(error);
}
```
