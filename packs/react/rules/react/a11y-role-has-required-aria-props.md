---
id: react/a11y-role-has-required-aria-props
title: "A role carries the ARIA attributes it requires"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "a non-heading element given role=heading gets aria-level=\"2\", ARIA's default level; the other roles need the element's live state, so they carry no edit"
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
      - has: {kind: jsx_attribute, regex: '^role=["'']heading["'']', pattern: $ROLE}
      - not:
          has: {field: name, regex: '^h[1-6]$'}
      - not:
          has: {kind: jsx_attribute, regex: ^aria-level\b}
message: This role requires ARIA state the element does not carry
fix:
  - insert-after: {target: $ROLE, text: ' aria-level="2"'}
auto_when:
  has: {kind: jsx_attribute, regex: '^role=["'']heading["'']'}
```

## How to fix

1. A `role="heading"` gets `aria-level="2"` from the fix. For the other roles, add the required attribute (`aria-checked`, `aria-valuenow`, `aria-expanded` or `aria-level`) and keep it in step with the element's state.

## Examples

### JavaScript

#### Don't

```js
const c = <div role="heading">Title</div>;
```

#### Fixed

```js
const c = <div role="heading" aria-level="2">Title</div>;
```

#### Do

```js
const c = <div role="heading" aria-level={level}>Title</div>;
```

### JSX

#### Don't

```jsx
const c = <div role="heading">Title</div>;
```

#### Fixed

```jsx
const c = <div role="heading" aria-level="2">Title</div>;
```

#### Do

```jsx
const c = <div role="heading" aria-level={level}>Title</div>;
```

### TSX

#### Don't

```tsx
const c = <div role="heading">Title</div>;
```

#### Fixed

```tsx
const c = <div role="heading" aria-level="2">Title</div>;
```

#### Do

```tsx
const c = <div role="heading" aria-level={level}>Title</div>;
```
