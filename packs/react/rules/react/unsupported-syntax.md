---
id: react/unsupported-syntax
title: "No eval or with in components and hooks"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: what the evaluated code does is written as ordinary code"
stability: preview
ported_from: [eslint-plugin-react-hooks/unsupported-syntax]
since: 0.5.0
---

Components and hooks do not use `eval`, `new Function` or a `with` statement.

**Why:** Code built at run time can read and change any variable, so neither a reader nor the React Compiler can tell what a render depends on.

```codelaw
utils:
  comp-or-hook:
    any:
      - { kind: function_declaration, has: { field: name, regex: '^([A-Z]|use[A-Z])' } }
      - any: [{ kind: arrow_function }, { kind: function_expression }]
        inside: { kind: variable_declarator, has: { field: name, regex: '^([A-Z]|use[A-Z])' } }
match:
  any:
    - { kind: call_expression, has: { field: function, regex: '^eval$' } }
    - { kind: new_expression, has: { field: constructor, regex: '^Function$' } }
    - { kind: with_statement }
  inside: { stopBy: end, matches: comp-or-hook }
message: "React can't reason about `$$MATCH`; write the code out"
```

## How to fix

1. Replace the evaluated string or the `with` block with ordinary code.

## Examples

### JavaScript

#### Don't

```js
function Calc({ expr }) {
  return <b>{eval(expr)}</b>;
}
```

#### Do

```js
function Calc({ expr }) {
  return <b>{evaluate(expr)}</b>;
}
```

### JSX

#### Don't

```jsx
function Calc({ expr }) {
  return <b>{eval(expr)}</b>;
}
```

#### Do

```jsx
function Calc({ expr }) {
  return <b>{evaluate(expr)}</b>;
}
```

### TSX

#### Don't

```tsx
function Calc({ expr }) {
  return <b>{eval(expr)}</b>;
}
```

#### Do

```tsx
function Calc({ expr }) {
  return <b>{evaluate(expr)}</b>;
}
```
