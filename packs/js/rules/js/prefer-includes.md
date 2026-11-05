---
id: js/prefer-includes
title: "includes() rather than indexOf() !== -1"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`includes` finds `NaN`, which `indexOf` never does"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-includes]
since: 0.5.0
---

`.indexOf(x) !== -1` is written `.includes(x)`.

**Why:** `includes` says what is asked and also finds `NaN`.

```codelaw
match:
  pattern: "$A.indexOf($X) !== -1"
message: "Use `.includes()`"
fix: "$A.includes($X)"
```

## Examples

### JavaScript

#### Don't

```js
if (list.indexOf(id) !== -1) {
  keep();
}
```

#### Do

```js
if (list.includes(id)) {
  keep();
}
```

### JSX

#### Don't

```jsx
if (list.indexOf(id) !== -1) {
  keep();
}
```

#### Do

```jsx
if (list.includes(id)) {
  keep();
}
```

### TypeScript

#### Don't

```ts
if (list.indexOf(id) !== -1) {
  keep();
}
```

#### Do

```ts
if (list.includes(id)) {
  keep();
}
```

### TSX

#### Don't

```tsx
if (list.indexOf(id) !== -1) {
  keep();
}
```

#### Do

```tsx
if (list.includes(id)) {
  keep();
}
```
