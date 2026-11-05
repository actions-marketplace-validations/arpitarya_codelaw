---
id: js/prefer-template
title: "Template literal rather than string concatenation"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "escaping the parts into one template literal is a computed edit"
fix_pending: "auto; needs computed-edit"
stability: preview
ported_from: [eslint/prefer-template]
since: 0.5.0
---

A string is not built by adding a string literal and another value with `+`.

**Why:** A template literal shows the shape of the result at a glance.

```codelaw
match:
  kind: binary_expression
  has:
    field: operator
    regex: ^\+$
  not:
    inside:
      kind: binary_expression
      has:
        field: operator
        regex: ^\+$
  all:
  - has:
      kind: string
  - has:
      not:
        any:
        - kind: string
        - kind: template_string
        - kind: binary_expression
message: Build the string with a template literal
```

## How to fix

1. Write one template literal: each string part as text, each other operand as `${…}`.

## Examples

### JavaScript

#### Don't

```js
const greeting = "Hello, " + name;
```

#### Do

```js
const greeting = `Hello, ${name}`;
```

### JSX

#### Don't

```jsx
const greeting = "Hello, " + name;
```

#### Do

```jsx
const greeting = `Hello, ${name}`;
```

### TypeScript

#### Don't

```ts
const greeting = "Hello, " + name;
```

#### Do

```ts
const greeting = `Hello, ${name}`;
```

### TSX

#### Don't

```tsx
const greeting = "Hello, " + name;
```

#### Do

```tsx
const greeting = `Hello, ${name}`;
```
