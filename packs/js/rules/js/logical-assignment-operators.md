---
id: js/logical-assignment-operators
title: "a ||= b rather than a = a || b"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the operator of the rewrite depends on which of three patterns matched"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint/logical-assignment-operators, eslint-plugin-unicorn/logical-assignment-operators]
since: 0.5.0
---

An assignment of the form `a = a || b` (or `&&`, `??`) uses the logical assignment operator.

**Why:** The short form names `a` once and assigns only when needed.

```codelaw

match:
  any:
    - pattern: $A = $A || $B
    - pattern: $A = $A && $B
    - pattern: $A = $A ?? $B
  has: { field: left, any: [{kind: identifier}, {kind: member_expression}] }
message: "Use a logical assignment (`||=`, `&&=` or `??=`)"
```

## How to fix

1. Write `a ||= b`, `a &&= b` or `a ??= b`.

## Examples

### JavaScript

#### Don't

```js
opts = opts || {};
```

#### Do

```js
opts ||= {};
```

### JSX

#### Don't

```jsx
opts = opts || {};
```

#### Do

```jsx
opts ||= {};
```

### TypeScript

#### Don't

```ts
opts = opts || {};
```

#### Do

```ts
opts ||= {};
```

### TSX

#### Don't

```tsx
opts = opts || {};
```

#### Do

```tsx
opts ||= {};
```
