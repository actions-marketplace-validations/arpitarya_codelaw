---
id: react/no-deriving-state-in-effects
title: "No effect that only copies a value into state"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the derived value is computed during render and the state and the effect are deleted"
stability: preview
ported_from: [eslint-plugin-react-hooks/no-deriving-state-in-effects]
since: 0.5.0
---

A `useEffect` whose body does nothing but call setters — at least one of them from this component's `useState` — with plain values (not updater functions) (`setFullName(first + " " + last)`) is replaced by a value computed during render.

**Why:** Deriving state in an effect renders twice, once with the stale value, and keeps two sources of truth in step by hand.

```codelaw
match:
  kind: call_expression
  all:
    - has: { field: function, regex: '^(React\.)?useEffect$' }
    - inside:
        stopBy: end
        kind: statement_block
        has:
          kind: lexical_declaration
          has:
            kind: variable_declarator
            all:
              - has: { field: name, kind: array_pattern, has: { nthChild: 2, kind: identifier, pattern: $S } }
              - has: { field: value, kind: call_expression, has: { field: function, regex: '^(React\.)?useState$' } }
    - has: { field: arguments, has: { nthChild: 1, has: { field: body, has: { kind: expression_statement, has: { kind: call_expression, has: { field: function, pattern: $S } } } } } }
    - not: { has: { field: arguments, has: { nthChild: 1, has: { field: body, has: { kind: expression_statement, has: { kind: call_expression, has: { field: arguments, has: { any: [{ kind: arrow_function }, { kind: function_expression }] } } } } } } } }
  has:
    field: arguments
    has:
      nthChild: 1
      any: [{ kind: arrow_function }, { kind: function_expression }]
      has:
        field: body
        kind: statement_block
        all:
          - has: { kind: expression_statement }
          - not: { has: { kind: expression_statement, not: { has: { kind: call_expression, has: { field: function, kind: identifier, regex: '^set[A-Z]' } } } } }
          - not: { has: { any: [{ kind: lexical_declaration }, { kind: variable_declaration }, { kind: if_statement }, { kind: return_statement }, { kind: for_statement }, { kind: for_in_statement }, { kind: while_statement }, { kind: try_statement }, { kind: switch_statement }, { kind: function_declaration }] } }
message: "This effect only derives state; compute the value during render"
```

## How to fix

1. Delete the state and the effect.
2. Compute the value in the component body: `const fullName = first + " " + last;` (with `useMemo` if it is expensive).

## Examples

### JavaScript

#### Don't

```js
function Name({ first, last }) {
  const [full, setFull] = useState("");
  useEffect(() => {
    setFull(first + " " + last);
  }, [first, last]);
  return full;
}
```

#### Do

```js
function Name({ first, last }) {
  const full = first + " " + last;
  return full;
}
```

### JSX

#### Don't

```jsx
function Name({ first, last }) {
  const [full, setFull] = useState("");
  useEffect(() => {
    setFull(first + " " + last);
  }, [first, last]);
  return full;
}
```

#### Do

```jsx
function Name({ first, last }) {
  const full = first + " " + last;
  return full;
}
```

### TSX

#### Don't

```tsx
function Name({ first, last }) {
  const [full, setFull] = useState("");
  useEffect(() => {
    setFull(first + " " + last);
  }, [first, last]);
  return full;
}
```

#### Do

```tsx
function Name({ first, last }) {
  const full = first + " " + last;
  return full;
}
```
