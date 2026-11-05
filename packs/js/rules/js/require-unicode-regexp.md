---
id: js/require-unicode-regexp
title: "Regular expressions use the u or v flag"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the flag makes the pattern stricter, so an escape that was ignored becomes a syntax error; the reviewer confirms the pattern still compiles"
stability: preview
ported_from: [eslint/require-unicode-regexp]
since: 0.5.0
---

A regular-expression literal has the `u` or `v` flag.

**Why:** Without it, a character outside the Basic Multilingual Plane is two units, so `.` and classes split emoji and other symbols.

```codelaw

match:
  kind: regex
  not: { has: { kind: regex_flags, regex: '[uv]' } }
message: "Add the `u` (or `v`) flag"
fix:
  - insert-after: "u"
```

## How to fix

1. Check the pattern still compiles with the `u` flag.

## Examples

### JavaScript

#### Don't

```js
const word = /\w+/g;
```

#### Fixed

```js
const word = /\w+/gu;
```

#### Do

```js
const word = /\w+/gu;
```

### JSX

#### Don't

```jsx
const word = /\w+/g;
```

#### Fixed

```jsx
const word = /\w+/gu;
```

#### Do

```jsx
const word = /\w+/gu;
```

### TypeScript

#### Don't

```ts
const word = /\w+/g;
```

#### Fixed

```ts
const word = /\w+/gu;
```

#### Do

```ts
const word = /\w+/gu;
```

### TSX

#### Don't

```tsx
const word = /\w+/g;
```

#### Fixed

```tsx
const word = /\w+/gu;
```

#### Do

```tsx
const word = /\w+/gu;
```
