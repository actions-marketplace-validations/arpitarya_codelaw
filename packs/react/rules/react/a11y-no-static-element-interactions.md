---
id: react/a11y-no-static-element-interactions
title: "div and span have a role when they handle events"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the fix is a native control or a role with keyboard behaviour; which is the author's choice"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-static-element-interactions]
since: 0.5.0
---

A `<div>` or `<span>` with a click or key handler also has a `role`.

**Why:** Without a role, assistive technology announces nothing usable about the element its handler makes interactive.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^(div|span)$}
  - has: {kind: jsx_attribute, regex: ^(onClick|onMouseDown|onMouseUp|onKeyPress|onKeyDown|onKeyUp)=}
  - not:
      has: {kind: jsx_attribute, regex: '^role\b|^aria-hidden(=(["'']true|\{\s*true)|$)'}
  - not:
      has: {kind: jsx_expression}
message: Give this element a role, or use a `<button>`
```

## How to fix

1. Use a `<button>` (or link), or add a `role` with matching keyboard behaviour and a `tabIndex`.

## Examples

### JavaScript

#### Don't

```js
const d = <div onClick={close}>×</div>;
```

#### Do

```js
const d = <button type="button" aria-label="Close" onClick={close}>×</button>;
```

### JSX

#### Don't

```jsx
const d = <div onClick={close}>×</div>;
```

#### Do

```jsx
const d = <button type="button" aria-label="Close" onClick={close}>×</button>;
```

### TSX

#### Don't

```tsx
const d = <div onClick={close}>×</div>;
```

#### Do

```tsx
const d = <button type="button" aria-label="Close" onClick={close}>×</button>;
```

## Differences from the source

Only `div` and `span` are checked as static elements; the source also covers rarer elements with no implicit role.
