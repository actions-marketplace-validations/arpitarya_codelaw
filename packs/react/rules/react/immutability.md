---
id: react/immutability
title: "Props and state are never mutated"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the change becomes a new value — a copy passed to the state setter, or a callback to the parent that owns the prop"
stability: preview
ported_from: [eslint-plugin-react-hooks/immutability]
since: 0.5.0
---

A component or hook does not assign to, or call a mutating method (`push`, `splice`, `sort`, …) on, its props or a value it got from `useState` or `useReducer`.

**Why:** React compares by reference: a mutated value looks unchanged, so the screen doesn't update, and memoised values go stale.

```codelaw
utils:
  comp-or-hook:
    any:
      - { kind: function_declaration, has: { field: name, regex: '^([A-Z]|use[A-Z])' } }
      - any: [{ kind: arrow_function }, { kind: function_expression }]
        inside: { kind: variable_declarator, has: { field: name, regex: '^([A-Z]|use[A-Z])' } }
  owned-name:
    any:
      - inside:
          stopBy: end
          matches: comp-or-hook
          has:
            field: parameters
            has:
              nthChild: 1
              any:
                - { kind: identifier, pattern: $R }
                - { kind: object_pattern, has: { kind: shorthand_property_identifier_pattern, pattern: $R } }
                - { kind: required_parameter, has: { field: pattern, any: [{ kind: identifier, pattern: $R }, { kind: object_pattern, has: { kind: shorthand_property_identifier_pattern, pattern: $R } }] } }
      - inside:
          stopBy: end
          matches: comp-or-hook
          has:
            stopBy: end
            kind: variable_declarator
            all:
              - has: { field: name, kind: array_pattern, has: { nthChild: 1, kind: identifier, pattern: $R } }
              - has: { field: value, kind: call_expression, has: { field: function, regex: '^(React\.)?(useState|useReducer)$' } }
match:
  any:
    - kind: assignment_expression
      has: { field: left, any: [{ kind: member_expression }, { kind: subscript_expression }], has: { stopBy: end, field: object, kind: identifier, pattern: $R } }
    - kind: augmented_assignment_expression
      has: { field: left, any: [{ kind: member_expression }, { kind: subscript_expression }], has: { stopBy: end, field: object, kind: identifier, pattern: $R } }
    - kind: call_expression
      has: { field: function, kind: member_expression, regex: '\.(push|pop|shift|unshift|splice|sort|reverse|fill|copyWithin)$', has: { field: object, kind: identifier, pattern: $R } }
  matches: owned-name
message: "`$R` belongs to React; don't mutate it"
```

## How to fix

1. Build a new value (`[...items, item]`, `{ ...user, name }`) and pass it to the state setter.
2. For a prop, call a callback the parent passes so the owner makes the change.

## Examples

### JavaScript

#### Don't

```js
function List({ items }) {
  items.push("x");
  return <ul>{items.length}</ul>;
}
```

#### Do

```js
function List({ items }) {
  const all = [...items, "x"];
  return <ul>{all.length}</ul>;
}
```

### JSX

#### Don't

```jsx
function List({ items }) {
  items.push("x");
  return <ul>{items.length}</ul>;
}
```

#### Do

```jsx
function List({ items }) {
  const all = [...items, "x"];
  return <ul>{all.length}</ul>;
}
```

### TSX

#### Don't

```tsx
function List({ items }) {
  items.push("x");
  return <ul>{items.length}</ul>;
}
```

#### Do

```tsx
function List({ items }) {
  const all = [...items, "x"];
  return <ul>{all.length}</ul>;
}
```
