---
id: react/a11y-role-supports-aria-props
title: "ARIA attributes fit the element's role"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "either the role or the attribute is wrong, and only the author knows which"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/role-supports-aria-props]
since: 0.5.0
---

An element with an explicit `role` written as a string uses only the role-specific `aria-*` state that role supports (`aria-checked` on a checkbox, not on a button).

**Why:** An attribute the role does not support is ignored, so the state it was meant to announce is never heard.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$'}
  - any:
    - has: {kind: jsx_attribute, regex: ^aria-checked\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](checkbox|menuitemcheckbox|menuitemradio|option|radio|switch)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-pressed\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](button)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-selected\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](columnheader|gridcell|option|row|rowheader|tab|treeitem)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-valuenow\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](meter|progressbar|scrollbar|separator|slider|spinbutton)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-valuemin\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](meter|progressbar|scrollbar|separator|slider|spinbutton)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-valuemax\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](meter|progressbar|scrollbar|separator|slider|spinbutton)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-level\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](heading|listitem|row|comment|treeitem|treegrid|grid)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-multiline\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](textbox)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-placeholder\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](textbox|searchbox)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-sort\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](columnheader|rowheader)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-autocomplete\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](combobox|searchbox|textbox)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-modal\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](alertdialog|dialog)["'']'}
    - has: {kind: jsx_attribute, regex: ^aria-multiselectable\b}
      not:
        has: {kind: jsx_attribute, regex: '^role=["''](grid|listbox|tablist|tree|treegrid)["'']'}
message: The element's role does not support one of its `aria-*` attributes
```

## How to fix

1. Use the attribute the role supports (`aria-pressed` on a `button`, `aria-checked` on a `checkbox`), or change the role.

## Examples

### JavaScript

#### Don't

```js
const b = <div role="button" aria-checked="true" />;
```

#### Do

```js
const b = <div role="button" aria-pressed="true" />;
```

### JSX

#### Don't

```jsx
const b = <div role="button" aria-checked="true" />;
```

#### Do

```jsx
const b = <div role="button" aria-pressed="true" />;
```

### TSX

#### Don't

```tsx
const b = <div role="button" aria-checked="true" />;
```

#### Do

```tsx
const b = <div role="button" aria-pressed="true" />;
```

## Differences from the source

Only an explicit `role` written as a string, and the role-specific states listed in the rule, are checked; the source also reads each element's implicit role and every ARIA attribute.
