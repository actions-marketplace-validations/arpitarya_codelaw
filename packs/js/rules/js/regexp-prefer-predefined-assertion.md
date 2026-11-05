---
id: js/regexp-prefer-predefined-assertion
title: "A lookaround that is a predefined assertion is written as one"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-predefined-assertion]
since: 0.5.0
---

A lookaround equal to `^`, `$`, `\b` or `\B` is written as that assertion: `a(?!\w)` is `a\b`; `(?![\s\S])` is `$` without `m`.

**Why:** The assertion is shorter and plainer.

```codelaw
native: js/regexp-prefer-predefined-assertion
```

## Examples

### JavaScript

#### Don't

```js
const re = /a(?!\w)/;
```

#### Do

```js
const re = /a\b/;
```

### JSX

#### Don't

```jsx
const re = /a(?!\w)/;
```

#### Do

```jsx
const re = /a\b/;
```

### TypeScript

#### Don't

```ts
const re = /a(?!\w)/;
```

#### Do

```ts
const re = /a\b/;
```

### TSX

#### Don't

```tsx
const re = /a(?!\w)/;
```

#### Do

```tsx
const re = /a\b/;
```
