---
id: js/prefer-query-selector
title: "querySelector rather than getElementById"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the selector string is built from the id, which a template cannot escape"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-query-selector]
since: 0.5.0
---

`getElementById`, `getElementsByClassName` and `getElementsByTagName` are written with `querySelector`/`querySelectorAll`.

**Why:** One selector API for every lookup.

```codelaw
match:
  pattern: "document.getElementById($I)"
message: "Use `.querySelector()`"
```

## How to fix

1. Write `document.querySelector("#id")` (escape the id with `CSS.escape` if it is not a plain name).

## Examples

### JavaScript

#### Don't

```js
const app = document.getElementById("app");
```

#### Do

```js
const app = document.querySelector("#app");
```

### JSX

#### Don't

```jsx
const app = document.getElementById("app");
```

#### Do

```jsx
const app = document.querySelector("#app");
```

### TypeScript

#### Don't

```ts
const app = document.getElementById("app");
```

#### Do

```ts
const app = document.querySelector("#app");
```

### TSX

#### Don't

```tsx
const app = document.getElementById("app");
```

#### Do

```tsx
const app = document.querySelector("#app");
```
