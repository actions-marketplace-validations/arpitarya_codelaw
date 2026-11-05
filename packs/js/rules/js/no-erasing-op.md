---
id: js/no-erasing-op
title: "Multiplying by zero erases the value"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`0` is the result for finite numbers only: `NaN * 0` is `NaN`, a BigInt needs `0n`, and the other operand's side effects go"
stability: preview
ported_from: [oxlint/erasing-op]
since: 0.5.0
---

Code writes `0`, not an operation that always yields zero: `x * 0`, `0 * x`, `x & 0`, `0 & x` and `0 / x`.

**Why:** The other operand is computed and thrown away; usually the `0` is a typo or a leftover from debugging.

```codelaw
match:
  any:
    - pattern: $X * 0
    - pattern: 0 * $X
    - pattern: $X & 0
    - pattern: 0 & $X
    - pattern: 0 / $X
message: "`$$MATCH` is always zero; write `0`, or use the value you meant"
fix: "0"
```

## Examples

### JavaScript

#### Don't

```js
const offset = page * 0;
```

#### Do

```js
const offset = 0;
```

### JSX

#### Don't

```jsx
const offset = page * 0;
```

#### Do

```jsx
const offset = 0;
```

### TypeScript

#### Don't

```ts
const offset = page * 0;
```

#### Do

```ts
const offset = 0;
```

### TSX

#### Don't

```tsx
const offset = page * 0;
```

#### Do

```tsx
const offset = 0;
```
