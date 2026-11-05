---
id: js/jsdoc-require-description
title: "A function's JSDoc has a description"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's to write"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/require-description]
since: 0.5.0
---

A JSDoc block on a function, method or function-valued `const` has description text before its first tag. A block with `@inheritdoc` is exempt.

**Why:** Tags say what the parameters are; only the description says what the function is for, and that is what a reader looks for first.

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
    - regex: '^/\*\*[\s*]*(@|/$)'
    - not: {regex: '@inherit[dD]oc\b'}
    - precedes: {matches: fn}
message: "Describe what this function does before its tags"
```

## How to fix

1. Write one sentence on what the function does, as the first line of the block.

## Examples

### JavaScript

#### Don't

```js
/**
 * @param {number} n the number
 */
function double(n) {
  return n * 2;
}
```

#### Do

```js
/**
 * Doubles a number.
 * @param {number} n the number
 */
function double(n) {
  return n * 2;
}
```

### JSX

#### Don't

```jsx
/**
 * @param {number} n the number
 */
function double(n) {
  return n * 2;
}
```

#### Do

```jsx
/**
 * Doubles a number.
 * @param {number} n the number
 */
function double(n) {
  return n * 2;
}
```

### TypeScript

#### Don't

```ts
/**
 * @param {number} n the number
 */
function double(n) {
  return n * 2;
}
```

#### Do

```ts
/**
 * Doubles a number.
 * @param {number} n the number
 */
function double(n) {
  return n * 2;
}
```

### TSX

#### Don't

```tsx
/**
 * @param {number} n the number
 */
function double(n) {
  return n * 2;
}
```

#### Do

```tsx
/**
 * Doubles a number.
 * @param {number} n the number
 */
function double(n) {
  return n * 2;
}
```
