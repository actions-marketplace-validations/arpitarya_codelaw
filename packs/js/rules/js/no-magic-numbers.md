---
id: js/no-magic-numbers
title: "No unnamed number in an expression"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the constant's name is the author's"
fix_exception: value
stability: preview
ported_from: [eslint/no-magic-numbers]
since: 0.5.0
---

A number literal is not used in an expression where its meaning is unnamed; it is given a name with a variable first. A number that initialises a variable, is an object property's value or a property assignment, is `parseInt`'s radix, or is a TypeScript literal type or enum value is left alone, as ESLint's defaults leave them.

**Why:** `timeout * 60000` makes the reader guess; `timeout * MS_PER_MINUTE` doesn't.

```codelaw
utils:
  definition:
    any:
      - kind: variable_declarator
      - kind: pair
      - kind: enum_assignment
      - kind: assignment_expression
        has: { field: left, kind: member_expression }
match:
  kind: number
  not:
    any:
      - inside: { matches: definition }
      - inside: { kind: unary_expression, inside: { matches: definition } }
      - inside: { kind: literal_type, stopBy: end }
      - inside: { kind: enum_body, stopBy: end }
      - inside:
          kind: arguments
          inside: { kind: call_expression, has: { field: function, regex: '^(Number\.)?parseInt$' } }
message: "Name the number `$$MATCH` with a constant"
```

## How to fix

1. Declare a `const` with a name that says what the number means, and use the name here.

## Examples

### JavaScript

#### Don't

```js
const ms = minutes * 60000;
```

#### Do

```js
const MS_PER_MINUTE = 60000;
const ms = minutes * MS_PER_MINUTE;
```


### JSX

#### Don't

```jsx
const ms = minutes * 60000;
```

#### Do

```jsx
const MS_PER_MINUTE = 60000;
const ms = minutes * MS_PER_MINUTE;
```


### TypeScript

#### Don't

```ts
const ms = minutes * 60000;
```

#### Do

```ts
const MS_PER_MINUTE = 60000;
const ms = minutes * MS_PER_MINUTE;
```


### TSX

#### Don't

```tsx
const ms = minutes * 60000;
```

#### Do

```tsx
const MS_PER_MINUTE = 60000;
const ms = minutes * MS_PER_MINUTE;
```
