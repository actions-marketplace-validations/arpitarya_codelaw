---
id: react/a11y-no-noninteractive-tabindex
title: "Non-interactive elements are not in the tab order"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the element either needs an interactive role and behaviour, or no `tabIndex`"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-noninteractive-tabindex]
since: 0.5.0
---

A non-interactive element with no interactive role does not get a `tabIndex` of 0 or more.

**Why:** Tabbing to something that does nothing wastes keyboard users' time and confuses screen-reader users.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^[a-z]'}
  - not:
      has: {field: name, regex: ^(a|area|button|input|select|textarea|option|summary|details|iframe|embed|audio|video)$}
  - has: {kind: jsx_attribute, regex: '^tabIndex=(["'']|\{\s*)[0-9]'}
  - not:
      has: {kind: jsx_attribute, regex: '^role=["''](button|checkbox|combobox|gridcell|link|listbox|menu|menubar|menuitem|menuitemcheckbox|menuitemradio|option|radio|radiogroup|scrollbar|searchbox|slider|spinbutton|switch|tab|tablist|textbox|tree|treegrid|treeitem|tabpanel)["'']|^role=\{'}
message: Remove `tabIndex` from a non-interactive element
```

## How to fix

1. Remove `tabIndex`, or give the element an interactive role and keyboard behaviour.

## Examples

### JavaScript

#### Don't

```js
const a = <article tabIndex={0}>…</article>;
```

#### Do

```js
const a = <article>…</article>;
```

### JSX

#### Don't

```jsx
const a = <article tabIndex={0}>…</article>;
```

#### Do

```jsx
const a = <article>…</article>;
```

### TSX

#### Don't

```tsx
const a = <article tabIndex={0}>…</article>;
```

#### Do

```tsx
const a = <article>…</article>;
```
