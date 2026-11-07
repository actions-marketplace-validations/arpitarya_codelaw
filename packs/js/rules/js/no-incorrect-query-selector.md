---
id: js/no-incorrect-query-selector
title: "querySelector() rather than the first element of querySelectorAll()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "with no match the first element of the list is `undefined` while `querySelector()` returns `null`; code that compares with `undefined` needs review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-incorrect-query-selector]
since: 0.5.0
---

Taking the first element of `querySelectorAll(selector)`, by `[0]` or `.at(0)`, is `querySelector(selector)`.

**Why:** `querySelector` stops at the first match instead of collecting every one first.

```codelaw
match:
  any:
    - pattern: "$D.querySelectorAll($S)[0]"
    - pattern: "$D.querySelectorAll($S).at(0)"
message: "Use `querySelector`"
fix: "$D.querySelector($S)"
```

## Examples

### JavaScript

#### Don't

```js
const form = document.querySelectorAll("form")[0];
```

#### Do

```js
const form = document.querySelector("form");
```


### JSX

#### Don't

```jsx
const form = document.querySelectorAll("form")[0];
```

#### Do

```jsx
const form = document.querySelector("form");
```


### TypeScript

#### Don't

```ts
const form = document.querySelectorAll("form")[0];
```

#### Do

```ts
const form = document.querySelector("form");
```


### TSX

#### Don't

```tsx
const form = document.querySelectorAll("form")[0];
```

#### Do

```tsx
const form = document.querySelector("form");
```
