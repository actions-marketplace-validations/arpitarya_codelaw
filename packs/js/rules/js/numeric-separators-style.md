---
id: js/numeric-separators-style
title: "Group digits of long numbers"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the grouped literal is a computed edit"
stability: preview
ported_from: [eslint-plugin-unicorn/numeric-separators-style]
since: 0.5.0
---

A decimal integer literal of five or more digits groups them in threes with `_`. Upstream also groups hexadecimal, binary and fractional parts; this port covers decimal integers.

**Why:** Grouped digits can be read at a glance.

```codelaw

match:
  kind: number
  regex: '^[0-9]{5,}$'
message: "Group the digits with `_` (`1_000_000`)"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const max = 1000000;
```

#### Do

```js
const max = 1_000_000;
```

### JSX

#### Don't

```jsx
const max = 1000000;
```

#### Do

```jsx
const max = 1_000_000;
```

### TypeScript

#### Don't

```ts
const max = 1000000;
```

#### Do

```ts
const max = 1_000_000;
```

### TSX

#### Don't

```tsx
const max = 1000000;
```

#### Do

```tsx
const max = 1_000_000;
```
