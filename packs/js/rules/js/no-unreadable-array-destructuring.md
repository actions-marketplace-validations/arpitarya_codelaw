---
id: js/no-unreadable-array-destructuring
title: "No consecutive holes in array destructuring"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the index is the number of skipped places, computed from the pattern"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [eslint-plugin-unicorn/no-unreadable-array-destructuring]
since: 0.5.0
---

An array destructuring pattern does not skip two or more elements in a row.

**Why:** Counting commas to find which element is read is error-prone.

```codelaw

match:
  kind: array_pattern
  regex: ',\s*,'
message: "Several skipped elements are hard to count; read the element by index"
```

## How to fix

1. Read the element by index (`parts[2]`).

## Examples

### JavaScript

#### Don't

```js
const [, , third] = parts;
```

#### Do

```js
const third = parts[2];
```

### JSX

#### Don't

```jsx
const [, , third] = parts;
```

#### Do

```jsx
const third = parts[2];
```

### TypeScript

#### Don't

```ts
const [, , third] = parts;
```

#### Do

```ts
const third = parts[2];
```

### TSX

#### Don't

```tsx
const [, , third] = parts;
```

#### Do

```tsx
const third = parts[2];
```
