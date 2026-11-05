---
id: js/no-empty-file
title: "No empty file"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to delete the file or fill it is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-empty-file]
since: 0.5.0
---

A file holds at least one statement; one with only comments or whitespace is reported.

**Why:** An empty file is a leftover or a placeholder that a build still picks up.

```codelaw

match:
  kind: program
  not: { has: { not: { kind: comment }, regex: "." } }
message: "This file has no code"
```

## How to fix

1. Delete the file, or write the code it was meant to hold.

## Examples

### JavaScript

#### Don't

```js
// nothing here yet
```

#### Do

```js
export const ready = true;
```

### JSX

#### Don't

```jsx
// nothing here yet
```

#### Do

```jsx
export const ready = true;
```

### TypeScript

#### Don't

```ts
// nothing here yet
```

#### Do

```ts
export const ready = true;
```

### TSX

#### Don't

```tsx
// nothing here yet
```

#### Do

```tsx
export const ready = true;
```
