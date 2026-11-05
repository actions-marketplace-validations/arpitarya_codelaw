---
id: js/regexp-no-useless-escape
title: "No useless escape in a regular expression"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-escape]
since: 0.5.0
---

A backslash in a pattern changes what the next character means: `\a` is `a`; `[\.]` is `[.]`. Escapes in strings are `js/no-useless-escape`.

**Why:** A needless escape makes a reader look for a meaning that isn't there.

```codelaw
native: js/regexp-no-useless-escape
```

## Examples

### JavaScript

#### Don't

```js
const re = /\a/;
```

#### Do

```js
const re = /a/;
```

### JSX

#### Don't

```jsx
const re = /\a/;
```

#### Do

```jsx
const re = /a/;
```

### TypeScript

#### Don't

```ts
const re = /\a/;
```

#### Do

```ts
const re = /a/;
```

### TSX

#### Don't

```tsx
const re = /\a/;
```

#### Do

```tsx
const re = /a/;
```
