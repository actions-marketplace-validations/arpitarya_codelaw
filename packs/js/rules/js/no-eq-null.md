---
id: js/no-eq-null
title: "No == null"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a plain identifier compared with `null` through `==` or `!=`: the explicit two-way check is exact; other operands are reported as suggest"
stability: preview
ported_from: [eslint/no-eq-null]
since: 0.5.0
---

Comparisons with `null` use `===` or `!==`.

**Why:** `== null` also matches `undefined`, which is either a hidden intention or a bug.

```codelaw
match:
  any:
    - all: [{ pattern: "$A == null" }, { pattern: $M1 }]
    - all: [{ pattern: "$B != null" }, { pattern: $M2 }]
    - all: [{ pattern: "null == $C" }, { pattern: $M3 }]
    - all: [{ pattern: "null != $D" }, { pattern: $M4 }]
message: "Use `===` or `!==` to compare with `null`"
auto_when:
  any:
    - has: { field: left, kind: identifier }
    - has: { field: right, kind: identifier }
fix:
  - replace: { target: $M1, with: "($A === null || $A === undefined)" }
  - replace: { target: $M2, with: "($B !== null && $B !== undefined)" }
  - replace: { target: $M3, with: "($C === null || $C === undefined)" }
  - replace: { target: $M4, with: "($D !== null && $D !== undefined)" }
```

## How to fix

1. Write `=== null` (or `x === null || x === undefined` if both were meant).

## Examples

### JavaScript

#### Don't

```js
if (value == null) reset();
```

#### Fixed

```js
if ((value === null || value === undefined)) reset();
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

#### Fixed

```jsx
if ((value === null || value === undefined)) reset();
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

#### Fixed

```ts
if ((value === null || value === undefined)) reset();
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

#### Fixed

```tsx
if ((value === null || value === undefined)) reset();
```

#### Do

```tsx
if (value === null) reset();
```
