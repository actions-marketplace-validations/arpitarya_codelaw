---
id: js/regexp-no-optional-assertion
title: "No optional assertion"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-optional-assertion]
since: 0.5.0
---

An assertion isn't the whole of an optional group: `(?:\b)?` asserts nothing, since the group can always be skipped.

**Why:** The assertion looks like a check and isn't one.

```codelaw
native: js/regexp-no-optional-assertion
```

## Examples

### JavaScript

#### Don't

```js
const re = /a(?:$)?/;
```

#### Do

```js
const re = /a/;
```

### JSX

#### Don't

```jsx
const re = /a(?:$)?/;
```

#### Do

```jsx
const re = /a/;
```

### TypeScript

#### Don't

```ts
const re = /a(?:$)?/;
```

#### Do

```ts
const re = /a/;
```

### TSX

#### Don't

```tsx
const re = /a(?:$)?/;
```

#### Do

```tsx
const re = /a/;
```
