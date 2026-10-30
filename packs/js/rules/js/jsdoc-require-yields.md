---
id: js/jsdoc-require-yields
title: "A generator that yields documents @yields"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the yielded values mean is the author's to describe"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-yields]
since: 0.5.0
---

A JSDoc block on a generator that yields a value has a `@yields` tag. A `yield` inside a nested function does not count; blocks with `@inheritdoc`, `@override` or `@type` are skipped.

**Why:** What a generator yields is its output; without `@yields` its consumers must read the body to find out.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - not:
      regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@yields?\b
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
message: This generator yields values; add a `@yields` tag
```

## How to fix

1. Add `@yields {type} description` saying what each yielded value is.

## Examples

### JavaScript

#### Don't

```js
/**
 * Counts from zero.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

#### Do

```js
/**
 * Counts from zero.
 * @yields {number} The next id.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

### JSX

#### Don't

```jsx
/**
 * Counts from zero.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

#### Do

```jsx
/**
 * Counts from zero.
 * @yields {number} The next id.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

### TypeScript

#### Don't

```ts
/**
 * Counts from zero.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

#### Do

```ts
/**
 * Counts from zero.
 * @yields {number} The next id.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

### TSX

#### Don't

```tsx
/**
 * Counts from zero.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```

#### Do

```tsx
/**
 * Counts from zero.
 * @yields {number} The next id.
 */
function* ids() {
  let n = 0;
  while (true) yield n++;
}
```
