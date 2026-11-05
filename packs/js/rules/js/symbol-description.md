---
id: js/symbol-description
title: "A Symbol has a description"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's"
fix_pending: "suggest"
stability: preview
ported_from: [eslint/symbol-description]
since: 0.5.0
---

`Symbol()` is given a description.

**Why:** The description is what a debugger and `String(symbol)` show; without it every symbol looks the same.

```codelaw
match:
  pattern: Symbol()
message: "Give the symbol a description"
```

## How to fix

1. Pass a short description: `Symbol('cache key')`.

## Examples

### JavaScript

#### Don't

```js
const key = Symbol();
```

#### Do

```js
const key = Symbol('key');
```

### JSX

#### Don't

```jsx
const key = Symbol();
```

#### Do

```jsx
const key = Symbol('key');
```

### TypeScript

#### Don't

```ts
const key = Symbol();
```

#### Do

```ts
const key = Symbol('key');
```

### TSX

#### Don't

```tsx
const key = Symbol();
```

#### Do

```tsx
const key = Symbol('key');
```
