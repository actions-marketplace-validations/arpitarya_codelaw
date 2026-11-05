---
id: js/regexp-prefer-d
title: "Digits are matched with `\\d`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-d]
since: 0.5.0
---

`[0-9]` is `\d` (and `[^0-9]` is `\D`).

**Why:** The escape is shorter and plainer.

```codelaw
native: js/regexp-prefer-d
```

## Examples

### JavaScript

#### Don't

```js
const re = /[0-9]+/;
```

#### Do

```js
const re = /\d+/;
```

### JSX

#### Don't

```jsx
const re = /[0-9]+/;
```

#### Do

```jsx
const re = /\d+/;
```

### TypeScript

#### Don't

```ts
const re = /[0-9]+/;
```

#### Do

```ts
const re = /\d+/;
```

### TSX

#### Don't

```tsx
const re = /[0-9]+/;
```

#### Do

```tsx
const re = /\d+/;
```
