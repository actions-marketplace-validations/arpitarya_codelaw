---
id: js/no-magic-array-flat-depth
title: "Name the depth given to flat()"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the name of the depth is the author's choice"
stability: preview
ported_from: [eslint-plugin-unicorn/no-magic-array-flat-depth]
since: 0.5.0
---

The depth given to `.flat()` is not a number other than 1 or `Infinity`.

**Why:** A bare depth like 3 hides why the data nests that deep.

```codelaw

match:
  pattern: $A.flat($N)
constraints:
  N: { kind: number, not: { regex: '^1$' } }
message: "Name the depth, or comment why it is `$N`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const flat = tree.flat(3);
```

#### Do

```js
const flat = tree.flat(Infinity);
```

### JSX

#### Don't

```jsx
const flat = tree.flat(3);
```

#### Do

```jsx
const flat = tree.flat(Infinity);
```

### TypeScript

#### Don't

```ts
const flat = tree.flat(3);
```

#### Do

```ts
const flat = tree.flat(Infinity);
```

### TSX

#### Don't

```tsx
const flat = tree.flat(3);
```

#### Do

```tsx
const flat = tree.flat(Infinity);
```
