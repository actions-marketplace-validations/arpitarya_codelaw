---
id: js/no-div-regex
title: "No regex starting with ="
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/no-div-regex]
since: 0.5.0
---

A regular-expression literal does not begin with `=`.

**Why:** It reads like the division-assignment operator `/=`.

```codelaw

match:
  kind: regex
  has: { kind: regex_pattern, regex: '^=', pattern: $P }
transform:
  R: { substring: { source: $P, startChar: 1 } }
message: "A regex starting with `=` reads like `/=`; escape it as `[=]`"
fix:
  - replace: { target: $P, with: "[=]$R" }
```

## How to fix

1. Write the leading `=` as `[=]`.

## Examples

### JavaScript

#### Don't

```js
const eq = /=foo/;
```

#### Do

```js
const eq = /[=]foo/;
```

### JSX

#### Don't

```jsx
const eq = /=foo/;
```

#### Do

```jsx
const eq = /[=]foo/;
```

### TypeScript

#### Don't

```ts
const eq = /=foo/;
```

#### Do

```ts
const eq = /[=]foo/;
```

### TSX

#### Don't

```tsx
const eq = /=foo/;
```

#### Do

```tsx
const eq = /[=]foo/;
```
