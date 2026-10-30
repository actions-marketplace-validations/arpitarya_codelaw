---
id: react/a11y-interactive-supports-focus
title: "An element with an interactive role is focusable"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "whether to add `tabIndex` or switch to a native control is the author's call"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/interactive-supports-focus]
since: 0.5.0
---

A non-interactive DOM element given an interactive role and a handler also has a `tabIndex`.

**Why:** A `div` with `role="button"` and an `onClick` cannot be reached with the Tab key unless it is focusable.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^[a-z]'}
  - not:
      has: {field: name, regex: ^(a|area|button|input|select|textarea|option|summary|details|iframe|embed|audio|video)$}
  - has: {kind: jsx_attribute, regex: '^role=["''](button|checkbox|combobox|gridcell|link|listbox|menu|menubar|menuitem|menuitemcheckbox|menuitemradio|option|radio|radiogroup|scrollbar|searchbox|slider|spinbutton|switch|tab|tablist|textbox|tree|treegrid|treeitem)["'']'}
  - has: {kind: jsx_attribute, regex: ^(onClick|onMouseDown|onMouseUp|onKeyPress|onKeyDown|onKeyUp)\b}
  - not:
      has: {kind: jsx_attribute, regex: ^(tabIndex|disabled|aria-disabled)\b}
  - not:
      has: {kind: jsx_expression}
message: This element has an interactive role but cannot be focused; add a `tabIndex`
```

## How to fix

1. Use the native element (`<button>`, `<a href>`), or add `tabIndex={0}`.

## Examples

### JavaScript

#### Don't

```js
const b = <span role="button" onClick={go}>Go</span>;
```

#### Do

```js
const b = <span role="button" tabIndex={0} onClick={go} onKeyDown={go}>Go</span>;
```

### JSX

#### Don't

```jsx
const b = <span role="button" onClick={go}>Go</span>;
```

#### Do

```jsx
const b = <span role="button" tabIndex={0} onClick={go} onKeyDown={go}>Go</span>;
```

### TSX

#### Don't

```tsx
const b = <span role="button" onClick={go}>Go</span>;
```

#### Do

```tsx
const b = <span role="button" tabIndex={0} onClick={go} onKeyDown={go}>Go</span>;
```
