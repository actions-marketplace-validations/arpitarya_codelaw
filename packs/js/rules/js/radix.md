---
id: js/radix
title: "parseInt gets a radix"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "base 10 is the usual intent, but a string with a `0x` prefix parses differently with it"
stability: preview
ported_from: [eslint/radix]
since: 0.5.0
---

`parseInt` and `Number.parseInt` are called with their second argument, the radix.

**Why:** Without it the base is guessed from the string (`0x` means 16), which surprises.

```codelaw
match:
  any:
    - pattern: parseInt($S)
    - pattern: Number.parseInt($S)
message: "Pass the radix to `parseInt`"
fix: "parseInt($S, 10)"
```

## Examples

### JavaScript

#### Don't

```js
const n = parseInt(text);
```

#### Do

```js
const n = parseInt(text, 10);
```

### JSX

#### Don't

```jsx
const n = parseInt(text);
```

#### Do

```jsx
const n = parseInt(text, 10);
```

### TypeScript

#### Don't

```ts
const n = parseInt(text);
```

#### Do

```ts
const n = parseInt(text, 10);
```

### TSX

#### Don't

```tsx
const n = parseInt(text);
```

#### Do

```tsx
const n = parseInt(text, 10);
```
