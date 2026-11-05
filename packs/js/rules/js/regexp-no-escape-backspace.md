---
id: js/regexp-no-escape-backspace
title: "No `[\\b]` for a backspace"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-escape-backspace]
since: 0.5.0
---

A backspace in a class is written `\x08`, not `[\b]`, which reads as a word boundary.

**Why:** `\b` means two things; one of them is rare.

```codelaw
native: js/regexp-no-escape-backspace
```

## Examples

### JavaScript

#### Don't

```js
const re = /[\b]/;
```

#### Do

```js
const re = /[\x08]/;
```

### JSX

#### Don't

```jsx
const re = /[\b]/;
```

#### Do

```jsx
const re = /[\x08]/;
```

### TypeScript

#### Don't

```ts
const re = /[\b]/;
```

#### Do

```ts
const re = /[\x08]/;
```

### TSX

#### Don't

```tsx
const re = /[\b]/;
```

#### Do

```tsx
const re = /[\x08]/;
```
