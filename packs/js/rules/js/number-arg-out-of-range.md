---
id: js/number-arg-out-of-range
title: "Number method argument out of range"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the radix or digit count that was meant is not in the code"
stability: preview
ported_from: [oxlint/number-arg-out-of-range]
since: 0.5.0
---

A literal passed to `toString()` is a radix from 2 to 36; to `toFixed()` and `toExponential()`, 0 to 100 digits; to `toPrecision()`, 1 to 100.

**Why:** An argument outside the range throws a RangeError when the line runs.

```codelaw
match:
  any:
    - pattern: $X.toString($R)
    - pattern: $X.toFixed($D)
    - pattern: $X.toExponential($D)
    - pattern: $X.toPrecision($P)
constraints:
  R: { regex: '^(-.*|[01]|3[7-9]|[4-9][0-9]|[0-9]{3,})$', any: [{kind: number}, {kind: unary_expression}] }
  D: { regex: '^(-.*|10[1-9]|1[1-9][0-9]|[2-9][0-9]{2}|[0-9]{4,})$', any: [{kind: number}, {kind: unary_expression}] }
  P: { regex: '^(-.*|0|10[1-9]|1[1-9][0-9]|[2-9][0-9]{2}|[0-9]{4,})$', any: [{kind: number}, {kind: unary_expression}] }
message: "This argument is out of range and throws a RangeError"
```

## How to fix

1. Pass the radix (2–36) or number of digits (0–100; 1–100 for `toPrecision`) you meant.

## Examples

### JavaScript

#### Don't

```js
const hex = n.toString(160);
```

#### Do

```js
const hex = n.toString(16);
```

### JSX

#### Don't

```jsx
const hex = n.toString(160);
```

#### Do

```jsx
const hex = n.toString(16);
```

### TypeScript

#### Don't

```ts
const hex = n.toString(160);
```

#### Do

```ts
const hex = n.toString(16);
```

### TSX

#### Don't

```tsx
const hex = n.toString(160);
```

#### Do

```tsx
const hex = n.toString(16);
```
