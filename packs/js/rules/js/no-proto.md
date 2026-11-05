---
id: js/no-proto
title: "No __proto__"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the replacement depends on whether it was read or written"
fix_pending: "suggest"
stability: preview
ported_from: [eslint/no-proto]
since: 0.5.0
---

`__proto__` is not read or written.

**Why:** It is a legacy accessor; `Object.getPrototypeOf` and `Object.setPrototypeOf` are the standard.

```codelaw
match:
  any:
    - kind: member_expression
      has: { field: property, regex: '^__proto__$' }
    - kind: subscript_expression
      has: { field: index, regex: '^["''`]__proto__["''`]$' }
message: "Use `Object.getPrototypeOf` or `Object.setPrototypeOf` instead of `__proto__`"
```

## How to fix

1. Use `Object.getPrototypeOf(obj)` to read and `Object.setPrototypeOf(obj, proto)` to write.

## Examples

### JavaScript

#### Don't

```js
const p = obj.__proto__;
```

#### Do

```js
const p = Object.getPrototypeOf(obj);
```

### JSX

#### Don't

```jsx
const p = obj.__proto__;
```

#### Do

```jsx
const p = Object.getPrototypeOf(obj);
```

### TypeScript

#### Don't

```ts
const p = obj.__proto__;
```

#### Do

```ts
const p = Object.getPrototypeOf(obj);
```

### TSX

#### Don't

```tsx
const p = obj.__proto__;
```

#### Do

```tsx
const p = Object.getPrototypeOf(obj);
```
