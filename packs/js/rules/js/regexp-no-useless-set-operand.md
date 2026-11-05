---
id: js/regexp-no-useless-set-operand
title: "No set operand that changes nothing"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-set-operand]
since: 0.5.0
---

Under the `v` flag, an intersection's operand isn't a superset of the other, and a subtraction's isn't disjoint from what it subtracts from: `[\w&&\d]` is `[\d]`.

**Why:** The operand adds nothing.

```codelaw
native: js/regexp-no-useless-set-operand
```

## Examples

### JavaScript

#### Don't

```js
const re = /[\w&&\d]/v;
```

#### Do

```js
const re = /[\d]/v;
```

### JSX

#### Don't

```jsx
const re = /[\w&&\d]/v;
```

#### Do

```jsx
const re = /[\d]/v;
```

### TypeScript

#### Don't

```ts
const re = /[\w&&\d]/v;
```

#### Do

```ts
const re = /[\d]/v;
```

### TSX

#### Don't

```tsx
const re = /[\w&&\d]/v;
```

#### Do

```tsx
const re = /[\d]/v;
```
