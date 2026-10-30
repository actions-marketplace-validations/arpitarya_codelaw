---
id: react/a11y-aria-activedescendant-has-tabindex
title: "An element with aria-activedescendant is focusable"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "whether the element joins the tab order (`0`) or not (`-1`) is the author's choice"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/aria-activedescendant-has-tabindex]
since: 0.5.0
---

A non-interactive element with `aria-activedescendant` also has a `tabIndex`.

**Why:** `aria-activedescendant` points focus into the element's children; it does nothing if the element itself can never be focused.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^[a-z]'}
  - not:
      has: {field: name, regex: ^(a|area|button|input|select|textarea|option|summary|details|iframe|embed|audio|video)$}
  - has: {kind: jsx_attribute, regex: ^aria-activedescendant\b}
  - not:
      has: {kind: jsx_attribute, regex: ^tabIndex\b}
  - not:
      has: {kind: jsx_expression}
message: Add a `tabIndex` to an element that uses `aria-activedescendant`
```

## How to fix

1. Add `tabIndex={0}` (or `-1` if it is focused from code only).

## Examples

### JavaScript

#### Don't

```js
const l = <div aria-activedescendant={id} />;
```

#### Do

```js
const l = <div aria-activedescendant={id} tabIndex={0} />;
```

### JSX

#### Don't

```jsx
const l = <div aria-activedescendant={id} />;
```

#### Do

```jsx
const l = <div aria-activedescendant={id} tabIndex={0} />;
```

### TSX

#### Don't

```tsx
const l = <div aria-activedescendant={id} />;
```

#### Do

```tsx
const l = <div aria-activedescendant={id} tabIndex={0} />;
```
