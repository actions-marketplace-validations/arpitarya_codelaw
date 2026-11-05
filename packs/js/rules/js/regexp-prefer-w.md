---
id: js/regexp-prefer-w
title: "Word characters are matched with `\\w`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-w]
since: 0.5.0
---

`[a-zA-Z0-9_]` is `\w` (and `[a-z0-9_]` with `i`).

**Why:** The escape is shorter and plainer.

```codelaw
native: js/regexp-prefer-w
```

## Examples

### JavaScript

#### Don't

```js
const re = /[a-zA-Z0-9_]+/;
```

#### Do

```js
const re = /\w+/;
```

### JSX

#### Don't

```jsx
const re = /[a-zA-Z0-9_]+/;
```

#### Do

```jsx
const re = /\w+/;
```

### TypeScript

#### Don't

```ts
const re = /[a-zA-Z0-9_]+/;
```

#### Do

```ts
const re = /\w+/;
```

### TSX

#### Don't

```tsx
const re = /[a-zA-Z0-9_]+/;
```

#### Do

```tsx
const re = /\w+/;
```
