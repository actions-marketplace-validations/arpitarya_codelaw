---
id: react/destructuring-assignment
title: "Props and state are destructured"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: a destructuring is written at the top of the component and every access is renamed, which a single edit cannot build"
stability: preview
ported_from: [eslint-plugin-react/destructuring-assignment]
since: 0.5.0
---

A component reads its props and state by destructuring them (`const { name } = props`), not as `props.name` or `this.props.name` at each use.

**Why:** Destructuring at the top lists in one place what the component reads.

```codelaw
utils:
  comp-fn:
    any:
      - { kind: function_declaration, has: { field: name, regex: '^[A-Z]' } }
      - any: [{ kind: arrow_function }, { kind: function_expression }]
        inside: { kind: variable_declarator, has: { field: name, regex: '^[A-Z]' } }
match:
  kind: member_expression
  any:
    - all:
        - regex: '^this\.(props|state)\.\w'
        - inside: { kind: member_expression, not: { inside: { kind: call_expression, field: function } } }
        - inside: { stopBy: end, kind: class_body }
    - all:
        - has: { field: object, kind: identifier, pattern: $P }
        - not: { inside: { kind: assignment_expression, field: left } }
        - inside:
            stopBy: end
            matches: comp-fn
            has: { field: parameters, has: { nthChild: 1, any: [{ kind: identifier, pattern: $P }, { kind: required_parameter, has: { field: pattern, kind: identifier, pattern: $P } }] } }
message: "Destructure the props instead of reading `$$MATCH`"
```

## How to fix

1. Write `const { … } = props;` (or `this.props`) at the top of the component, and use the names.

## Examples

### JavaScript

#### Don't

```js
function Hello(props) {
  return <p>{props.name}</p>;
}
```

#### Do

```js
function Hello({ name }) {
  return <p>{name}</p>;
}
```

### JSX

#### Don't

```jsx
function Hello(props) {
  return <p>{props.name}</p>;
}
```

#### Do

```jsx
function Hello({ name }) {
  return <p>{name}</p>;
}
```

### TSX

#### Don't

```tsx
function Hello(props) {
  return <p>{props.name}</p>;
}
```

#### Do

```tsx
function Hello({ name }) {
  return <p>{name}</p>;
}
```
