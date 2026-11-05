---
id: js/no-multi-str
title: "No multiline strings with a backslash"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a template literal keeps the indentation of the next line, so the author chooses the form"
fix_pending: "auto; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint/no-multi-str]
since: 0.5.0
---

A string is not continued onto the next line with a trailing backslash.

**Why:** The backslash-newline is easy to break with trailing whitespace; a template literal or `+` says the same clearly.

```codelaw
match:
  kind: string
  regex: '\\\r?\n'
message: "Use a template literal or `+` instead of a backslash at the end of the line"
```

## How to fix

1. Use a template literal, or join the parts with `+`.

## Examples

### JavaScript

#### Don't

```js
const s = 'one \
two';
```

#### Do

```js
const s = 'one ' +
  'two';
```

### JSX

#### Don't

```jsx
const s = 'one \
two';
```

#### Do

```jsx
const s = 'one ' +
  'two';
```

### TypeScript

#### Don't

```ts
const s = 'one \
two';
```

#### Do

```ts
const s = 'one ' +
  'two';
```

### TSX

#### Don't

```tsx
const s = 'one \
two';
```

#### Do

```tsx
const s = 'one ' +
  'two';
```
