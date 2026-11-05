---
id: js/prefer-regexp-test
title: "regex.test() rather than match() in a condition"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-regexp-test]
since: 0.5.0
---

A `.match()` used only as an `if` condition is written `regex.test(string)`.

**Why:** `test` returns a boolean and builds no match array.

```codelaw
match:
  pattern: "$S.match($R)"
  inside:
    kind: parenthesized_expression
    inside: { kind: if_statement, field: condition }
constraints:
  R: { kind: regex, not: { has: { kind: regex_flags, regex: g } } }
message: "Use `.test()`"
fix: "$R.test($S)"
```

## Examples

### JavaScript

#### Don't

```js
if (name.match(/^a/)) {
  start();
}
```

#### Do

```js
if (/^a/.test(name)) {
  start();
}
```

### JSX

#### Don't

```jsx
if (name.match(/^a/)) {
  start();
}
```

#### Do

```jsx
if (/^a/.test(name)) {
  start();
}
```

### TypeScript

#### Don't

```ts
if (name.match(/^a/)) {
  start();
}
```

#### Do

```ts
if (/^a/.test(name)) {
  start();
}
```

### TSX

#### Don't

```tsx
if (name.match(/^a/)) {
  start();
}
```

#### Do

```tsx
if (/^a/.test(name)) {
  start();
}
```
