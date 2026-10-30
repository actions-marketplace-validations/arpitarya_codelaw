---
id: react/a11y-label-has-associated-control
title: "A label is tied to a control"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "which control the label belongs to is the author's"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/label-has-associated-control]
since: 0.5.0
---

A `<label>` has an `htmlFor`, or wraps the `<input>`, `<select>`, `<textarea>` or component it labels.

**Why:** An unconnected label is not announced with its field, and clicking it does not focus the field.

```codelaw
match:
  any:
  - kind: jsx_self_closing_element
    all:
    - has: {field: name, regex: ^label$}
    - not:
        has: {kind: jsx_attribute, regex: ^htmlFor\b}
    - not:
        has: {kind: jsx_expression}
  - kind: jsx_element
    has:
      field: open_tag
      has: {field: name, regex: ^label$}
      not:
        any:
        - has: {kind: jsx_attribute, regex: ^htmlFor\b}
        - has: {kind: jsx_expression}
    not:
      has:
        stopBy: end
        any:
        - {kind: jsx_expression}
        - any:
          - {kind: jsx_opening_element}
          - {kind: jsx_self_closing_element}
          has: {field: name, regex: '^(input|select|textarea|meter|output|progress|[A-Z].*|.*\..*)$'}
message: Tie this label to its control with `htmlFor`, or put the control inside it
```

## How to fix

1. Add `htmlFor` with the control's `id`, or move the control inside the `<label>`.

## Examples

### JavaScript

#### Don't

```js
const f = <><label>Name</label><input id="name" /></>;
```

#### Do

```js
const f = <><label htmlFor="name">Name</label><input id="name" /></>;
```

### JSX

#### Don't

```jsx
const f = <><label>Name</label><input id="name" /></>;
```

#### Do

```jsx
const f = <><label htmlFor="name">Name</label><input id="name" /></>;
```

### TSX

#### Don't

```tsx
const f = <><label>Name</label><input id="name" /></>;
```

#### Do

```tsx
const f = <><label htmlFor="name">Name</label><input id="name" /></>;
```

## Differences from the source

A component or an expression inside the label counts as its control; the source looks only for the components it is configured with, to a fixed depth.
