---
id: js/prefer-negative-index
title: "slice(-n) rather than slice(a.length - n)"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-negative-index]
since: 0.5.0
---

`a.slice(a.length - n)` is written `a.slice(-n)`.

**Why:** Negative indexes count from the end already.

```codelaw
match:
  pattern: "$A.slice($A.length - $N)"
message: "Use a negative index"
fix: "$A.slice(-$N)"
```

## Examples

### JavaScript

#### Don't

```js
const tail = list.slice(list.length - 2);
```

#### Do

```js
const tail = list.slice(-2);
```

### JSX

#### Don't

```jsx
const tail = list.slice(list.length - 2);
```

#### Do

```jsx
const tail = list.slice(-2);
```

### TypeScript

#### Don't

```ts
const tail = list.slice(list.length - 2);
```

#### Do

```ts
const tail = list.slice(-2);
```

### TSX

#### Don't

```tsx
const tail = list.slice(list.length - 2);
```

#### Do

```tsx
const tail = list.slice(-2);
```
