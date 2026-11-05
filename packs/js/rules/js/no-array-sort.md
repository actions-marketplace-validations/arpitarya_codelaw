---
id: js/no-array-sort
title: "toSorted() rather than sort()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`toSorted()` leaves the original untouched, which code later reading it may rely on either way"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-sort]
since: 0.5.0
---

The result of `.sort()` is not used as a value; `.toSorted()` returns a sorted copy. A `.sort()` call on its own line is left alone.

**Why:** `.sort()` sorts the array in place and returns it, so code that uses the result also reorders the original.

```codelaw

match:
  pattern: $A.sort($$$X)
  not: { inside: { kind: expression_statement } }
message: "`.sort()` mutates `$A`; use `.toSorted()`"
fix: "$A.toSorted($$$X)"
```

## Examples

### JavaScript

#### Don't

```js
const names = users.sort(byName);
```

#### Do

```js
const names = users.toSorted(byName);
```

### JSX

#### Don't

```jsx
const names = users.sort(byName);
```

#### Do

```jsx
const names = users.toSorted(byName);
```

### TypeScript

#### Don't

```ts
const names = users.sort(byName);
```

#### Do

```ts
const names = users.toSorted(byName);
```

### TSX

#### Don't

```tsx
const names = users.sort(byName);
```

#### Do

```tsx
const names = users.toSorted(byName);
```
