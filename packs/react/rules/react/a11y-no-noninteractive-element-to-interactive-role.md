---
id: react/a11y-no-noninteractive-element-to-interactive-role
title: "Non-interactive elements keep their role"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the fix is the native control, which changes the markup"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-noninteractive-element-to-interactive-role]
since: 0.5.0
---

A non-interactive element (`<h1>`, `<p>`, `<img>`, `<main>`, …) is not given an interactive role such as `button` or `link`.

**Why:** The element looks usable to assistive technology but gets none of a native control's keyboard behaviour.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - any:
    - all:
      - has: {field: name, regex: '^(main|article|aside|nav|header|footer|h[1-6]|p|blockquote|figure|img|pre|dl|dt|dd|label|form|address)$'}
      - has: {kind: jsx_attribute, regex: '^role=["''](button|checkbox|combobox|gridcell|link|listbox|menu|menubar|menuitem|menuitemcheckbox|menuitemradio|option|radio|radiogroup|scrollbar|searchbox|slider|spinbutton|switch|tab|tablist|textbox|tree|treegrid|treeitem)["'']'}
    - all:
      - has: {field: name, regex: ^(ul|ol|li|table|td|fieldset)$}
      - has: {kind: jsx_attribute, regex: '^role=["''](button|checkbox|combobox|link|radio|searchbox|slider|spinbutton|switch|textbox)["'']'}
message: Use a native interactive element instead of giving a non-interactive one an interactive role
```

## How to fix

1. Replace the element with the native control (`<button>`, `<a href>`, `<input>`).

## Examples

### JavaScript

#### Don't

```js
const b = <h3 role="button" onClick={toggle}>Details</h3>;
```

#### Do

```js
const b = <h3><button type="button" onClick={toggle}>Details</button></h3>;
```

### JSX

#### Don't

```jsx
const b = <h3 role="button" onClick={toggle}>Details</h3>;
```

#### Do

```jsx
const b = <h3><button type="button" onClick={toggle}>Details</button></h3>;
```

### TSX

#### Don't

```tsx
const b = <h3 role="button" onClick={toggle}>Details</h3>;
```

#### Do

```tsx
const b = <h3><button type="button" onClick={toggle}>Details</button></h3>;
```
