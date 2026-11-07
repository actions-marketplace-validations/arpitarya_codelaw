---
id: js/prefer-number-coercion
title: "Number() rather than parseFloat() or parseInt(value, 10)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`parseFloat` and `parseInt` read a numeric prefix (`'50px'` is 50) where `Number` gives `NaN`; where the prefix was the point the edit must be refused"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-number-coercion]
since: 0.5.0
---

Turning a value into a number is `Number(value)`, not `parseFloat(value)` (or `Number.parseFloat`); and an integer with radix 10 is `Math.trunc(Number(value))`, not `parseInt(value, 10)`. `parseInt` without a radix is left to `js/radix`.

**Why:** `Number` rejects input that isn't a number instead of quietly using its first digits.

```codelaw
match:
  kind: call_expression
  any:
    - all:
        - any:
            - pattern: "parseFloat($V)"
            - pattern: "Number.parseFloat($V)"
        - pattern: $W1
    - all:
        - any:
            - pattern: "parseInt($V, 10)"
            - pattern: "Number.parseInt($V, 10)"
        - pattern: $W2
constraints:
  V: { not: { kind: spread_element } }
message: "Coerce with `Number`"
fix:
  - replace: { target: $W1, with: "Number($V)" }
  - replace: { target: $W2, with: "Math.trunc(Number($V))" }
```

## Examples

### JavaScript

#### Don't

```js
const ratio = parseFloat(input);
```

#### Do

```js
const ratio = Number(input);
```


### JSX

#### Don't

```jsx
const ratio = parseFloat(input);
```

#### Do

```jsx
const ratio = Number(input);
```


### TypeScript

#### Don't

```ts
const ratio = parseFloat(input);
```

#### Do

```ts
const ratio = Number(input);
```


### TSX

#### Don't

```tsx
const ratio = parseFloat(input);
```

#### Do

```tsx
const ratio = Number(input);
```
