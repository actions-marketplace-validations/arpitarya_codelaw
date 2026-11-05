---
id: react/a11y-no-access-key
title: "No accessKey"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "removing the shortcut may need another way to reach the element"
fix_pending: "suggest"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-access-key]
since: 0.5.0
---

DOM elements carry no `accessKey`.

**Why:** Access keys clash with the shortcuts of screen readers and browsers, differently on every platform.

```codelaw
match:
  kind: jsx_attribute
  regex: '^accessKey\b'
  not: { regex: '^accessKey=\{\s*undefined\s*\}$' }
  inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^[a-z]' } }
message: "Remove `accessKey`; it clashes with assistive-technology shortcuts"
```

## How to fix

1. Remove the attribute.

## Examples

### JavaScript

#### Don't

```js
const s = <button accessKey="s">Save</button>;
```

#### Do

```js
const s = <button>Save</button>;
```

### JSX

#### Don't

```jsx
const s = <button accessKey="s">Save</button>;
```

#### Do

```jsx
const s = <button>Save</button>;
```

### TSX

#### Don't

```tsx
const s = <button accessKey="s">Save</button>;
```

#### Do

```tsx
const s = <button>Save</button>;
```
