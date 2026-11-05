---
id: js/no-incorrect-template-string-interpolation
title: "No ${} in a plain string"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "changing the quotes may need re-escaping the text"
stability: preview
ported_from: [eslint-plugin-unicorn/no-incorrect-template-string-interpolation]
since: 0.5.0
---

A single- or double-quoted string does not contain `${…}`.

**Why:** The text is printed literally; backticks were meant.

```codelaw

match:
  kind: string
  regex: '\$\{[^}]+\}'
message: "`${…}` only interpolates in a template literal; use backticks"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const msg = "Hello ${name}";
```

#### Do

```js
const msg = `Hello ${name}`;
```

### JSX

#### Don't

```jsx
const msg = "Hello ${name}";
```

#### Do

```jsx
const msg = `Hello ${name}`;
```

### TypeScript

#### Don't

```ts
const msg = "Hello ${name}";
```

#### Do

```ts
const msg = `Hello ${name}`;
```

### TSX

#### Don't

```tsx
const msg = "Hello ${name}";
```

#### Do

```tsx
const msg = `Hello ${name}`;
```
