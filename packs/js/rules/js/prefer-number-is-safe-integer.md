---
id: js/prefer-number-is-safe-integer
title: "Number.isSafeInteger() rather than its two halves"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the two forms agree on every number, but the rewrite drops a hand-written bound the author may have meant to differ; offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-number-is-safe-integer]
since: 0.5.0
---

A test that a number is an integer within the safe range is `Number.isSafeInteger(x)`, not `Number.isInteger(x) && Math.abs(x) <= Number.MAX_SAFE_INTEGER`.

**Why:** The built-in names the check and keeps the bound right.

```codelaw
match:
  pattern: "Number.isInteger($X) && Math.abs($X) <= Number.MAX_SAFE_INTEGER"
message: "Use `Number.isSafeInteger($X)`"
fix: "Number.isSafeInteger($X)"
```

## Examples

### JavaScript

#### Don't

```js
const ok = Number.isInteger(id) && Math.abs(id) <= Number.MAX_SAFE_INTEGER;
```

#### Do

```js
const ok = Number.isSafeInteger(id);
```


### JSX

#### Don't

```jsx
const ok = Number.isInteger(id) && Math.abs(id) <= Number.MAX_SAFE_INTEGER;
```

#### Do

```jsx
const ok = Number.isSafeInteger(id);
```


### TypeScript

#### Don't

```ts
const ok = Number.isInteger(id) && Math.abs(id) <= Number.MAX_SAFE_INTEGER;
```

#### Do

```ts
const ok = Number.isSafeInteger(id);
```


### TSX

#### Don't

```tsx
const ok = Number.isInteger(id) && Math.abs(id) <= Number.MAX_SAFE_INTEGER;
```

#### Do

```tsx
const ok = Number.isSafeInteger(id);
```
