---
id: js/no-instanceof-builtins
title: "Array.isArray rather than instanceof Array"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "an array from another realm now passes the check"
stability: preview
ported_from: [eslint-plugin-unicorn/no-instanceof-builtins]
since: 0.5.0
---

`x instanceof Array` is written `Array.isArray(x)`. Upstream also covers the primitive wrappers (`instanceof String`); this port covers arrays.

**Why:** `instanceof Array` is false for an array from an iframe or another realm; `Array.isArray` is not.

```codelaw

match:
  pattern: $X instanceof Array
message: "`instanceof Array` fails for arrays from another realm; use `Array.isArray($X)`"
fix: "Array.isArray($X)"
```

## Examples

### JavaScript

#### Don't

```js
if (value instanceof Array) {
  flat(value);
}
```

#### Do

```js
if (Array.isArray(value)) {
  flat(value);
}
```

### JSX

#### Don't

```jsx
if (value instanceof Array) {
  flat(value);
}
```

#### Do

```jsx
if (Array.isArray(value)) {
  flat(value);
}
```

### TypeScript

#### Don't

```ts
if (value instanceof Array) {
  flat(value);
}
```

#### Do

```ts
if (Array.isArray(value)) {
  flat(value);
}
```

### TSX

#### Don't

```tsx
if (value instanceof Array) {
  flat(value);
}
```

#### Do

```tsx
if (Array.isArray(value)) {
  flat(value);
}
```
