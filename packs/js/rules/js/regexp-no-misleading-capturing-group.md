---
id: js/regexp-no-misleading-capturing-group
title: "No capturing group whose start the quantifier before it always takes"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "making the quantifier before the group lazy changes what it matches; it is offered for an agent or a person to accept"
stability: preview
ported_from: [eslint-plugin-regexp/no-misleading-capturing-group]
since: 0.5.0
---

A capturing group whose first element is optional isn't right after a greedy quantifier that matches the same characters: in `^\w+(\d*)$` the group always captures nothing.

**Why:** The group looks like it captures digits; it never does.

```codelaw
native: js/regexp-no-misleading-capturing-group
```

## How to fix

1. Make the quantifier before it lazy, or narrow what it matches.

## Examples

### JavaScript

#### Don't

```js
const re = /^\w+(\d*)$/;
```

#### Do

```js
const re = /^\w+?(\d*)$/;
```

### JSX

#### Don't

```jsx
const re = /^\w+(\d*)$/;
```

#### Do

```jsx
const re = /^\w+?(\d*)$/;
```

### TypeScript

#### Don't

```ts
const re = /^\w+(\d*)$/;
```

#### Do

```ts
const re = /^\w+?(\d*)$/;
```

### TSX

#### Don't

```tsx
const re = /^\w+(\d*)$/;
```

#### Do

```tsx
const re = /^\w+?(\d*)$/;
```
