---
id: js/regexp-no-useless-assertions
title: "No assertion that always succeeds or always fails"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing an assertion that always succeeds keeps what the pattern matches, but one that always fails needs the author"
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-assertions]
since: 0.5.0
---

An assertion's outcome isn't fixed by its neighbours: `a^` (without `m`) always fails; `a\b-` always succeeds.

**Why:** Such an assertion is a mistake or noise.

```codelaw
native: js/regexp-no-useless-assertions
```

## How to fix

1. Remove an assertion that always succeeds; rethink the pattern where one always fails.

## Examples

### JavaScript

#### Don't

```js
const re = /a\b-/;
```

#### Do

```js
const re = /a-/;
```

### JSX

#### Don't

```jsx
const re = /a\b-/;
```

#### Do

```jsx
const re = /a-/;
```

### TypeScript

#### Don't

```ts
const re = /a\b-/;
```

#### Do

```ts
const re = /a-/;
```

### TSX

#### Don't

```tsx
const re = /a\b-/;
```

#### Do

```tsx
const re = /a-/;
```
