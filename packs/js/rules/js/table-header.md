---
id: js/table-header
title: "A table has header cells"
languages: [jsx, tsx]
fix: manual
fix_reason: "which cells are headers is the author's"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/table-header]
since: 0.5.0
---

A JSX `<table>` that has rows has at least one `<th>` (or a cell with `role="columnheader"` or `"rowheader"`).

**Why:** Screen readers announce each cell with its header; without one a table is a stream of values.

```codelaw
match:
  kind: jsx_element
  has: { field: open_tag, has: { field: name, regex: '^table$' } }
  all:
    - has: { stopBy: end, kind: jsx_opening_element, has: { field: name, regex: '^tr$' } }
  not:
    has:
      stopBy: end
      any:
        - { kind: jsx_opening_element, has: { field: name, regex: '^th$' } }
        - { kind: jsx_self_closing_element, has: { field: name, regex: '^th$' } }
        - { kind: jsx_attribute, regex: '^role=["''](columnheader|rowheader)["'']$' }
message: "Mark the header cells with `<th>`"
```

## How to fix

1. Use `<th>` (with `scope` where it helps) for the cells that label a column or a row.

## Examples

### JSX

#### Don't

```jsx
const t = (
  <table>
    <tbody>
      <tr><td>Name</td><td>Age</td></tr>
    </tbody>
  </table>
);
```

#### Do

```jsx
const t = (
  <table>
    <thead>
      <tr><th>Name</th><th>Age</th></tr>
    </thead>
  </table>
);
```


### TSX

#### Don't

```tsx
const t = (
  <table>
    <tbody>
      <tr><td>Name</td><td>Age</td></tr>
    </tbody>
  </table>
);
```

#### Do

```tsx
const t = (
  <table>
    <thead>
      <tr><th>Name</th><th>Age</th></tr>
    </thead>
  </table>
);
```
