---
id: js/jsdoc-require-yields-check
title: "@yields only where a value is yielded"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to yield the documented value or drop the tag is the author's call"
fix_pending: "suggest"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-yields-check]
since: 0.5.0
---

A JSDoc `@yields` tag sits on a generator whose body yields a value. A `yield` inside a nested function does not count.

**Why:** A `@yields` on a generator that yields nothing promises its consumers values they never get.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@yields?\b
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
        - kind: generator_function_declaration
        - kind: generator_function
        - kind: method_definition
          regex: ^(?:(?:static|public|private|protected|override|readonly)\s+)*(?:async\s+)?\*
      - not:
          has:
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
              kind: yield_expression
              regex: ^yield\s*\S
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
message: The docs promise yielded values but the generator yields none
```

## How to fix

1. Yield the documented values, or remove the `@yields` tag.

## Examples

### JavaScript

#### Don't

```js
/**
 * Waits for a tick.
 * @yields {number} The tick.
 */
function* wait() {
  yield;
}
```

#### Do

```js
/**
 * Waits for a tick.
 * @yields {number} The tick.
 */
function* wait() {
  yield Date.now();
}
```

### JSX

#### Don't

```jsx
/**
 * Waits for a tick.
 * @yields {number} The tick.
 */
function* wait() {
  yield;
}
```

#### Do

```jsx
/**
 * Waits for a tick.
 * @yields {number} The tick.
 */
function* wait() {
  yield Date.now();
}
```

### TypeScript

#### Don't

```ts
/**
 * Waits for a tick.
 * @yields {number} The tick.
 */
function* wait() {
  yield;
}
```

#### Do

```ts
/**
 * Waits for a tick.
 * @yields {number} The tick.
 */
function* wait() {
  yield Date.now();
}
```

### TSX

#### Don't

```tsx
/**
 * Waits for a tick.
 * @yields {number} The tick.
 */
function* wait() {
  yield;
}
```

#### Do

```tsx
/**
 * Waits for a tick.
 * @yields {number} The tick.
 */
function* wait() {
  yield Date.now();
}
```
