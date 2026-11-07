---
id: js/prefer-query-selector
title: "querySelector rather than getElementById"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`querySelector` reads its argument as a selector and `getElementById` as a name; the author confirms nothing else relied on the old call"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-query-selector]
since: 0.5.0
---

`getElementById`, `getElementsByClassName` and `getElementsByTagName` are written with `querySelector`/`querySelectorAll`.

**Why:** One selector API for every lookup.

```codelaw
match:
  pattern: "document.getElementById($I)"
transform:
  Q: { substring: { source: $I, endChar: 1 } }
  ID: { substring: { source: $I, startChar: 1, endChar: -1 } }
message: "Use `.querySelector()`"
fix: "document.querySelector($Q|map(':', \":\")#$ID|escape(css-ident)|escape(js-string)$Q)"
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
