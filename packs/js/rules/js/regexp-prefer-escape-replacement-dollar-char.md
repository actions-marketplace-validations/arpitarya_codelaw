---
id: js/regexp-prefer-escape-replacement-dollar-char
title: "A dollar sign in a replacement is written `$$`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-escape-replacement-dollar-char]
since: 0.5.0
---

A literal `$` in a replacement string is written `$$`.

**Why:** A lone `$` works by accident and breaks when a digit follows.

```codelaw
native: js/regexp-prefer-escape-replacement-dollar-char
```

## Examples

### JavaScript

#### Don't

```js
s.replace(/a/, 'cost: $ 5');
```

#### Do

```js
s.replace(/a/, 'cost: $$ 5');
```

### JSX

#### Don't

```jsx
s.replace(/a/, 'cost: $ 5');
```

#### Do

```jsx
s.replace(/a/, 'cost: $$ 5');
```

### TypeScript

#### Don't

```ts
s.replace(/a/, 'cost: $ 5');
```

#### Do

```ts
s.replace(/a/, 'cost: $$ 5');
```

### TSX

#### Don't

```tsx
s.replace(/a/, 'cost: $ 5');
```

#### Do

```tsx
s.replace(/a/, 'cost: $$ 5');
```
