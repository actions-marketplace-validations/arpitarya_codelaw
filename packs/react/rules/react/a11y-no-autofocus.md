---
id: react/a11y-no-autofocus
title: "No autoFocus"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "removing autoFocus drops initial focus, so the reviewer confirms the element needs none or moves focus from the open handler"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-autofocus]
since: 0.5.0
---

No element carries `autoFocus`.

**Why:** Moving focus on load skips content for screen-reader users and scrolls the page from under everyone else.

```codelaw
match:
  kind: jsx_attribute
  regex: '^autoFocus\b'
  not: { regex: '^autoFocus=\{\s*false\s*\}$' }
message: "Remove `autoFocus`; focus moved on load disorients keyboard and screen-reader users"
fix: [remove]
```

## How to fix

1. Remove the attribute; focus a dialog's first field from its open handler instead.

## Examples

### JavaScript

#### Don't

```js
const q = <input autoFocus />;
```

#### Fixed

```js
const q = <input  />;
```

#### Do

```js
const q = <input />;
```

### JSX

#### Don't

```jsx
const q = <input autoFocus />;
```

#### Fixed

```jsx
const q = <input  />;
```

#### Do

```jsx
const q = <input />;
```

### TSX

#### Don't

```tsx
const q = <input autoFocus />;
```

#### Fixed

```tsx
const q = <input  />;
```

#### Do

```tsx
const q = <input />;
```
