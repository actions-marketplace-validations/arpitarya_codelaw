---
id: js/prefer-bigint-literals
title: "BigInt literal rather than BigInt() of a constant"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-bigint-literals]
since: 0.5.0
---

A `BigInt` of an integer literal, or of a string of decimal digits, is written as a BigInt literal: `10n`, not `BigInt(10)` or `BigInt("10")`.

**Why:** The literal is shorter and is a constant the reader sees at once; the call does the same work at run time.

```codelaw
match:
  pattern: "$FN($A)"
constraints:
  FN: { regex: '^BigInt$' }
  A:
    any:
      - { kind: number, regex: '^(0|[1-9][0-9_]*|0[xX][0-9a-fA-F_]+|0[oO][0-7_]+|0[bB][01_]+)$' }
      - { kind: string, regex: '^["''](0|[1-9][0-9]*)["'']$' }
where:
  $FN: builtin
message: "Write a BigInt literal"
fix:
  - edit-text: { pattern: '^BigInt\(\s*["'']?([0-9a-fA-FxXoObB_]+)["'']?\s*\)$', replace: '${1}n' }
```

## Examples

### JavaScript

#### Don't

```js
const big = BigInt(9007199254740993);
const id = BigInt("42");
```

#### Do

```js
const big = 9007199254740993n;
const id = 42n;
```


### JSX

#### Don't

```jsx
const big = BigInt(9007199254740993);
const id = BigInt("42");
```

#### Do

```jsx
const big = 9007199254740993n;
const id = 42n;
```


### TypeScript

#### Don't

```ts
const big = BigInt(9007199254740993);
const id = BigInt("42");
```

#### Do

```ts
const big = 9007199254740993n;
const id = 42n;
```


### TSX

#### Don't

```tsx
const big = BigInt(9007199254740993);
const id = BigInt("42");
```

#### Do

```tsx
const big = 9007199254740993n;
const id = 42n;
```
