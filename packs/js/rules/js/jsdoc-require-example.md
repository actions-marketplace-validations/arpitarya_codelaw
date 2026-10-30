---
id: js/jsdoc-require-example
title: "A function's JSDoc has an @example"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the example is the author's to write"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-example]
since: 0.5.0
---

A JSDoc block on a function, method or function-valued `const` includes an `@example`. A block with `@inheritdoc` is exempt; a function with no JSDoc block is not reported here.

**Why:** An example shows how to call the function faster than any description, and is what readers copy.

```codelaw
utils:
  fn-value:
    has:
      kind: variable_declarator
      has: {field: value, any: [{kind: arrow_function}, {kind: function_expression}]}
  fn-decl:
    any:
      - kind: function_declaration
      - kind: generator_function_declaration
      - {kind: lexical_declaration, matches: fn-value}
      - {kind: variable_declaration, matches: fn-value}
  fn:
    any:
      - matches: fn-decl
      - {kind: export_statement, has: {field: declaration, matches: fn-decl}}
      - kind: method_definition
      - {kind: pair, has: {field: value, any: [{kind: arrow_function}, {kind: function_expression}]}}
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - not: {regex: '@(example|inheritdoc|inheritDoc)\b'}
    - precedes: {matches: fn}
message: "Add an `@example` showing how to call this function"
```

## How to fix

1. Add an `@example` tag followed by a short call and its result.

## Examples

### JavaScript

#### Don't

```js
/**
 * Doubles a number.
 */
function double(n) {
  return n * 2;
}
```

#### Do

```js
/**
 * Doubles a number.
 * @example
 * double(2); // 4
 */
function double(n) {
  return n * 2;
}
```

### JSX

#### Don't

```jsx
/**
 * Doubles a number.
 */
function double(n) {
  return n * 2;
}
```

#### Do

```jsx
/**
 * Doubles a number.
 * @example
 * double(2); // 4
 */
function double(n) {
  return n * 2;
}
```

### TypeScript

#### Don't

```ts
/**
 * Doubles a number.
 */
function double(n) {
  return n * 2;
}
```

#### Do

```ts
/**
 * Doubles a number.
 * @example
 * double(2); // 4
 */
function double(n) {
  return n * 2;
}
```

### TSX

#### Don't

```tsx
/**
 * Doubles a number.
 */
function double(n) {
  return n * 2;
}
```

#### Do

```tsx
/**
 * Doubles a number.
 * @example
 * double(2); // 4
 */
function double(n) {
  return n * 2;
}
```
