---
id: js/no-string-case-mismatch
title: "No comparison that case conversion makes impossible"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the literal's intended case is the author's to confirm"
fix_pending: "suggest"
stability: preview
ported_from: [biome/correctness/noStringCaseMismatch]
since: 0.5.0
---

The result of `toUpperCase()` is not compared with a string that holds lowercase letters, nor `toLowerCase()` with one that holds uppercase letters.

**Why:** The comparison can never be true, so the branch it guards never runs.

```codelaw
match:
  any:
    - pattern: $S.toUpperCase() === $L
    - pattern: $S.toUpperCase() == $L
    - pattern: $L === $S.toUpperCase()
    - pattern: $L == $S.toUpperCase()
    - pattern: $S.toUpperCase() !== $L
    - pattern: $S.toUpperCase() != $L
    - pattern: $S.toLowerCase() === $M
    - pattern: $S.toLowerCase() == $M
    - pattern: $M === $S.toLowerCase()
    - pattern: $M == $S.toLowerCase()
    - pattern: $S.toLowerCase() !== $M
    - pattern: $S.toLowerCase() != $M
constraints:
  L: { kind: string, regex: '[a-z]' }
  M: { kind: string, regex: '[A-Z]' }
message: "This comparison is always false (or always true): the string's case can never match"
```

## How to fix

1. Write the literal in the case the conversion produces (`'ADMIN'` after `toUpperCase()`).

## Examples

### JavaScript

#### Don't

```js
if (role.toUpperCase() === 'admin') grant();
```

#### Do

```js
if (role.toUpperCase() === 'ADMIN') grant();
```

### JSX

#### Don't

```jsx
if (role.toUpperCase() === 'admin') grant();
```

#### Do

```jsx
if (role.toUpperCase() === 'ADMIN') grant();
```

### TypeScript

#### Don't

```ts
if (role.toUpperCase() === 'admin') grant();
```

#### Do

```ts
if (role.toUpperCase() === 'ADMIN') grant();
```

### TSX

#### Don't

```tsx
if (role.toUpperCase() === 'admin') grant();
```

#### Do

```tsx
if (role.toUpperCase() === 'ADMIN') grant();
```
