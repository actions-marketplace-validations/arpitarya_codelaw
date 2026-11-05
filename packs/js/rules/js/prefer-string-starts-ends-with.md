---
id: js/prefer-string-starts-ends-with
title: "startsWith() rather than a ^ regex test"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the string literal is the pattern without its anchor, which a template cannot strip"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-string-starts-ends-with]
since: 0.5.0
---

A regex test of a plain literal prefix (`/^abc/.test(s)`) is written `s.startsWith("abc")`. Upstream also covers `$` suffixes; this port covers prefixes.

**Why:** `startsWith` needs no regex and says what is checked.

```codelaw

match:
  pattern: $R.test($S)
  has: { field: function, has: { field: object, kind: regex, not: { has: { kind: regex_flags } }, has: { kind: regex_pattern, regex: '^\^[A-Za-z0-9 _-]+$', pattern: $P } } }
message: "Use `.startsWith()` for a fixed prefix"
```

## How to fix

1. Write `s.startsWith("…")` with the text after `^`.

## Examples

### JavaScript

#### Don't

```js
if (/^http/.test(url)) {
  fetchIt();
}
```

#### Do

```js
if (url.startsWith("http")) {
  fetchIt();
}
```

### JSX

#### Don't

```jsx
if (/^http/.test(url)) {
  fetchIt();
}
```

#### Do

```jsx
if (url.startsWith("http")) {
  fetchIt();
}
```

### TypeScript

#### Don't

```ts
if (/^http/.test(url)) {
  fetchIt();
}
```

#### Do

```ts
if (url.startsWith("http")) {
  fetchIt();
}
```

### TSX

#### Don't

```tsx
if (/^http/.test(url)) {
  fetchIt();
}
```

#### Do

```tsx
if (url.startsWith("http")) {
  fetchIt();
}
```
