---
id: js/prefer-logical-operator-over-ternary
title: "a || b rather than a ? a : b"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`a` is evaluated once instead of twice, which matters if it has a side effect"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-logical-operator-over-ternary]
since: 0.5.0
---

A ternary whose condition and first branch are the same is written with `||`.

**Why:** The ternary repeats `a`; `||` says "a, or else b".

```codelaw
match:
  pattern: "$A ? $A : $B"
message: "Use `||`"
fix: "$A || $B"
```

## Examples

### JavaScript

#### Don't

```js
const name = input ? input : "anonymous";
```

#### Do

```js
const name = input || "anonymous";
```

### JSX

#### Don't

```jsx
const name = input ? input : "anonymous";
```

#### Do

```jsx
const name = input || "anonymous";
```

### TypeScript

#### Don't

```ts
const name = input ? input : "anonymous";
```

#### Do

```ts
const name = input || "anonymous";
```

### TSX

#### Don't

```tsx
const name = input ? input : "anonymous";
```

#### Do

```tsx
const name = input || "anonymous";
```
