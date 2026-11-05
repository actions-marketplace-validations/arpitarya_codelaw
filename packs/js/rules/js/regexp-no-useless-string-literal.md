---
id: js/regexp-no-useless-string-literal
title: "No string literal of single characters"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-string-literal]
since: 0.5.0
---

Under the `v` flag, a `\q{…}` string of one character is written in the class directly: `[\q{a}b]` is `[ab]`.

**Why:** The string literal adds nothing.

```codelaw
native: js/regexp-no-useless-string-literal
```

## Examples

### JavaScript

#### Don't

```js
const re = /[\q{a}b]/v;
```

#### Do

```js
const re = /[ab]/v;
```

### JSX

#### Don't

```jsx
const re = /[\q{a}b]/v;
```

#### Do

```jsx
const re = /[ab]/v;
```

### TypeScript

#### Don't

```ts
const re = /[\q{a}b]/v;
```

#### Do

```ts
const re = /[ab]/v;
```

### TSX

#### Don't

```tsx
const re = /[\q{a}b]/v;
```

#### Do

```tsx
const re = /[ab]/v;
```
