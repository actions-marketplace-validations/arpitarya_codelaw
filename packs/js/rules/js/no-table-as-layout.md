---
id: js/no-table-as-layout
title: "No table for layout"
languages: [jsx, tsx]
fix: manual
fix_reason: "the CSS layout is the author's to write"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-table-as-layout]
since: 0.5.0
---

A JSX `<table>` is not marked `role="presentation"` or `role="none"` (a layout table).

**Why:** CSS grid and flexbox lay out content without telling screen readers it is tabular data.

```codelaw
match:
  kind: jsx_opening_element
  has:
    field: name
    regex: ^table$
  all:
  - has:
      kind: jsx_attribute
      regex: ^role=\{?.(presentation|none).
message: A table used for layout confuses assistive technology; lay out with CSS
```

## How to fix

1. Replace the table with elements laid out by CSS grid or flexbox.

## Examples

### JSX

#### Don't

```jsx
const v = <table role="presentation"><tbody><tr><td>a</td></tr></tbody></table>;
```

#### Do

```jsx
const v = <div className="grid">a</div>;
```

### TSX

#### Don't

```tsx
const v = <table role="presentation"><tbody><tr><td>a</td></tr></tbody></table>;
```

#### Do

```tsx
const v = <div className="grid">a</div>;
```
