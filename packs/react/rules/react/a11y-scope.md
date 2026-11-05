---
id: react/a11y-scope
title: "scope only on th"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the cell may need to become a `<th>`, which changes the table's markup"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/scope]
since: 0.5.0
---

The `scope` attribute is used only on `<th>`.

**Why:** Elsewhere it is invalid and ignored; on a `<td>` it suggests a header that screen readers will not find.

```codelaw
match:
  kind: jsx_attribute
  regex: '^scope\b'
  inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^[a-z]' }, not: { has: { field: name, regex: '^th$' } } }
message: "`scope` belongs on `<th>` only"
```

## How to fix

1. Make the cell a `<th>`, or remove `scope`.

## Examples

### JavaScript

#### Don't

```js
const c = <td scope="row">Total</td>;
```

#### Do

```js
const c = <th scope="row">Total</th>;
```

### JSX

#### Don't

```jsx
const c = <td scope="row">Total</td>;
```

#### Do

```jsx
const c = <th scope="row">Total</th>;
```

### TSX

#### Don't

```tsx
const c = <td scope="row">Total</td>;
```

#### Do

```tsx
const c = <th scope="row">Total</th>;
```
