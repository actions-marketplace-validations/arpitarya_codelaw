---
id: js/no-undefined
title: "No undefined identifier"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to omit the value or write `void 0` depends on the position"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint/no-undefined]
since: 0.5.0
---

The identifier `undefined` is not used (taste preset).

**Why:** `undefined` can be shadowed by a local variable; leaving a value out, or `void 0`, cannot.

```codelaw

match:
  kind: undefined
message: "Use `void 0`, or leave the value out, rather than `undefined`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
let value = undefined;
```

#### Do

```js
let value;
```

### JSX

#### Don't

```jsx
let value = undefined;
```

#### Do

```jsx
let value;
```

### TypeScript

#### Don't

```ts
let value = undefined;
```

#### Do

```ts
let value;
```

### TSX

#### Don't

```tsx
let value = undefined;
```

#### Do

```tsx
let value;
```
