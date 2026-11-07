---
id: js/prefer-single-call
title: "One push() with several items"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the receiver is evaluated once instead of twice, and a value the first push's arguments read is now computed before the second's: a getter or a call with effects can tell"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-single-call]
since: 0.5.0
---

Consecutive `push` calls on the same array are merged. Upstream also covers `classList.add` and others; this port covers `push`.

**Why:** One call says the same in one statement.

```codelaw

match:
  kind: expression_statement
  pattern: $A.push($$$X);
  all:
    - has: { kind: call_expression, has: { field: arguments, pattern: $INTO } }
    - precedes: { all: [{ pattern: $A.push($$$Y); }, { pattern: $NEXT }, { has: { kind: call_expression, has: { field: arguments, pattern: $FROM } } }] }
message: "Push both values in one call: `$A.push(a, b)`"
fix:
  - merge-into: { target: $FROM, into: $INTO }
  - remove: $NEXT
```

## How to fix

1. Move the second call's arguments into the first call, after its own, and delete the second statement.

## Examples

### JavaScript

#### Don't

```js
list.push(a);
list.push(b);
```

#### Do

```js
list.push(a, b);
```

### JSX

#### Don't

```jsx
list.push(a);
list.push(b);
```

#### Do

```jsx
list.push(a, b);
```

### TypeScript

#### Don't

```ts
list.push(a);
list.push(b);
```

#### Do

```ts
list.push(a, b);
```

### TSX

#### Don't

```tsx
list.push(a);
list.push(b);
```

#### Do

```tsx
list.push(a, b);
```
