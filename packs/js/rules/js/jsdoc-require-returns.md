---
id: js/jsdoc-require-returns
title: "A function that returns a value documents @returns"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the return value means is the author's to describe"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-returns]
since: 0.5.0
---

A JSDoc block on a function that returns a value (a `return` with an expression, or an arrow with an expression body) has a `@returns` tag. Generators, and blocks with `@inheritdoc`, `@override`, `@type` or a file-level tag, are skipped.

**Why:** The return value is half of a function's contract; leaving it out of the docs leaves callers guessing.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - not:
      regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@returns?\b
  - not:
      regex: (?i)@(?:inheritdoc|override|type|file|fileoverview|overview|license|module|copyright)\b
  precedes:
    any:
    - all: &id001
      - any:
        - kind: function_declaration
        - kind: generator_function_declaration
        - kind: function_expression
        - kind: generator_function
        - kind: arrow_function
        - kind: method_definition
      - not:
          any:
          - kind: generator_function_declaration
          - kind: generator_function
          - kind: method_definition
            regex: ^(?:(?:static|public|private|protected|override|readonly)\s+)*(?:async\s+)?\*
      - any:
        - has:
            field: body
            kind: statement_block
            has:
              stopBy:
                any:
                - kind: function_declaration
                - kind: generator_function_declaration
                - kind: function_expression
                - kind: generator_function
                - kind: arrow_function
                - kind: method_definition
                - kind: class_declaration
                - kind: class
              kind: return_statement
              regex: ^return\s*[^\s;]
        - kind: arrow_function
          has:
            field: body
            not:
              kind: statement_block
    - kind: export_statement
      has:
        field: declaration
        all: *id001
    - kind: export_statement
      has:
        field: value
        all: *id001
    - kind: export_statement
      has:
        field: declaration
        any: &id002
        - kind: lexical_declaration
        - kind: variable_declaration
        has: &id003
          kind: variable_declarator
          has:
            field: value
            all: *id001
    - any: *id002
      has: *id003
    - any:
      - kind: pair
      - kind: field_definition
      - kind: public_field_definition
      has:
        field: value
        all: *id001
    - kind: expression_statement
      has:
        kind: assignment_expression
        has:
          field: right
          all: *id001
message: This function returns a value; add a `@returns` tag
```

## How to fix

1. Add `@returns {type} description` saying what the function gives back.

## Examples

### JavaScript

#### Don't

```js
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```js
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```

### JSX

#### Don't

```jsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```jsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```

### TypeScript

#### Don't

```ts
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```ts
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```

### TSX

#### Don't

```tsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```tsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```
