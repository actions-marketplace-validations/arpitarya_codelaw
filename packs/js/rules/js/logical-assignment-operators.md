---
id: js/logical-assignment-operators
title: "a ||= b rather than a = a || b"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the assignment no longer runs when the value is already set, so a setter or a proxy on the target is not called"
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
  all: [{ has: { field: right, has: { field: operator, pattern: $$OP } } }]
message: "Use a logical assignment (`||=`, `&&=` or `??=`)"
fix: "$A $OP= $B"
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
