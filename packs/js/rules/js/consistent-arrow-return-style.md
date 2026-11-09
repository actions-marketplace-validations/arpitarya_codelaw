---
id: js/consistent-arrow-return-style
title: "A multi-line arrow body as a block; a one-line `return` as a concise body"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-arrow-return-style]
since: 0.5.0
---

An arrow whose concise body spans more than one line is rewritten as a block with `return`, re-indented one level; an arrow whose block holds only `return <value>` with a one-line value is rewritten as `=> value` (an object literal or a sequence wrapped in parentheses). A body with a comment or a multi-line string or template, and a block followed by a token that would join it to the next expression (`(`, `[`, a template, an operator), are reported without a fix.

**Why:** A concise body reads as one expression; once it spans lines, a block with `return` shows where it starts and ends.

```codelaw
native: js/consistent-arrow-return-style
```

## Examples

### JavaScript

#### Don't

```js
const total = (items) => {
  return items.length;
};
```

#### Do

```js
const total = (items) => items.length;
```

#### Fixed

```js
const total = (items) => items.length;
```


### JSX

#### Don't

```jsx
const total = (items) => {
  return items.length;
};
```

#### Do

```jsx
const total = (items) => items.length;
```

#### Fixed

```jsx
const total = (items) => items.length;
```


### TypeScript

#### Don't

```ts
const total = (items) => {
  return items.length;
};
```

#### Do

```ts
const total = (items) => items.length;
```

#### Fixed

```ts
const total = (items) => items.length;
```


### TSX

#### Don't

```tsx
const total = (items) => {
  return items.length;
};
```

#### Do

```tsx
const total = (items) => items.length;
```

#### Fixed

```tsx
const total = (items) => items.length;
```
