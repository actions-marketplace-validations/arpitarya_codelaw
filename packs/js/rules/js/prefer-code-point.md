---
id: js/prefer-code-point
title: "codePointAt rather than charCodeAt"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "for an astral character the result differs"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-code-point]
since: 0.5.0
---

`charCodeAt` and `String.fromCharCode` are written `codePointAt` and `String.fromCodePoint`.

**Why:** The code-point methods handle characters outside the Basic Multilingual Plane.

```codelaw
match:
  pattern: "$S.charCodeAt($$$A)"
message: "Use `.codePointAt()`"
fix: "$S.codePointAt($$$A)"
```

## Examples

### JavaScript

#### Don't

```js
const c = text.charCodeAt(0);
```

#### Do

```js
const c = text.codePointAt(0);
```

### JSX

#### Don't

```jsx
const c = text.charCodeAt(0);
```

#### Do

```jsx
const c = text.codePointAt(0);
```

### TypeScript

#### Don't

```ts
const c = text.charCodeAt(0);
```

#### Do

```ts
const c = text.codePointAt(0);
```

### TSX

#### Don't

```tsx
const c = text.charCodeAt(0);
```

#### Do

```tsx
const c = text.codePointAt(0);
```
