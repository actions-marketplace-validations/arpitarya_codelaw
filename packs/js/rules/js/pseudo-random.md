---
id: js/pseudo-random
title: "Math.random() in security code"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the value needs to be unpredictable is the author's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/pseudo-random]
since: 0.5.0
---

`Math.random()` is reviewed: it must not produce tokens, ids or keys.

**Why:** Its output can be predicted from earlier values, so a token made from it can be guessed.

```codelaw

match:
  pattern: Math.random()
message: "`Math.random()` is predictable; use `crypto.getRandomValues()` or `crypto.randomUUID()` for anything secret"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const token = Math.random().toString(36);
```

#### Do

```js
const token = crypto.randomUUID();
```

### JSX

#### Don't

```jsx
const token = Math.random().toString(36);
```

#### Do

```jsx
const token = crypto.randomUUID();
```

### TypeScript

#### Don't

```ts
const token = Math.random().toString(36);
```

#### Do

```ts
const token = crypto.randomUUID();
```

### TSX

#### Don't

```tsx
const token = Math.random().toString(36);
```

#### Do

```tsx
const token = crypto.randomUUID();
```
