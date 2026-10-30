---
id: js/no-eq-null
title: "No == null"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether `undefined` should match too decides between `=== null` and an explicit two-way check"
stability: preview
ported_from: [eslint/no-eq-null]
since: 0.5.0
---

Comparisons with `null` use `===` or `!==`.

**Why:** `== null` also matches `undefined`, which is either a hidden intention or a bug.

```codelaw
match:
  any:
    - pattern: $A == null
    - pattern: $A != null
    - pattern: null == $A
    - pattern: null != $A
message: "Use `===` or `!==` to compare with `null`"
```

## How to fix

1. Write `=== null` (or `x === null || x === undefined` if both were meant).

## Examples

### JavaScript

#### Don't

```js
if (value == null) reset();
```

#### Do

```js
if (value === null) reset();
```

### JSX

#### Don't

```jsx
if (value == null) reset();
```

#### Do

```jsx
if (value === null) reset();
```

### TypeScript

#### Don't

```ts
if (value == null) reset();
```

#### Do

```ts
if (value === null) reset();
```

### TSX

#### Don't

```tsx
if (value == null) reset();
```

#### Do

```tsx
if (value === null) reset();
```
