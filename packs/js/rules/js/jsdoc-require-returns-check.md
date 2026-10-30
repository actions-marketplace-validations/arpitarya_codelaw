---
id: js/jsdoc-require-returns-check
title: "@returns only where a value is returned"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to return the documented value or drop the tag is the author's call"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-returns-check]
since: 0.5.0
---

A JSDoc `@returns` tag (other than `{void}`, `{undefined}`, `{never}` or `{Promise<void>}`) sits on a function whose body returns a value. Async functions, generators, functions that throw, and blocks with `@abstract`, `@virtual` or `@interface` are skipped.

**Why:** A `@returns` on a function that returns nothing promises callers a value they never get.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@returns?\b
  - not:
      regex: '@returns?[ \t]*\{(?:void|undefined|never|Promise<void>)\}'
  - not:
      regex: '@(?:abstract|virtual|interface)\b'
  - not:
      regex: (?i)@(?:inheritdoc|override|type|file|fileoverview|overview|license|module|copyright)\b
  precedes:
    any:
    - all: &id002
      - any:
        - kind: function_declaration
        - kind: generator_function_declaration
        - kind: function_expression
        - kind: generator_function
        - kind: arrow_function
        - kind: method_definition
      - has:
          field: body
          kind: statement_block
      - not:
          regex: ^(?:(?:static|public|private|protected|override|readonly)\s+)*async\b
      - not:
          any:
          - kind: generator_function_declaration
          - kind: generator_function
          - kind: method_definition
            regex: ^(?:(?:static|public|private|protected|override|readonly)\s+)*(?:async\s+)?\*
      - not:
          any:
          - has:
              field: body
              kind: statement_block
              has:
                stopBy: &id001
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
      - not:
          has:
            field: body
            kind: statement_block
            has:
              stopBy: *id001
              kind: throw_statement
    - kind: export_statement
      has:
        field: declaration
        all: *id002
    - kind: export_statement
      has:
        field: value
        all: *id002
    - kind: export_statement
      has:
        field: declaration
        any: &id003
        - kind: lexical_declaration
        - kind: variable_declaration
        has: &id004
          kind: variable_declarator
          has:
            field: value
            all: *id002
    - any: *id003
      has: *id004
    - any:
      - kind: pair
      - kind: field_definition
      - kind: public_field_definition
      has:
        field: value
        all: *id002
    - kind: expression_statement
      has:
        kind: assignment_expression
        has:
          field: right
          all: *id002
message: The docs promise a return value but the function returns none
```

## How to fix

1. Return the documented value, or remove the `@returns` tag (or make it `@returns {void}`).

## Examples

### JavaScript

#### Don't

```js
/**
 * Logs a message.
 * @param {string} m The message.
 * @returns {string} The message.
 */
function log(m) {
  console.log(m);
}
```

#### Do

```js
/**
 * Logs a message.
 * @param {string} m The message.
 * @returns {string} The message.
 */
function log(m) {
  console.log(m);
  return m;
}
```

### JSX

#### Don't

```jsx
/**
 * Logs a message.
 * @param {string} m The message.
 * @returns {string} The message.
 */
function log(m) {
  console.log(m);
}
```

#### Do

```jsx
/**
 * Logs a message.
 * @param {string} m The message.
 * @returns {string} The message.
 */
function log(m) {
  console.log(m);
  return m;
}
```

### TypeScript

#### Don't

```ts
/**
 * Logs a message.
 * @param {string} m The message.
 * @returns {string} The message.
 */
function log(m) {
  console.log(m);
}
```

#### Do

```ts
/**
 * Logs a message.
 * @param {string} m The message.
 * @returns {string} The message.
 */
function log(m) {
  console.log(m);
  return m;
}
```

### TSX

#### Don't

```tsx
/**
 * Logs a message.
 * @param {string} m The message.
 * @returns {string} The message.
 */
function log(m) {
  console.log(m);
}
```

#### Do

```tsx
/**
 * Logs a message.
 * @param {string} m The message.
 * @returns {string} The message.
 */
function log(m) {
  console.log(m);
  return m;
}
```
