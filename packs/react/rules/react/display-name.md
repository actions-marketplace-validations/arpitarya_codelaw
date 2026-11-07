---
id: react/display-name
title: "A component passed to memo or forwardRef has a name"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "a component held by a `const` at the top of the module gets `displayName` set to that name, which changes nothing but what DevTools and error messages show; one held anywhere else is named by hand"
stability: preview
ported_from: [eslint-plugin-react/display-name]
since: 0.1.0
---

A component wrapped in `memo` or `forwardRef` is a named function, so it has a display name.

**Why:** An anonymous component shows up as `Anonymous` in React DevTools and in error messages,
which makes both harder to read.

```codelaw
match:
  all:
    - any:
        - pattern: memo($F)
        - pattern: React.memo($F)
        - pattern: forwardRef($F)
        - pattern: React.forwardRef($F)
    - not:
        inside:
          kind: variable_declarator
          inside:
            kind: lexical_declaration
            any:
              - precedes: { kind: expression_statement, regex: '^\w+\.displayName\s*=' }
              - inside: { kind: export_statement, precedes: { kind: expression_statement, regex: '^\w+\.displayName\s*=' } }
    - any:
        - inside:
            kind: variable_declarator
            has: { field: name, kind: identifier, pattern: $NAME }
            inside:
              kind: lexical_declaration
              any:
                - { inside: { kind: program }, pattern: $STMT }
                - { inside: { kind: export_statement, inside: { kind: program }, pattern: $STMT } }
        - regex: ''
constraints:
  F:
    any:
      - kind: arrow_function
      - kind: function_expression
        not: { has: { field: name, pattern: $NAME } }
message: "This component has no name, so React DevTools and error messages show it as `Anonymous`"
fix:
  - insert-after: { target: $STMT, text: "$NAME.displayName = \"$NAME\";", line: true }
```

## How to fix

1. Pass a named function: `memo(function Card(props) { … })`.
2. Or set `Card.displayName = "Card"`.

## Examples

### JavaScript

#### Don't

```js
export const Card = memo((props) => <div>{props.title}</div>);
```

#### Do

```js
export const Card = memo((props) => <div>{props.title}</div>);
Card.displayName = "Card";
```

### JSX

#### Don't

```jsx
export const Card = memo((props) => <div>{props.title}</div>);
```

#### Do

```jsx
export const Card = memo((props) => <div>{props.title}</div>);
Card.displayName = "Card";
```

### TSX

#### Don't

```tsx
export const Card = memo((props) => <div>{props.title}</div>);
```

#### Do

```tsx
export const Card = memo((props) => <div>{props.title}</div>);
Card.displayName = "Card";
```

## Differences from the source

Components defined some other way (`createReactClass`, anonymous default exports) are not checked
yet.
