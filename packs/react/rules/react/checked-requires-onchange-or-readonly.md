---
id: react/checked-requires-onchange-or-readonly
title: "A checked input has onChange or readOnly"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "`readOnly` says the input is meant to be fixed; if it should toggle, the author writes an `onChange` or uses `defaultChecked` instead"
stability: preview
ported_from: [eslint-plugin-react/checked-requires-onchange-or-readonly]
since: 0.5.0
---

An element with a `checked` prop also has `onChange` or `readOnly`, or uses `defaultChecked`.

**Why:** A `checked` with no `onChange` makes a controlled input the user cannot change, and React warns about it at run time.

```codelaw
match:
  kind: jsx_attribute
  regex: '^checked\b'
  inside:
    any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
    all:
      - has: { field: name, regex: '^[a-z]' }
      - not: { has: { kind: jsx_attribute, regex: '^(onChange|readOnly)\b' } }
      - not: { has: { kind: jsx_expression, has: { kind: spread_element } } }
message: "`checked` without `onChange` or `readOnly` makes an input the user can't change"
fix:
  - insert-after: " readOnly"
```

## Examples

### JavaScript

#### Don't

```js
const c = <input type="checkbox" checked={on} />;
```

#### Do

```js
const c = <input type="checkbox" checked={on} onChange={toggle} />;
```

#### Fixed

```js
const c = <input type="checkbox" checked={on} readOnly />;
```

### JSX

#### Don't

```jsx
const c = <input type="checkbox" checked={on} />;
```

#### Do

```jsx
const c = <input type="checkbox" checked={on} onChange={toggle} />;
```

#### Fixed

```jsx
const c = <input type="checkbox" checked={on} readOnly />;
```

### TSX

#### Don't

```tsx
const c = <input type="checkbox" checked={on} />;
```

#### Do

```tsx
const c = <input type="checkbox" checked={on} onChange={toggle} />;
```

#### Fixed

```tsx
const c = <input type="checkbox" checked={on} readOnly />;
```
