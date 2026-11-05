---
id: react/function-component-definition
title: "Named components are function declarations"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the arrow is rewritten as a declaration, with its body, its type annotation and its export"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/function-component-definition]
since: 0.5.0
---

A named component is written as a function declaration (`function Card() {…}`), not as an arrow function or function expression assigned to a constant.

**Why:** One shape for components; a declaration is hoisted and shows its name in stack traces without inference.

```codelaw
match:
  kind: variable_declarator
  all:
    - has: { field: name, regex: '^[A-Z]', pattern: $N }
  has:
    field: value
    any: [{ kind: arrow_function }, { kind: function_expression }]
    has: { stopBy: end, any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }] }
message: "Declare `$N` with `function $N(…)`"
```

## How to fix

1. Rewrite `const $N = (props) => …` as `function $N(props) { return …; }`, keeping the export.

## Examples

### JavaScript

#### Don't

```js
const Card = ({ title }) => <h2>{title}</h2>;
```

#### Do

```js
function Card({ title }) {
  return <h2>{title}</h2>;
}
```

### JSX

#### Don't

```jsx
const Card = ({ title }) => <h2>{title}</h2>;
```

#### Do

```jsx
function Card({ title }) {
  return <h2>{title}</h2>;
}
```

### TSX

#### Don't

```tsx
const Card = ({ title }) => <h2>{title}</h2>;
```

#### Do

```tsx
function Card({ title }) {
  return <h2>{title}</h2>;
}
```
