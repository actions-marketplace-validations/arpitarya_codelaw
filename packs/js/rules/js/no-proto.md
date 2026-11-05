---
id: js/no-proto
title: "No __proto__"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Object.getPrototypeOf` and `Object.setPrototypeOf` differ from `__proto__` for null-prototype objects and own `__proto__` properties, which the reviewer confirms"
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
      nthChild: 1
      has: { field: property, regex: '^__proto__$' }
      all:
        - has: { field: object, pattern: $O1 }
        - inside:
            kind: assignment_expression
            pattern: $ASG
            has: { field: right, pattern: $P }
    - kind: subscript_expression
      nthChild: 1
      has: { field: index, regex: '^["''`]__proto__["''`]$' }
      all:
        - has: { field: object, pattern: $O1 }
        - inside:
            kind: assignment_expression
            pattern: $ASG
            has: { field: right, pattern: $P }
    - kind: member_expression
      has: { field: property, regex: '^__proto__$' }
      all:
        - pattern: $M2
        - has: { field: object, pattern: $O2 }
    - kind: subscript_expression
      has: { field: index, regex: '^["''`]__proto__["''`]$' }
      all:
        - pattern: $M2
        - has: { field: object, pattern: $O2 }
message: "Use `Object.getPrototypeOf` or `Object.setPrototypeOf` instead of `__proto__`"
fix:
  - replace: { target: $ASG, with: "Object.setPrototypeOf($O1, $P)" }
  - replace: { target: $M2, with: "Object.getPrototypeOf($O2)" }
```

## How to fix

1. Use `Object.getPrototypeOf(obj)` to read and `Object.setPrototypeOf(obj, proto)` to write.

## Examples

### JavaScript

#### Don't

```js
const p = obj.__proto__;
```

#### Fixed

```js
const p = Object.getPrototypeOf(obj);
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

#### Fixed

```jsx
const p = Object.getPrototypeOf(obj);
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

#### Fixed

```ts
const p = Object.getPrototypeOf(obj);
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

#### Fixed

```tsx
const p = Object.getPrototypeOf(obj);
```

#### Do

```tsx
const p = Object.getPrototypeOf(obj);
```
