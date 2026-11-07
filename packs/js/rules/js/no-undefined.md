---
id: js/no-undefined
title: "No undefined identifier"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`void 0` stands wherever `undefined` does, but leaving the value out may read better; the reviewer picks"
stability: preview
ported_from: [eslint/no-undefined]
since: 0.5.0
---

The identifier `undefined` is not used (taste preset).

**Why:** `undefined` can be shadowed by a local variable; leaving a value out, or `void 0`, cannot.

```codelaw

match:
  kind: undefined
  # TypeScript's `undefined` type (`T | undefined`) is not the identifier,
  # and `void 0` is not a type (W-139 corpus: a fix that was not a fixpoint)
  not: { inside: { kind: literal_type, stopBy: neighbor } }
message: "Use `void 0`, or leave the value out, rather than `undefined`"
fix: "void 0"
```

## How to fix

1. Leave the value out where the position allows it, as the message says; otherwise `void 0` is what the fix writes.

## Examples

### JavaScript

#### Don't

```js
let value = undefined;
```

#### Fixed

```js
let value = void 0;
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

#### Fixed

```jsx
let value = void 0;
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

#### Fixed

```ts
let value = void 0;
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

#### Fixed

```tsx
let value = void 0;
```

#### Do

```tsx
let value;
```
