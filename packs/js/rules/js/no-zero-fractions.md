---
id: js/no-zero-fractions
title: "No zero fraction on a number"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-zero-fractions]
since: 0.5.0
---

A number literal does not end in `.` or `.0`.

**Why:** A zero fraction looks like a precise decimal and is the same integer.

```codelaw

match:
  kind: number
  regex: '^[0-9_]+\.0*$'
message: "The fraction is zero; write the integer"
fix: "$$MATCH|number(strip-zero-fraction)"
```

## How to fix

1. Drop the `.0` (or the trailing `.`).

## Examples

### JavaScript

#### Don't

```js
const price = 10.0;
```

#### Do

```js
const price = 10;
```

### JSX

#### Don't

```jsx
const price = 10.0;
```

#### Do

```jsx
const price = 10;
```

### TypeScript

#### Don't

```ts
const price = 10.0;
```

#### Do

```ts
const price = 10;
```

### TSX

#### Don't

```tsx
const price = 10.0;
```

#### Do

```tsx
const price = 10;
```
