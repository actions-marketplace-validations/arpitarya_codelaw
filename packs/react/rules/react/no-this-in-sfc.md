---
id: react/no-this-in-sfc
title: "No this in a function component"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: what `this` was meant to reach — props, state, a ref — is passed in or held in a hook"
stability: preview
ported_from: [eslint-plugin-react/no-this-in-sfc]
since: 0.5.0
---

A function component does not use `this`.

**Why:** A function component has no instance; `this` is `undefined` there, so `this.props` throws.

```codelaw
utils:
  comp-fn:
    any:
      - { kind: function_declaration, has: { field: name, regex: '^[A-Z]' } }
      - any: [{ kind: arrow_function }, { kind: function_expression }]
        inside: { kind: variable_declarator, has: { field: name, regex: '^[A-Z]' } }
match:
  kind: this
  inside:
    stopBy: { any: [{ kind: function_declaration }, { kind: function_expression }, { kind: method_definition }, { kind: class_body }, { kind: arrow_function, inside: { kind: variable_declarator, has: { field: name, regex: '^[A-Z]' } } }] }
    matches: comp-fn
message: "A function component has no `this`"
```

## How to fix

1. Read `props` from the function's parameter; keep state in `useState` and refs in `useRef`.

## Examples

### JavaScript

#### Don't

```js
function Hello(props) {
  return <p>{this.props.name}</p>;
}
```

#### Do

```js
function Hello(props) {
  return <p>{props.name}</p>;
}
```

### JSX

#### Don't

```jsx
function Hello(props) {
  return <p>{this.props.name}</p>;
}
```

#### Do

```jsx
function Hello(props) {
  return <p>{props.name}</p>;
}
```

### TSX

#### Don't

```tsx
function Hello(props) {
  return <p>{this.props.name}</p>;
}
```

#### Do

```tsx
function Hello(props) {
  return <p>{props.name}</p>;
}
```
