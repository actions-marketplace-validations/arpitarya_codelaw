---
id: js/use-simple-number-keys
title: "Plain decimal numbers as object keys"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the decimal form has to be computed from the literal, which codelaw's fix templates cannot do"
fix_pending: "auto; needs computed-edit (W-162)"
stability: preview
ported_from: [biome/complexity/useSimpleNumberKeys]
since: 0.5.0
---

A numeric object key is written as a plain decimal: not hexadecimal, octal, binary, with an exponent, with separators or with a leading or trailing dot.

**Why:** The key becomes the decimal string anyway (`0x1` is the key `"1"`); writing it another way hides which key it is.

```codelaw
match:
  kind: number
  regex: '^(0[xXoObB]|.*[eE_]|\.|.*\.$|0[0-9])'
  inside:
    kind: pair
    field: key
message: "Write this key as a plain decimal number"
```

## How to fix

1. Write the key's decimal value (`0x10` → `16`, `1e2` → `100`).

## Examples

### JavaScript

#### Don't

```js
const codes = { 0x10: 'a' };
```

#### Do

```js
const codes = { 16: 'a' };
```

### JSX

#### Don't

```jsx
const codes = { 0x10: 'a' };
```

#### Do

```jsx
const codes = { 16: 'a' };
```

### TypeScript

#### Don't

```ts
const codes = { 0x10: 'a' };
```

#### Do

```ts
const codes = { 16: 'a' };
```

### TSX

#### Don't

```tsx
const codes = { 0x10: 'a' };
```

#### Do

```tsx
const codes = { 16: 'a' };
```

## Differences from the source

The source's fix computes the decimal value; this rule reports the key and leaves the rewrite to the author.
