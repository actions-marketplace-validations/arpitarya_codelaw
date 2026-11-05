---
id: js/jsdoc-require-param
title: "A documented function documents its parameters"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "each parameter needs its own @param line with the author's description"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/require-param]
since: 0.5.0
---

A JSDoc block on a function that takes parameters has `@param` tags. Ported in part: a block with no `@param` at all is reported; one that names some parameters and misses others is not (that needs the parameter list compared with the tags). Blocks with `@inheritdoc`, `@override`, `@type` or a file-level tag are skipped.

**Why:** A comment that documents a function but not what it takes leaves the reader guessing at its inputs.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - not:
      regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:param|arg|argument)\b
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
      - any:
        - has:
            field: parameters
            regex: ^\(\s*[^\s)]
        - has:
            field: parameter
            regex: .
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
message: This function takes parameters; add a `@param` tag for each
```

## How to fix

1. Add a `@param` line for each parameter, in order, with its name and what it means.

## Examples

### JavaScript

#### Don't

```js
/**
 * Adds two numbers.
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```js
/**
 * Adds two numbers.
 * @param a The first.
 * @param b The second.
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
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```jsx
/**
 * Adds two numbers.
 * @param a The first.
 * @param b The second.
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
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```ts
/**
 * Adds two numbers.
 * @param a The first.
 * @param b The second.
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
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```tsx
/**
 * Adds two numbers.
 * @param a The first.
 * @param b The second.
 */
function add(a, b) {
  return a + b;
}
```
