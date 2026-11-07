---
id: js/max-statements
title: "No function with more than ten statements"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where to split the function is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint/max-statements]
since: 0.5.0
---

A function's body has at most ten statements (ESLint's default `max: 10`). ESLint also counts statements inside nested blocks; this port counts the body's own statements, so it reports only functions that are over the limit by either count.

**Why:** A long function does several things; split, each part can be named and tested.

```codelaw
match:
  any:
    - kind: function_declaration
    - kind: function_expression
    - kind: arrow_function
    - kind: method_definition
  has:
    field: body
    kind: statement_block
    has: { nthChild: 11, not: { kind: comment } }
message: "This function has more than ten statements; split it"
```

## How to fix

1. Move a coherent group of statements into a function with a name that says what it does, and call it.

## Examples

### JavaScript

#### Don't

```js
function setup() {
  a(); b(); c(); d(); e();
  f(); g(); h(); i(); j();
  k();
}
```

#### Do

```js
function setup() {
  a(); b(); c(); d(); e();
  rest();
}
```


### JSX

#### Don't

```jsx
function setup() {
  a(); b(); c(); d(); e();
  f(); g(); h(); i(); j();
  k();
}
```

#### Do

```jsx
function setup() {
  a(); b(); c(); d(); e();
  rest();
}
```


### TypeScript

#### Don't

```ts
function setup() {
  a(); b(); c(); d(); e();
  f(); g(); h(); i(); j();
  k();
}
```

#### Do

```ts
function setup() {
  a(); b(); c(); d(); e();
  rest();
}
```


### TSX

#### Don't

```tsx
function setup() {
  a(); b(); c(); d(); e();
  f(); g(); h(); i(); j();
  k();
}
```

#### Do

```tsx
function setup() {
  a(); b(); c(); d(); e();
  rest();
}
```
