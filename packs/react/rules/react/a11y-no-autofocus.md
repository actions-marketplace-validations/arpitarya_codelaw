---
id: react/a11y-no-autofocus
title: "No autoFocus"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "some dialogs need initial focus, set deliberately in code"
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
```

## How to fix

1. Remove the attribute; focus a dialog's first field from its open handler instead.

## Examples

### JavaScript

#### Don't

```js
const q = <input autoFocus />;
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

#### Do

```jsx
const q = <input />;
```

### TSX

#### Don't

```tsx
const q = <input autoFocus />;
```

#### Do

```tsx
const q = <input />;
```
