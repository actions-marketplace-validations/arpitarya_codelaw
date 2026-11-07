---
id: react/a11y-prefer-tag-over-role
title: "The native element, not a role"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "swapping the element changes markup and styling"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsx-a11y/prefer-tag-over-role]
since: 0.5.0
---

A `<div>` or `<span>` is not given a role a native element already has (`role="button"` → `<button>`).

**Why:** The native element brings keyboard handling, focus and states for free; a role brings only the announcement.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^(div|span)$}
  - has: {kind: jsx_attribute, regex: '^role=["''](button|checkbox|radio|link|heading|img|list|listitem|navigation|main|banner|contentinfo|complementary|table|row|cell|columnheader|rowheader|textbox|separator|dialog|form|article|region|progressbar|meter|option|listbox|combobox)["'']$'}
message: Use the native element instead of `role` on a `div` or `span`
```

## How to fix

1. Use the native element (`<button>`, `<a href>`, `<nav>`, `<h2>`, …) and drop the role.

## Examples

### JavaScript

#### Don't

```js
const b = <div role="button" tabIndex={0} onClick={go}>Go</div>;
```

#### Do

```js
const b = <button type="button" onClick={go}>Go</button>;
```

### JSX

#### Don't

```jsx
const b = <div role="button" tabIndex={0} onClick={go}>Go</div>;
```

#### Do

```jsx
const b = <button type="button" onClick={go}>Go</button>;
```

### TSX

#### Don't

```tsx
const b = <div role="button" tabIndex={0} onClick={go}>Go</div>;
```

#### Do

```tsx
const b = <button type="button" onClick={go}>Go</button>;
```
