---
id: react/a11y-label-has-associated-control
title: "A label is tied to a control"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "htmlFor ties the label to the next sibling control that has a static id, which the reviewer confirms is the intended one; a label with no such sibling gets no edit"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/label-has-associated-control]
since: 0.5.0
---

A `<label>` has an `htmlFor`, or wraps the `<input>`, `<select>`, `<textarea>` or component it labels.

**Why:** An unconnected label is not announced with its field, and clicking it does not focus the field.

```codelaw
match:
  all:
  - any:
    - kind: jsx_self_closing_element
      all:
      - has: {field: name, regex: ^label$, pattern: $N}
      - not:
          has: {kind: jsx_attribute, regex: ^htmlFor\b}
      - not:
          has: {kind: jsx_expression}
    - kind: jsx_element
      has:
        field: open_tag
        has: {field: name, regex: ^label$, pattern: $N}
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
  - any:
    - precedes:
        any:
        - {kind: jsx_self_closing_element}
        - {kind: jsx_opening_element}
        has: {kind: jsx_attribute, regex: '^id=["'']', has: {kind: string, pattern: $ID}}
    - {regex: '.'}
message: Tie this label to its control with `htmlFor`, or put the control inside it
fix:
  - insert-after: { target: $N, text: " htmlFor=$ID" }
```

## How to fix

1. Add `htmlFor` with the control's `id`, or move the control inside the `<label>`.

## Examples

### JavaScript

#### Don't

```js
const f = <><label>Name</label><input id="name" /></>;
```

#### Fixed

```js
const f = <><label htmlFor="name">Name</label><input id="name" /></>;
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

#### Fixed

```jsx
const f = <><label htmlFor="name">Name</label><input id="name" /></>;
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

#### Fixed

```tsx
const f = <><label htmlFor="name">Name</label><input id="name" /></>;
```

#### Do

```tsx
const f = <><label htmlFor="name">Name</label><input id="name" /></>;
```

## Differences from the source

A component or an expression inside the label counts as its control; the source looks only for the components it is configured with, to a fixed depth.
