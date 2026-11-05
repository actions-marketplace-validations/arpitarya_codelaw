---
id: js/regexp-prefer-named-capture-group
title: "Capturing groups are named"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the group's name is the author's to choose"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-regexp/prefer-named-capture-group]
since: 0.5.0
---

A capturing group has a name: `(?<year>\d{4})`, not `(\d{4})`.

**Why:** A name says what the group holds; a number doesn't.

```codelaw
native: js/regexp-prefer-named-capture-group
```

## How to fix

1. Give the group a name, and read it as `match.groups.name`.

## Examples

### JavaScript

#### Don't

```js
const re = /(\d{4})/;
```

#### Do

```js
const re = /(?<year>\d{4})/;
```

### JSX

#### Don't

```jsx
const re = /(\d{4})/;
```

#### Do

```jsx
const re = /(?<year>\d{4})/;
```

### TypeScript

#### Don't

```ts
const re = /(\d{4})/;
```

#### Do

```ts
const re = /(?<year>\d{4})/;
```

### TSX

#### Don't

```tsx
const re = /(\d{4})/;
```

#### Do

```tsx
const re = /(?<year>\d{4})/;
```
