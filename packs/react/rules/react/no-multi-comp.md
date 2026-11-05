---
id: react/no-multi-comp
title: "One component per file"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the component moves to its own file"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/no-multi-comp]
since: 0.5.0
---

A file defines one component. Every component after the first is reported.

**Why:** One component per file keeps files small and makes each component findable by its file name.

```codelaw
utils:
  comp:
    any:
      - { kind: function_declaration, all: [{ has: { field: name, regex: '^[A-Z]' } }, { has: { stopBy: end, any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }] } }] }
      - { kind: lexical_declaration, has: { kind: variable_declarator, all: [{ has: { field: name, regex: '^[A-Z]' } }], has: { field: value, any: [{ kind: arrow_function }, { kind: function_expression }], has: { stopBy: end, any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }] } } } }
      - { kind: class_declaration, has: { kind: class_heritage, regex: '\b(Pure)?Component\b' } }
  top-comp:
    any:
      - matches: comp
      - { kind: export_statement, has: { matches: comp } }
match:
  matches: top-comp
  inside: { kind: program }
  follows: { stopBy: end, matches: top-comp }
message: "Move this component to its own file"
```

## How to fix

1. Move the component to its own file and import it.

## Examples

### JavaScript

#### Don't

```js
export function List() {
  return <ul />;
}
export function Item() {
  return <li />;
}
```

#### Do

```js
export function List() {
  return <ul />;
}
```

### JSX

#### Don't

```jsx
export function List() {
  return <ul />;
}
export function Item() {
  return <li />;
}
```

#### Do

```jsx
export function List() {
  return <ul />;
}
```

### TSX

#### Don't

```tsx
export function List() {
  return <ul />;
}
export function Item() {
  return <li />;
}
```

#### Do

```tsx
export function List() {
  return <ul />;
}
```
