---
id: js/regexp-no-useless-character-class
title: "No character class of one"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-character-class]
since: 0.5.0
---

A class holding one character or escape is written without the brackets: `[a]` is `a`, `[\d]` is `\d`.

**Why:** The brackets add nothing.

```codelaw
native: js/regexp-no-useless-character-class
```

## Examples

### JavaScript

#### Don't

```js
const re = /[a]b/;
```

#### Do

```js
const re = /ab/;
```

### JSX

#### Don't

```jsx
const re = /[a]b/;
```

#### Do

```jsx
const re = /ab/;
```

### TypeScript

#### Don't

```ts
const re = /[a]b/;
```

#### Do

```ts
const re = /ab/;
```

### TSX

#### Don't

```tsx
const re = /[a]b/;
```

#### Do

```tsx
const re = /ab/;
```
