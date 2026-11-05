---
id: js/regexp-no-empty-lookarounds-assertion
title: "No lookaround that can match the empty string"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the lookaround should check is the author's to write"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-regexp/no-empty-lookarounds-assertion]
since: 0.5.0
---

A lookaround doesn't match the empty string: `(?=)`, `(?=a*)`. A positive one then always succeeds, a negative one always fails.

**Why:** The assertion asserts nothing.

```codelaw
native: js/regexp-no-empty-lookarounds-assertion
```

## How to fix

1. Make the lookaround require the characters it should check.

## Examples

### JavaScript

#### Don't

```js
const re = /a(?=b*)/;
```

#### Do

```js
const re = /a(?=b)/;
```

### JSX

#### Don't

```jsx
const re = /a(?=b*)/;
```

#### Do

```jsx
const re = /a(?=b)/;
```

### TypeScript

#### Don't

```ts
const re = /a(?=b*)/;
```

#### Do

```ts
const re = /a(?=b)/;
```

### TSX

#### Don't

```tsx
const re = /a(?=b*)/;
```

#### Do

```tsx
const re = /a(?=b)/;
```
