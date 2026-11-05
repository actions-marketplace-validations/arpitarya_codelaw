---
id: js/regexp-no-empty-group
title: "No empty group"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "auto for a non-capturing group (removing it changes nothing); a capturing one is reported without an edit, since its number is used"
stability: preview
ported_from: [eslint-plugin-regexp/no-empty-group]
since: 0.5.0
---

A group holds something: `(?:)` and `()` match only the empty string.

**Why:** An empty group is noise or a forgotten edit.

```codelaw
native: js/regexp-no-empty-group
```

## Examples

### JavaScript

#### Don't

```js
const re = /a(?:)b/;
```

#### Do

```js
const re = /ab/;
```

### JSX

#### Don't

```jsx
const re = /a(?:)b/;
```

#### Do

```jsx
const re = /ab/;
```

### TypeScript

#### Don't

```ts
const re = /a(?:)b/;
```

#### Do

```ts
const re = /ab/;
```

### TSX

#### Don't

```tsx
const re = /a(?:)b/;
```

#### Do

```tsx
const re = /ab/;
```
