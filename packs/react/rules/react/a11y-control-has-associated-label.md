---
id: react/a11y-control-has-associated-label
title: "A button has a label"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the label is the author's"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/control-has-associated-label]
since: 0.5.0
---

A `<button>`, or an element with an interactive role, has text inside it or an `aria-label`, `aria-labelledby` or `title`.

**Why:** A control with no name is announced as "button" and nothing more.

```codelaw
utils:
  labelled: { has: { kind: jsx_attribute, regex: '^(aria-label|aria-labelledby|title)\b' } }
  control:
    any:
      - has: { field: name, regex: '^button$' }
      - has: { kind: jsx_attribute, regex: '^role=["''](button|checkbox|link|menuitem|option|radio|switch|tab)["'']' }
match:
  any:
    - kind: jsx_self_closing_element
      matches: control
      not: { any: [{ matches: labelled }, { has: { kind: jsx_expression } }] }
    - kind: jsx_element
      has: { field: open_tag, matches: control, not: { any: [{ matches: labelled }, { has: { kind: jsx_expression } }] } }
      not: { has: { any: [{kind: jsx_element}, {kind: jsx_self_closing_element}, {kind: jsx_expression}, {kind: jsx_text, regex: '\S'}] } }
message: "This control has no label; put text inside it or add an `aria-label`"
```

## How to fix

1. Put the control's text inside it, or add an `aria-label`.

## Examples

### JavaScript

#### Don't

```js
const b = <button onClick={close}></button>;
```

#### Do

```js
const b = <button onClick={close} aria-label="Close"></button>;
```

### JSX

#### Don't

```jsx
const b = <button onClick={close}></button>;
```

#### Do

```jsx
const b = <button onClick={close} aria-label="Close"></button>;
```

### TSX

#### Don't

```tsx
const b = <button onClick={close}></button>;
```

#### Do

```tsx
const b = <button onClick={close} aria-label="Close"></button>;
```
