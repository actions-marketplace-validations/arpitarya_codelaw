---
id: js/prefer-dom-node-text-content
title: "textContent rather than innerText"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`innerText` drops hidden text and applies CSS whitespace rules; `textContent` does not"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-dom-node-text-content]
since: 0.5.0
---

`innerText` is written `textContent`.

**Why:** `innerText` depends on CSS and forces layout; `textContent` reads the text directly.

```codelaw
match:
  pattern: "$E.innerText"
message: "Use `.textContent`"
fix: "$E.textContent"
```

## Examples

### JavaScript

#### Don't

```js
const label = el.innerText;
```

#### Do

```js
const label = el.textContent;
```

### JSX

#### Don't

```jsx
const label = el.innerText;
```

#### Do

```jsx
const label = el.textContent;
```

### TypeScript

#### Don't

```ts
const label = el.innerText;
```

#### Do

```ts
const label = el.textContent;
```

### TSX

#### Don't

```tsx
const label = el.innerText;
```

#### Do

```tsx
const label = el.textContent;
```
