---
id: react/a11y-role-has-required-aria-props
title: "A role carries the ARIA attributes it requires"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the attribute's value is the element's state, which the author wires up"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/role-has-required-aria-props]
since: 0.5.0
---

An element given a role that requires state (`checkbox`, `switch`, `radio`, `slider`, `heading`, …) carries those attributes.

**Why:** Without `aria-checked`, a custom checkbox is announced with no state; without `aria-valuenow`, a slider with no value.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^[a-z]'}
  - not:
      has: {field: name, regex: ^input$}
  - not:
      has: {kind: jsx_expression}
  - any:
    - has: {kind: jsx_attribute, regex: '^role=["''](checkbox|switch|radio|menuitemcheckbox|menuitemradio)["'']'}
      not:
        has: {kind: jsx_attribute, regex: ^aria-checked\b}
    - has: {kind: jsx_attribute, regex: '^role=["''](slider|scrollbar|meter)["'']'}
      not:
        has: {kind: jsx_attribute, regex: ^aria-valuenow\b}
    - has: {kind: jsx_attribute, regex: '^role=["'']combobox["'']'}
      not:
        has: {kind: jsx_attribute, regex: ^aria-expanded\b}
    - all:
      - has: {kind: jsx_attribute, regex: '^role=["'']heading["'']'}
      - not:
          has: {field: name, regex: '^h[1-6]$'}
      - not:
          has: {kind: jsx_attribute, regex: ^aria-level\b}
message: This role requires ARIA state the element does not carry
```

## How to fix

1. Add the required attribute (`aria-checked`, `aria-valuenow`, `aria-expanded` or `aria-level`) and keep it in step with the element's state.

## Examples

### JavaScript

#### Don't

```js
const c = <span role="checkbox" tabIndex={0} />;
```

#### Do

```js
const c = <span role="checkbox" aria-checked={on} tabIndex={0} />;
```

### JSX

#### Don't

```jsx
const c = <span role="checkbox" tabIndex={0} />;
```

#### Do

```jsx
const c = <span role="checkbox" aria-checked={on} tabIndex={0} />;
```

### TSX

#### Don't

```tsx
const c = <span role="checkbox" tabIndex={0} />;
```

#### Do

```tsx
const c = <span role="checkbox" aria-checked={on} tabIndex={0} />;
```
