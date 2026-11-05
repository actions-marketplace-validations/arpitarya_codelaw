---
id: js/regexp-strict
title: "Regular expressions use strict syntax"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "auto for every form but a quantified lookahead, which is reported without an edit"
stability: preview
ported_from: [eslint-plugin-regexp/strict]
since: 0.5.0
---

A pattern uses no syntax only the legacy mode (Annex B) allows: a lone `]` or `{`, a useless escape like `\a`, an octal escape, a quantified lookahead.

**Why:** These forms are errors with the `u` flag and hide mistakes without it.

```codelaw
native: js/regexp-strict
```

## Examples

### JavaScript

#### Don't

```js
const re = /a]/;
```

#### Do

```js
const re = /a\]/;
```

### JSX

#### Don't

```jsx
const re = /a]/;
```

#### Do

```jsx
const re = /a\]/;
```

### TypeScript

#### Don't

```ts
const re = /a]/;
```

#### Do

```ts
const re = /a\]/;
```

### TSX

#### Don't

```tsx
const re = /a]/;
```

#### Do

```tsx
const re = /a\]/;
```
