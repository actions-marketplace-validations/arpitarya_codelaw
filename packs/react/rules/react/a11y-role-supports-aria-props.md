---
id: react/a11y-role-supports-aria-props
title: "ARIA attributes fit the element's role"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "either the role or the attribute is wrong; the reviewer picks removing the attribute or changing the role to one that supports it"
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
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-checked\b, pattern: $A_CHECKED}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_CHECKED}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](checkbox|menuitemcheckbox|menuitemradio|option|radio|switch)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-pressed\b, pattern: $A_PRESSED}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_PRESSED}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](button)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-selected\b, pattern: $A_SELECTED}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_SELECTED}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](columnheader|gridcell|option|row|rowheader|tab|treeitem)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-valuenow\b, pattern: $A_VALUENOW}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_VALUENOW}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](meter|progressbar|scrollbar|separator|slider|spinbutton)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-valuemin\b, pattern: $A_VALUEMIN}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_VALUEMIN}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](meter|progressbar|scrollbar|separator|slider|spinbutton)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-valuemax\b, pattern: $A_VALUEMAX}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_VALUEMAX}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](meter|progressbar|scrollbar|separator|slider|spinbutton)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-level\b, pattern: $A_LEVEL}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_LEVEL}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](heading|listitem|row|comment|treeitem|treegrid|grid)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-multiline\b, pattern: $A_MULTILINE}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_MULTILINE}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](textbox)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-placeholder\b, pattern: $A_PLACEHOLDER}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_PLACEHOLDER}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](textbox|searchbox)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-sort\b, pattern: $A_SORT}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_SORT}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](columnheader|rowheader)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-autocomplete\b, pattern: $A_AUTOCOMPLETE}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_AUTOCOMPLETE}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](combobox|searchbox|textbox)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-modal\b, pattern: $A_MODAL}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_MODAL}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](alertdialog|dialog)["'']'}
    - all:
      - has: {kind: jsx_attribute, regex: ^aria-multiselectable\b, pattern: $A_MULTISELECTABLE}
      - has: {kind: jsx_attribute, regex: '^role=["''][a-z]+["'']$', pattern: $R_MULTISELECTABLE}
      - not:
          has: {kind: jsx_attribute, regex: '^role=["''](grid|listbox|tablist|tree|treegrid)["'']'}
message: The element's role does not support one of its `aria-*` attributes
fix:
  choices:
    - label: remove the attribute
      fix:
      - remove: $A_CHECKED
      - remove: $A_PRESSED
      - remove: $A_SELECTED
      - remove: $A_VALUENOW
      - remove: $A_VALUEMIN
      - remove: $A_VALUEMAX
      - remove: $A_LEVEL
      - remove: $A_MULTILINE
      - remove: $A_PLACEHOLDER
      - remove: $A_SORT
      - remove: $A_AUTOCOMPLETE
      - remove: $A_MODAL
      - remove: $A_MULTISELECTABLE
    - label: change the role
      fix:
      - replace: {target: $R_CHECKED, with: 'role="checkbox"'}
      - replace: {target: $R_PRESSED, with: 'role="button"'}
      - replace: {target: $R_SELECTED, with: 'role="option"'}
      - replace: {target: $R_VALUENOW, with: 'role="slider"'}
      - replace: {target: $R_VALUEMIN, with: 'role="slider"'}
      - replace: {target: $R_VALUEMAX, with: 'role="slider"'}
      - replace: {target: $R_LEVEL, with: 'role="heading"'}
      - replace: {target: $R_MULTILINE, with: 'role="textbox"'}
      - replace: {target: $R_PLACEHOLDER, with: 'role="textbox"'}
      - replace: {target: $R_SORT, with: 'role="columnheader"'}
      - replace: {target: $R_AUTOCOMPLETE, with: 'role="combobox"'}
      - replace: {target: $R_MODAL, with: 'role="dialog"'}
      - replace: {target: $R_MULTISELECTABLE, with: 'role="listbox"'}
```

## How to fix

1. Use the attribute the role supports (`aria-pressed` on a `button`, `aria-checked` on a `checkbox`), or change the role.

## Examples

### JavaScript

#### Don't

```js
const b = <div role="button" aria-checked="true" />;
```

#### Fixed (remove the attribute)

```js
const b = <div role="button"  />;
```

#### Fixed (change the role)

```js
const b = <div role="checkbox" aria-checked="true" />;
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

#### Fixed (remove the attribute)

```jsx
const b = <div role="button"  />;
```

#### Fixed (change the role)

```jsx
const b = <div role="checkbox" aria-checked="true" />;
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

#### Fixed (remove the attribute)

```tsx
const b = <div role="button"  />;
```

#### Fixed (change the role)

```tsx
const b = <div role="checkbox" aria-checked="true" />;
```

#### Do

```tsx
const b = <div role="button" aria-pressed="true" />;
```

## Differences from the source

Only an explicit `role` written as a string, and the role-specific states listed in the rule, are checked; the source also reads each element's implicit role and every ARIA attribute.
