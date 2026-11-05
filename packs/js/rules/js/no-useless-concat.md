---
id: js/no-useless-concat
title: "No concatenation of two literals"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the joined literal has to be written out"
stability: preview
ported_from: [eslint/no-useless-concat, eslint-plugin-unicorn/no-useless-concat]
since: 0.5.0
---

Two string literals on the same line are not joined with `+`.

**Why:** One literal says the same and reads better.

```codelaw
match:
  pattern: $A + $B
  not: { regex: '\n' }
constraints:
  A: { any: [{kind: string}, {kind: template_string}] }
  B: { any: [{kind: string}, {kind: template_string}] }
message: "Join these literals into one"
```

## How to fix

1. Write the two literals as one.

## Examples

### JavaScript

#### Don't

```js
const s = 'a' + 'b';
```

#### Do

```js
const s = 'ab';
```

### JSX

#### Don't

```jsx
const s = 'a' + 'b';
```

#### Do

```jsx
const s = 'ab';
```

### TypeScript

#### Don't

```ts
const s = 'a' + 'b';
```

#### Do

```ts
const s = 'ab';
```

### TSX

#### Don't

```tsx
const s = 'a' + 'b';
```

#### Do

```tsx
const s = 'ab';
```
