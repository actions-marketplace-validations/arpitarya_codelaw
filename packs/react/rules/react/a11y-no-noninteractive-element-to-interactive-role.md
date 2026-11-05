---
id: react/a11y-no-noninteractive-element-to-interactive-role
title: "Non-interactive elements keep their role"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "for role button the role is dropped and the onClick moves onto a new inner button, which changes markup, styling and focus order, so the reviewer confirms it; other roles get no edit"
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
  - any:
    - kind: jsx_opening_element
      has: {kind: jsx_attribute, regex: '^role=["'']button["'']', pattern: $R}
      all:
      - has: {kind: jsx_attribute, regex: ^onClick\b, pattern: $A}
      - inside:
          kind: jsx_element
          has: {field: close_tag, pattern: $CT}
    - {regex: '.'}
message: Use a native interactive element instead of giving a non-interactive one an interactive role
fix:
  - remove: $R
  - remove: $A
  - insert-after: { target: match, text: '<button type="button" $A>' }
  - insert-before: { target: $CT, text: "</button>" }
```

## How to fix

1. Replace the element with the native control (`<button>`, `<a href>`, `<input>`).

## Examples

### JavaScript

#### Don't

```js
const b = <h3 role="button" onClick={toggle}>Details</h3>;
```

#### Fixed

```js
const b = <h3  ><button type="button" onClick={toggle}>Details</button></h3>;
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

#### Fixed

```jsx
const b = <h3  ><button type="button" onClick={toggle}>Details</button></h3>;
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

#### Fixed

```tsx
const b = <h3  ><button type="button" onClick={toggle}>Details</button></h3>;
```

#### Do

```tsx
const b = <h3><button type="button" onClick={toggle}>Details</button></h3>;
```
