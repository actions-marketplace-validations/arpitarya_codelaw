---
id: js/no-void
title: "No void operator"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the replacement — `undefined`, or handling the promise — depends on the use"
stability: preview
ported_from: [eslint/no-void]
since: 0.5.0
---

The `void` operator is not used.

**Why:** `void 0` is `undefined` written obscurely; `void promise` hides an unhandled promise.

```codelaw
match:
  kind: unary_expression
  regex: '^void\b'
message: "Do not use `void`"
```

## How to fix

1. Write `undefined`, or handle the promise with `await` or `.catch()`.

## Examples

### JavaScript

#### Don't

```js
const nothing = void 0;
```

#### Do

```js
const nothing = undefined;
```

### JSX

#### Don't

```jsx
const nothing = void 0;
```

#### Do

```jsx
const nothing = undefined;
```

### TypeScript

#### Don't

```ts
const nothing = void 0;
```

#### Do

```ts
const nothing = undefined;
```

### TSX

#### Don't

```tsx
const nothing = void 0;
```

#### Do

```tsx
const nothing = undefined;
```
