---
id: react/button-has-type
title: "A <button> says its type"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "a button with no type submits its form; `type=\"button\"` stops that, which is usually the intent but changes behaviour"
stability: preview
ported_from: [eslint-plugin-react/button-has-type]
since: 0.5.0
---

A `<button>` has an explicit `type`. A `type` with a value other than `button`, `submit` or `reset` is left to the DOM types for now.

**Why:** A button's default type is `submit`, so a button meant only for a click handler submits the form around it.

```codelaw
match:
  any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
  all:
    - has: { field: name, regex: '^button$', pattern: $N }
    - not: { has: { kind: jsx_attribute, regex: '^type\b' } }
    - not: { has: { kind: jsx_expression, has: { kind: spread_element } } }
message: "Give this `<button>` a `type` (`button`, `submit` or `reset`)"
fix:
  - insert-after: { target: $N, text: ' type="button"' }
```

## Examples

### JavaScript

#### Don't

```js
const b = <button onClick={save}>Save</button>;
```

#### Do

```js
const b = <button type="button" onClick={save}>Save</button>;
```

### JSX

#### Don't

```jsx
const b = <button onClick={save}>Save</button>;
```

#### Do

```jsx
const b = <button type="button" onClick={save}>Save</button>;
```

### TSX

#### Don't

```tsx
const b = <button onClick={save}>Save</button>;
```

#### Do

```tsx
const b = <button type="button" onClick={save}>Save</button>;
```
