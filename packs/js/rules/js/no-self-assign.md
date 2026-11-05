---
id: js/no-self-assign
title: "No assigning a variable to itself"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing the statement is exact for a local identifier but hides what was meant to be assigned, which the reviewer confirms; a property or other target carries no edit"
stability: preview
ported_from: [eslint/no-self-assign, eslint-plugin-sonarjs/no-redundant-assignments]
since: 0.1.0
---

An assignment's left and right sides are not the same expression.

**Why:** `a = a` has no effect; it is usually a typo for another name.

```codelaw
match:
  any:
    - pattern: $A = $A
      has: { field: left, kind: identifier }
      inside: { kind: expression_statement, stopBy: neighbor, pattern: $ST }
    - pattern: $A = $A
message: "Assigning `$A` to itself does nothing"
fix:
  - remove: $ST
```

## How to fix

1. If another value was meant, assign that.
2. Otherwise remove the assignment.

## Examples

### JavaScript

#### Don't

```js
let x = 1;
x = x;
```

#### Fixed

```js
let x = 1;
```

#### Do

```js
let x = 1;
x = y;
```

### JSX

#### Don't

```jsx
let x = 1;
x = x;
```

#### Fixed

```jsx
let x = 1;
```

#### Do

```jsx
let x = 1;
x = y;
```

### TypeScript

#### Don't

```ts
let x = 1;
x = x;
```

#### Fixed

```ts
let x = 1;
```

#### Do

```ts
let x = 1;
x = y;
```

### TSX

#### Don't

```tsx
let x = 1;
x = x;
```

#### Fixed

```tsx
let x = 1;
```

#### Do

```tsx
let x = 1;
x = y;
```
