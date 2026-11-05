---
id: js/regexp-no-useless-non-capturing-group
title: "No non-capturing group that groups nothing"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-non-capturing-group]
since: 0.5.0
---

A non-capturing group is there to bound an alternation or a quantifier: `(?:ab)c` is `abc`, `(?:a)+` is `a+`.

**Why:** The brackets add noise.

```codelaw
native: js/regexp-no-useless-non-capturing-group
```

## Examples

### JavaScript

#### Don't

```js
const re = /(?:ab)c/;
```

#### Do

```js
const re = /abc/;
```

### JSX

#### Don't

```jsx
const re = /(?:ab)c/;
```

#### Do

```jsx
const re = /abc/;
```

### TypeScript

#### Don't

```ts
const re = /(?:ab)c/;
```

#### Do

```ts
const re = /abc/;
```

### TSX

#### Don't

```tsx
const re = /(?:ab)c/;
```

#### Do

```tsx
const re = /abc/;
```
