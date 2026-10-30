---
id: react/a11y-click-events-have-key-events
title: "A clickable element also answers the keyboard"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the right keys and behaviour depend on what the element does — usually a `<button>` is the better fix"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/click-events-have-key-events]
since: 0.5.0
---

A non-interactive DOM element with an `onClick` also has an `onKeyDown`, `onKeyUp` or `onKeyPress`.

**Why:** Keyboard users cannot click; without a key handler the action is out of their reach.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^[a-z]'}
  - not:
      has: {field: name, regex: ^(a|area|button|input|select|textarea|option|summary|details|iframe|embed|audio|video)$}
  - has: {kind: jsx_attribute, regex: ^onClick\b}
  - not:
      has: {kind: jsx_attribute, regex: '^(onKeyDown|onKeyUp|onKeyPress)\b|^aria-hidden(=(["''])true|=\{\s*true|$)|^role=["''](presentation|none)'}
  - not:
      has: {kind: jsx_expression}
message: Add a keyboard handler beside `onClick`, or use a `<button>`
```

## How to fix

1. Use a `<button>` (it handles Enter and Space itself), or add `onKeyDown` and a `tabIndex`.

## Examples

### JavaScript

#### Don't

```js
const c = <div onClick={open}>Open</div>;
```

#### Do

```js
const c = <button type="button" onClick={open}>Open</button>;
```

### JSX

#### Don't

```jsx
const c = <div onClick={open}>Open</div>;
```

#### Do

```jsx
const c = <button type="button" onClick={open}>Open</button>;
```

### TSX

#### Don't

```tsx
const c = <div onClick={open}>Open</div>;
```

#### Do

```tsx
const c = <button type="button" onClick={open}>Open</button>;
```
