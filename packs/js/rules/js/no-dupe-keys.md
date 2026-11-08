---
id: js/no-dupe-keys
title: "No two keys in an object literal are the same"
languages: [javascript, jsx]
fix: suggest
fix_reason: "the later key is the one the object keeps, so the choices are deleting the earlier (the default, which keeps the value; its own evaluation goes with it) or the later"
fix_exception: value
stability: preview
ported_from: [eslint/no-dupe-keys]
since: 0.1.0
---

An object literal names each key once; `a`, `'a'` and `["a"]` are the same key, as are `1` and
`0x1`.

**Why:** The later value silently replaces the earlier one.

```codelaw
native: js/no-dupe-keys
```

## How to fix

1. Decide which value the key should have.
2. Remove the other entry, or give it its own key.

## Examples

### JavaScript

#### Don't

```js
const user = {
  name: "Ada",
  name: "Grace",
};
```

#### Fixed

```js
const user = {
  name: "Grace",
};
```

#### Do

```js
const user = {
  name: "Ada",
  nickname: "Grace",
};
```

### JSX

#### Don't

```jsx
const user = {
  name: "Ada",
  name: "Grace",
};
```

#### Fixed

```jsx
const user = {
  name: "Grace",
};
```

#### Do

```jsx
const user = {
  name: "Ada",
  nickname: "Grace",
};
```
