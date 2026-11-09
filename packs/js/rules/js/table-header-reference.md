---
id: js/table-header-reference
title: "A table cell's `headers` names a header of its own row or column"
languages: [jsx, tsx]
fix: manual
fix_reason: "which header a cell belongs to is the author's to say"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/table-header-reference]
since: 0.5.0
---

In a JSX `<table>` with a known layout (rows of literal `<td>`/`<th>` cells in `<thead>`, `<tbody>`, `<tfoot>` or a fragment, `colSpan` and `rowSpan` taken into account), each id in a cell's `headers` must be the `id` of a `<th>` in a column the cell covers or in its row. An id that is a `<th>` elsewhere in the table is reported as another column's or row's header; one that is no `<th>` at all is reported as unknown. A table with a component, an expression or a non-literal span among its rows is not known and is left alone. Behaviour of eslint-plugin-sonarjs 4.2.2 (read, not copied).

**Why:** Assistive technology reads a cell's headers from `headers`; one that points at the wrong header makes every announcement of that cell wrong.

```codelaw
native: js/table-header-reference
```

## How to fix

1. Point `headers` at the `id` of the `<th>` above the cell (and the one at the start of its row, where the table has row headers).
2. If the cell has no header, remove its `headers` attribute.

## Examples

### JSX

#### Don't

```jsx
const t = (
  <table>
    <thead>
      <tr><th id="name">Name</th><th id="age">Age</th></tr>
    </thead>
    <tbody>
      <tr><td headers="age">Ann</td><td headers="age">3</td></tr>
    </tbody>
  </table>
);
```

#### Do

```jsx
const t = (
  <table>
    <thead>
      <tr><th id="name">Name</th><th id="age">Age</th></tr>
    </thead>
    <tbody>
      <tr><td headers="name">Ann</td><td headers="age">3</td></tr>
    </tbody>
  </table>
);
```


### TSX

#### Don't

```tsx
const t = (
  <table>
    <thead>
      <tr><th id="name">Name</th><th id="age">Age</th></tr>
    </thead>
    <tbody>
      <tr><td headers="age">Ann</td><td headers="age">3</td></tr>
    </tbody>
  </table>
);
```

#### Do

```tsx
const t = (
  <table>
    <thead>
      <tr><th id="name">Name</th><th id="age">Age</th></tr>
    </thead>
    <tbody>
      <tr><td headers="name">Ann</td><td headers="age">3</td></tr>
    </tbody>
  </table>
);
```
