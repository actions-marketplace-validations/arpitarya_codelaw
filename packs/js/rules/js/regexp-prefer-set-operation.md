---
id: js/regexp-prefer-set-operation
title: "A negative lookahead before a class is a set subtraction"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-set-operation]
since: 0.5.0
---

Under the `v` flag, `(?!a)\w` is written `[\w--a]`.

**Why:** One class says it, without a lookaround.

```codelaw
native: js/regexp-prefer-set-operation
```

## Examples

### JavaScript

#### Don't

```js
const re = /(?!a)\w/v;
```

#### Do

```js
const re = /[\w--a]/v;
```

### JSX

#### Don't

```jsx
const re = /(?!a)\w/v;
```

#### Do

```jsx
const re = /[\w--a]/v;
```

### TypeScript

#### Don't

```ts
const re = /(?!a)\w/v;
```

#### Do

```ts
const re = /[\w--a]/v;
```

### TSX

#### Don't

```tsx
const re = /(?!a)\w/v;
```

#### Do

```tsx
const re = /[\w--a]/v;
```
