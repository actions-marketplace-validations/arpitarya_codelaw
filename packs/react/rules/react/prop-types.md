---
id: react/prop-types
title: "Props are declared in propTypes"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "authored: the prop's type is the author's to declare"
stability: preview
ported_from: [eslint-plugin-react/prop-types]
since: 0.5.0
---

A component that reads a prop declares it: in `Component.propTypes`, or with a type on its props parameter (TypeScript). The rule reads destructured props of function components and `this.props.x` in class components, and checks a component only when its file assigns it no `propTypes` at all.

**Why:** Undeclared props are the component's interface written nowhere; a typo in a caller then passes silently.

```codelaw
utils:
  comp-params:
    any:
      - inside: { kind: function_declaration, has: { field: name, regex: '^[A-Z]', pattern: $C } }
      - inside: { any: [{ kind: arrow_function }, { kind: function_expression }], inside: { kind: variable_declarator, has: { field: name, regex: '^[A-Z]', pattern: $C } } }
match:
  any:
    - kind: shorthand_property_identifier_pattern
      all:
        - pattern: $K
        - inside:
            kind: object_pattern
            any:
              - { nthChild: 1, inside: { kind: formal_parameters, matches: comp-params } }
              - { inside: { kind: required_parameter, nthChild: 1, not: { has: { kind: type_annotation } }, inside: { kind: formal_parameters, matches: comp-params } } }
        - not: { inside: { stopBy: end, kind: program, has: { stopBy: end, kind: assignment_expression, has: { field: left, pattern: $C.propTypes } } } }
    - kind: member_expression
      regex: '^this\.props\.\w+$'
      all:
        - inside:
            stopBy: end
            kind: class_declaration
            has: { field: name, pattern: $C }
            all: [{ has: { kind: class_heritage, regex: '\b(React\.)?(Pure)?Component$' } }]
            not: { has: { field: body, has: { any: [{ kind: field_definition }, { kind: public_field_definition }], regex: '^static\s+propTypes\b' } } }
        - not: { inside: { stopBy: end, kind: program, has: { stopBy: end, kind: assignment_expression, has: { field: left, pattern: $C.propTypes } } } }
message: "This prop is not declared in `propTypes`"
```

## How to fix

1. Declare the component's props: `Name.propTypes = { prop: PropTypes.string }`, or type the props parameter in TypeScript.

## Examples

### JavaScript

#### Don't

```js
function Hello({ name }) {
  return <p>{name}</p>;
}
```

#### Do

```js
function Hello({ name }) {
  return <p>{name}</p>;
}
Hello.propTypes = { name: PropTypes.string };
```

### JSX

#### Don't

```jsx
function Hello({ name }) {
  return <p>{name}</p>;
}
```

#### Do

```jsx
function Hello({ name }) {
  return <p>{name}</p>;
}
Hello.propTypes = { name: PropTypes.string };
```

### TSX

#### Don't

```tsx
function Hello({ name }) {
  return <p>{name}</p>;
}
```

#### Do

```tsx
function Hello({ name }) {
  return <p>{name}</p>;
}
Hello.propTypes = { name: PropTypes.string };
```
