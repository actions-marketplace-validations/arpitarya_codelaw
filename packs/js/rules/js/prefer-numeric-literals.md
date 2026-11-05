---
id: js/prefer-numeric-literals
title: "0b101 rather than parseInt(\"101\", 2)"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the literal's prefix depends on the radix, which a template cannot choose"
fix_pending: "auto; needs computed-edit"
stability: preview
ported_from: [eslint/prefer-numeric-literals]
since: 0.5.0
---

`parseInt` and `Number.parseInt` are not called on a string literal with radix 2, 8 or 16.

**Why:** A literal (`0b1111`, `0o17`, `0xf`) is evaluated at parse time and reads as a number.

```codelaw

match:
  kind: call_expression
  any:
    - pattern: parseInt($S, $R)
    - pattern: Number.parseInt($S, $R)
  has: { field: arguments, all: [{ has: { nthChild: 1, kind: string, regex: '^.[0-9a-fA-F]+.$' } }, { has: { nthChild: 2, kind: number, regex: '^(2|8|16)$' } }] }
message: "Write the number as a binary, octal or hexadecimal literal"
```

## How to fix

1. Write `0b…` for radix 2, `0o…` for 8 and `0x…` for 16, with the same digits.

## Examples

### JavaScript

#### Don't

```js
const mask = parseInt("1111", 2);
```

#### Do

```js
const mask = 0b1111;
```

### JSX

#### Don't

```jsx
const mask = parseInt("1111", 2);
```

#### Do

```jsx
const mask = 0b1111;
```

### TypeScript

#### Don't

```ts
const mask = parseInt("1111", 2);
```

#### Do

```ts
const mask = 0b1111;
```

### TSX

#### Don't

```tsx
const mask = parseInt("1111", 2);
```

#### Do

```tsx
const mask = 0b1111;
```
