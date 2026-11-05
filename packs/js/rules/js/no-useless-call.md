---
id: js/no-useless-call
title: "No needless call or apply"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the plain call is equivalent only when `this` is as written; the author confirms"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint/no-useless-call]
since: 0.5.0
---

`.call()` and `.apply()` are not used where a plain call does the same.

**Why:** `f.call(undefined, a)` is `f(a)` and `obj.m.call(obj, a)` is `obj.m(a)`; the long form hides that.

```codelaw
match:
  any:
    - pattern: $F.call(undefined, $$$A)
    - pattern: $F.call(null, $$$A)
    - pattern: $O.$M.call($O, $$$A)
message: "This `.call()` is not needed; call the function directly"
```

## How to fix

1. Call the function directly.

## Examples

### JavaScript

#### Don't

```js
greet.call(undefined, 'Ada');
```

#### Do

```js
greet('Ada');
```

### JSX

#### Don't

```jsx
greet.call(undefined, 'Ada');
```

#### Do

```jsx
greet('Ada');
```

### TypeScript

#### Don't

```ts
greet.call(undefined, 'Ada');
```

#### Do

```ts
greet('Ada');
```

### TSX

#### Don't

```tsx
greet.call(undefined, 'Ada');
```

#### Do

```tsx
greet('Ada');
```
