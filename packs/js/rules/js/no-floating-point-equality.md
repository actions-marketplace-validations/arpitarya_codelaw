---
id: js/no-floating-point-equality
title: "No === between a computed number and a decimal literal"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the tolerance is a judgement; `Number.EPSILON` suits values near 1, not every scale"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-floating-point-equality]
since: 0.5.0
---

Binary floating point can't hold most decimals exactly: `0.1 + 0.2 === 0.3` is false. An equality (`===`, `!==`, `==`, `!=`) between an arithmetic expression and a literal with a fractional part compares within a tolerance instead: `Math.abs(a - b) < Number.EPSILON`. Upstream also tracks values through variables; those are not reported here.

**Why:** The comparison fails for values that print the same.

```codelaw
match:
  kind: binary_expression
  any:
    - all:
        - any:
            - pattern: "$A === $B"
            - pattern: "$A == $B"
        - pattern: $EQ
    - all:
        - any:
            - pattern: "$A !== $B"
            - pattern: "$A != $B"
        - pattern: $NE
  all:
    - has: { field: left, kind: binary_expression, has: { field: operator, regex: '^[-+*/]$' } }
    - has: { field: right, kind: number, regex: '\.\d*[1-9]' }
message: "Compare floating-point values within a tolerance"
fix:
  - replace: { target: $EQ, with: "Math.abs($A - $B) < Number.EPSILON" }
  - replace: { target: $NE, with: "Math.abs($A - $B) >= Number.EPSILON" }
```

## Examples

### JavaScript

#### Don't

```js
if (a + b === 0.3) {}
```

#### Do

```js
if (Math.abs(a + b - 0.3) < Number.EPSILON) {}
```

#### Fixed

```js
if (Math.abs(a + b - 0.3) < Number.EPSILON) {}
```


### JSX

#### Don't

```jsx
if (a + b === 0.3) {}
```

#### Do

```jsx
if (Math.abs(a + b - 0.3) < Number.EPSILON) {}
```

#### Fixed

```jsx
if (Math.abs(a + b - 0.3) < Number.EPSILON) {}
```


### TypeScript

#### Don't

```ts
if (a + b === 0.3) {}
```

#### Do

```ts
if (Math.abs(a + b - 0.3) < Number.EPSILON) {}
```

#### Fixed

```ts
if (Math.abs(a + b - 0.3) < Number.EPSILON) {}
```


### TSX

#### Don't

```tsx
if (a + b === 0.3) {}
```

#### Do

```tsx
if (Math.abs(a + b - 0.3) < Number.EPSILON) {}
```

#### Fixed

```tsx
if (Math.abs(a + b - 0.3) < Number.EPSILON) {}
```
