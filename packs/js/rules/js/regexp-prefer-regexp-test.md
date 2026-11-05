---
id: js/regexp-prefer-regexp-test
title: "`RegExp#test` when only whether it matches is used"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`String#match` on a receiver that isn't a string may behave differently; the edit is for an agent or a person to accept"
stability: preview
ported_from: [eslint-plugin-regexp/prefer-regexp-test]
since: 0.5.0
---

`/re/.exec(s)` or `s.match(/re/)` whose result is used only as a condition is `/re/.test(s)`.

**Why:** `test` returns a boolean and builds no match array.

```codelaw
native: js/regexp-prefer-regexp-test
```

## How to fix

1. Call `test`.

## Examples

### JavaScript

#### Don't

```js
if (/a/.exec(s)) {}
```

#### Do

```js
if (/a/.test(s)) {}
```

### JSX

#### Don't

```jsx
if (/a/.exec(s)) {}
```

#### Do

```jsx
if (/a/.test(s)) {}
```

### TypeScript

#### Don't

```ts
if (/a/.exec(s)) {}
```

#### Do

```ts
if (/a/.test(s)) {}
```

### TSX

#### Don't

```tsx
if (/a/.exec(s)) {}
```

#### Do

```tsx
if (/a/.test(s)) {}
```
