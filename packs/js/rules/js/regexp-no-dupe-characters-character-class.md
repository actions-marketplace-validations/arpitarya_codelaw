---
id: js/regexp-no-dupe-characters-character-class
title: "No character listed twice in a class"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-dupe-characters-character-class]
since: 0.5.0
---

A class doesn't list a character, range or escape that another of its elements already matches: `[aab]`, `[\da-z0-5]`.

**Why:** A duplicate is noise, and often a typo for another character.

```codelaw
native: js/regexp-no-dupe-characters-character-class
```

## Examples

### JavaScript

#### Don't

```js
const re = /[aab]/;
```

#### Do

```js
const re = /[ab]/;
```

### JSX

#### Don't

```jsx
const re = /[aab]/;
```

#### Do

```jsx
const re = /[ab]/;
```

### TypeScript

#### Don't

```ts
const re = /[aab]/;
```

#### Do

```ts
const re = /[ab]/;
```

### TSX

#### Don't

```tsx
const re = /[aab]/;
```

#### Do

```tsx
const re = /[ab]/;
```
