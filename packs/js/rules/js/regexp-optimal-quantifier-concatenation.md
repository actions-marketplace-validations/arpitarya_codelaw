---
id: js/regexp-optimal-quantifier-concatenation
title: "No adjacent quantifiers that one quantifier covers"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/optimal-quantifier-concatenation]
since: 0.5.0
---

Two adjacent quantified elements aren't simpler as one: `a+a*` is `a+`; in `\w+\d*` the `\d*` never matches anything.

**Why:** Redundant quantifiers backtrack for nothing.

```codelaw
native: js/regexp-optimal-quantifier-concatenation
```

## Examples

### JavaScript

#### Don't

```js
const re = /\w+\d*/;
```

#### Do

```js
const re = /\w+/;
```

### JSX

#### Don't

```jsx
const re = /\w+\d*/;
```

#### Do

```jsx
const re = /\w+/;
```

### TypeScript

#### Don't

```ts
const re = /\w+\d*/;
```

#### Do

```ts
const re = /\w+/;
```

### TSX

#### Don't

```tsx
const re = /\w+\d*/;
```

#### Do

```tsx
const re = /\w+/;
```
