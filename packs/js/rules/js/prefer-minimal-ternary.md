---
id: js/prefer-minimal-ternary
title: "f(c ? a : b) rather than c ? f(a) : f(b)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the function is now evaluated before the condition"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-minimal-ternary]
since: 0.5.0
---

A ternary whose two branches call the same function with one argument is written as one call with a ternary argument.

**Why:** The call is written once and the difference stands out.

```codelaw
match:
  pattern: "$C ? $F($A) : $F($B)"
message: "Move the ternary into the call"
fix: "$F($C ? $A : $B)"
```

## Examples

### JavaScript

#### Don't

```js
const label = big ? format(a) : format(b);
```

#### Do

```js
const label = format(big ? a : b);
```

### JSX

#### Don't

```jsx
const label = big ? format(a) : format(b);
```

#### Do

```jsx
const label = format(big ? a : b);
```

### TypeScript

#### Don't

```ts
const label = big ? format(a) : format(b);
```

#### Do

```ts
const label = format(big ? a : b);
```

### TSX

#### Don't

```tsx
const label = big ? format(a) : format(b);
```

#### Do

```tsx
const label = format(big ? a : b);
```
