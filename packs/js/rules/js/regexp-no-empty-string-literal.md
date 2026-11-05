---
id: js/regexp-no-empty-string-literal
title: "No empty string literal in a class"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which strings the literal should hold is the author's to write"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-regexp/no-empty-string-literal]
since: 0.5.0
---

A `\q{}` string literal holds something; an empty one makes the class match the empty string.

**Why:** It is almost always a forgotten edit.

```codelaw
native: js/regexp-no-empty-string-literal
```

## How to fix

1. Write the strings the literal should hold, or remove it.

## Examples

### JavaScript

#### Don't

```js
const re = /[\q{}a]/v;
```

#### Do

```js
const re = /[\q{bc}a]/v;
```

### JSX

#### Don't

```jsx
const re = /[\q{}a]/v;
```

#### Do

```jsx
const re = /[\q{bc}a]/v;
```

### TypeScript

#### Don't

```ts
const re = /[\q{}a]/v;
```

#### Do

```ts
const re = /[\q{bc}a]/v;
```

### TSX

#### Don't

```tsx
const re = /[\q{}a]/v;
```

#### Do

```tsx
const re = /[\q{bc}a]/v;
```
