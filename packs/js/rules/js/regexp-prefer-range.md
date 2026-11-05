---
id: js/regexp-prefer-range
title: "Consecutive letters or digits are a range"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-range]
since: 0.5.0
---

Three or more consecutive digits or same-case letters in a class are written as a range: `[abcd]` is `[a-d]`.

**Why:** The range is shorter and shows its bounds.

```codelaw
native: js/regexp-prefer-range
```

## Examples

### JavaScript

#### Don't

```js
const re = /[abcd]/;
```

#### Do

```js
const re = /[a-d]/;
```

### JSX

#### Don't

```jsx
const re = /[abcd]/;
```

#### Do

```jsx
const re = /[a-d]/;
```

### TypeScript

#### Don't

```ts
const re = /[abcd]/;
```

#### Do

```ts
const re = /[a-d]/;
```

### TSX

#### Don't

```tsx
const re = /[abcd]/;
```

#### Do

```tsx
const re = /[a-d]/;
```
