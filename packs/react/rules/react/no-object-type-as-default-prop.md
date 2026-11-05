---
id: react/no-object-type-as-default-prop
title: "No object or function literal as a prop default"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the default is hoisted to a module-level constant, whose name is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/no-object-type-as-default-prop]
since: 0.5.0
---

A component's destructured props have no default that is a new object, array or function (`function List({ items = [] })`).

**Why:** The default is a new value on every render, so a memoised child or an effect dependency sees a change each time.

```codelaw
utils:
  comp-fn:
    any:
      - { kind: function_declaration, has: { field: name, regex: '^[A-Z]' } }
      - any: [{ kind: arrow_function }, { kind: function_expression }]
        inside: { kind: variable_declarator, has: { field: name, regex: '^[A-Z]' } }
match:
  kind: object_assignment_pattern
  has: { field: right, any: [{ kind: object }, { kind: array }, { kind: arrow_function }, { kind: function_expression }, { kind: new_expression }, { kind: regex }] }
  inside:
    kind: object_pattern
    any:
      - inside: { kind: formal_parameters, inside: { matches: comp-fn } }
      - inside: { kind: required_parameter, inside: { kind: formal_parameters, inside: { matches: comp-fn } } }
message: "This default is a new value on every render; hoist it to a constant"
```

## How to fix

1. Define the default once at module level (`const NO_ITEMS = [];`) and use it: `{ items = NO_ITEMS }`.

## Examples

### JavaScript

#### Don't

```js
function List({ items = [] }) {
  return <ul>{items.length}</ul>;
}
```

#### Do

```js
const NO_ITEMS = [];
function List({ items = NO_ITEMS }) {
  return <ul>{items.length}</ul>;
}
```

### JSX

#### Don't

```jsx
function List({ items = [] }) {
  return <ul>{items.length}</ul>;
}
```

#### Do

```jsx
const NO_ITEMS = [];
function List({ items = NO_ITEMS }) {
  return <ul>{items.length}</ul>;
}
```

### TSX

#### Don't

```tsx
function List({ items = [] }) {
  return <ul>{items.length}</ul>;
}
```

#### Do

```tsx
const NO_ITEMS = [];
function List({ items = NO_ITEMS }) {
  return <ul>{items.length}</ul>;
}
```
