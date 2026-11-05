---
id: js/no-useless-undefined
title: "No return undefined"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-undefined]
since: 0.5.0
---

`return undefined;` is written `return;`. Upstream also covers `let x = undefined` and `undefined` arguments; this port covers the return.

**Why:** A bare `return` already returns `undefined`.

```codelaw
match:
  pattern: "return undefined;"
message: "A bare `return` already returns `undefined`"
fix: "return;"
```

## Examples

### JavaScript

#### Don't

```js
function find() {
  return undefined;
}
```

#### Do

```js
function find() {
  return;
}
```

### JSX

#### Don't

```jsx
function find() {
  return undefined;
}
```

#### Do

```jsx
function find() {
  return;
}
```

### TypeScript

#### Don't

```ts
function find() {
  return undefined;
}
```

#### Do

```ts
function find() {
  return;
}
```

### TSX

#### Don't

```tsx
function find() {
  return undefined;
}
```

#### Do

```tsx
function find() {
  return;
}
```
