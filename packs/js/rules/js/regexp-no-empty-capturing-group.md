---
id: js/regexp-no-empty-capturing-group
title: "No capturing group that captures nothing"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the group should capture is the author's to write"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-regexp/no-empty-capturing-group]
since: 0.5.0
---

A capturing group holds something that can match characters, not only assertions: `(\b)` always captures an empty string.

**Why:** The group's number is taken for nothing.

```codelaw
native: js/regexp-no-empty-capturing-group
```

## How to fix

1. Put what should be captured inside the group, or remove it.

## Examples

### JavaScript

#### Don't

```js
const re = /a(\b)b/;
```

#### Do

```js
const re = /a\bb/;
```

### JSX

#### Don't

```jsx
const re = /a(\b)b/;
```

#### Do

```jsx
const re = /a\bb/;
```

### TypeScript

#### Don't

```ts
const re = /a(\b)b/;
```

#### Do

```ts
const re = /a\bb/;
```

### TSX

#### Don't

```tsx
const re = /a(\b)b/;
```

#### Do

```tsx
const re = /a\bb/;
```
