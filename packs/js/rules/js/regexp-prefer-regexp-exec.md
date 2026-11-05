---
id: js/regexp-prefer-regexp-exec
title: "`RegExp#exec` over `String#match` without `g`"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`String#match` on a receiver that isn't a string may behave differently; the edit is for an agent or a person to accept"
stability: preview
ported_from: [eslint-plugin-regexp/prefer-regexp-exec]
since: 0.5.0
---

`s.match(/re/)` with a regular expression without `g` is written `/re/.exec(s)`.

**Why:** `exec` says the regular expression drives the match, and is what `match` calls.

```codelaw
native: js/regexp-prefer-regexp-exec
```

## How to fix

1. Call `exec` on the regular expression with the string.

## Examples

### JavaScript

#### Don't

```js
const m = s.match(/a/);
```

#### Do

```js
const m = /a/.exec(s);
```

### JSX

#### Don't

```jsx
const m = s.match(/a/);
```

#### Do

```jsx
const m = /a/.exec(s);
```

### TypeScript

#### Don't

```ts
const m = s.match(/a/);
```

#### Do

```ts
const m = /a/.exec(s);
```

### TSX

#### Don't

```tsx
const m = s.match(/a/);
```

#### Do

```tsx
const m = /a/.exec(s);
```
