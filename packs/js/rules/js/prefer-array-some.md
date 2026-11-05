---
id: js/prefer-array-some
title: "some() rather than find() in a boolean test"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-some]
since: 0.5.0
---

`.filter(f).length > 0` is written `.some(f)`. Upstream also covers `find` in boolean positions; this port covers the filter-length form.

**Why:** `.some()` stops at the first match and returns a boolean.

```codelaw
match:
  pattern: "$A.filter($F).length > 0"
message: "Use `.some()`"
fix: "$A.some($F)"
```

## Examples

### JavaScript

#### Don't

```js
if (users.filter((u) => u.admin).length > 0) {
  warn();
}
```

#### Do

```js
if (users.some((u) => u.admin)) {
  warn();
}
```

### JSX

#### Don't

```jsx
if (users.filter((u) => u.admin).length > 0) {
  warn();
}
```

#### Do

```jsx
if (users.some((u) => u.admin)) {
  warn();
}
```

### TypeScript

#### Don't

```ts
if (users.filter((u) => u.admin).length > 0) {
  warn();
}
```

#### Do

```ts
if (users.some((u) => u.admin)) {
  warn();
}
```

### TSX

#### Don't

```tsx
if (users.filter((u) => u.admin).length > 0) {
  warn();
}
```

#### Do

```tsx
if (users.some((u) => u.admin)) {
  warn();
}
```
