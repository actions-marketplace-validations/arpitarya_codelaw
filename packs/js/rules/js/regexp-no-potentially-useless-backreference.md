---
id: js/regexp-no-potentially-useless-backreference
title: "No backreference to a group that may not take part"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the group should always take part is the author's decision"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-regexp/no-potentially-useless-backreference]
since: 0.5.0
---

A backreference doesn't refer to a group that is optional or in another alternative's branch: in `(a)?\1` the backreference matches the empty string when the group didn't match.

**Why:** It silently matches nothing.

```codelaw
native: js/regexp-no-potentially-useless-backreference
```

## How to fix

1. Make the group required, or move the backreference inside the same optional part.

## Examples

### JavaScript

#### Don't

```js
const re = /(a)?b\1/;
```

#### Do

```js
const re = /(a)b\1/;
```

### JSX

#### Don't

```jsx
const re = /(a)?b\1/;
```

#### Do

```jsx
const re = /(a)b\1/;
```

### TypeScript

#### Don't

```ts
const re = /(a)?b\1/;
```

#### Do

```ts
const re = /(a)b\1/;
```

### TSX

#### Don't

```tsx
const re = /(a)?b\1/;
```

#### Do

```tsx
const re = /(a)b\1/;
```
