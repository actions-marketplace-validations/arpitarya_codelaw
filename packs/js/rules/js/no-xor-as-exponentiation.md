---
id: js/no-xor-as-exponentiation
title: "No ^ where a power was meant"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`^` is bitwise XOR; whether a power was meant is the author's to confirm"
stability: preview
ported_from: [eslint-plugin-unicorn/no-xor-as-exponentiation]
since: 0.5.0
---

`2 ^ 8` and `10 ^ 3` are not written to mean a power: `^` is bitwise XOR in JavaScript (`2 ^ 8` is 10), and a power is `2 ** 8`.

**Why:** The XOR of two small constants is almost never wanted; the power almost always is.

```codelaw
match:
  pattern: "$A ^ $B"
constraints:
  A: { kind: number, regex: '^(2|10)$' }
  B: { kind: number }
message: "`^` is XOR; write a power as `$A ** $B`"
fix: "$A ** $B"
```

## Examples

### JavaScript

#### Don't

```js
const max = 2 ^ 16;
```

#### Do

```js
const max = 2 ** 16;
```


### JSX

#### Don't

```jsx
const max = 2 ^ 16;
```

#### Do

```jsx
const max = 2 ** 16;
```


### TypeScript

#### Don't

```ts
const max = 2 ^ 16;
```

#### Do

```ts
const max = 2 ** 16;
```


### TSX

#### Don't

```tsx
const max = 2 ^ 16;
```

#### Do

```tsx
const max = 2 ** 16;
```
