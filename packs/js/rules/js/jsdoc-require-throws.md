---
id: js/jsdoc-require-throws
title: "A function that throws documents @throws"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "when and why the function throws is the author's to describe"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-throws]
since: 0.5.0
---

A JSDoc block on a non-async function whose body has a `throw` has a `@throws` tag. A throw inside a nested function does not count; async functions are `jsdoc-require-rejects`'s.

**Why:** Callers need to know which errors to expect and when, and the signature does not say.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - not:
      regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:throws|exception)\b
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
          regex: ^(?:(?:static|public|private|protected|override|readonly)\s+)*async\b
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
            kind: throw_statement
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
message: This function throws; add a `@throws` tag saying when
```

## How to fix

1. Add `@throws {ErrorType} When ...` describing when the function throws.

## Examples

### JavaScript

#### Don't

```js
/**
 * Parses a port.
 * @param {string} s The text.
 * @returns {number} The port.
 */
function port(s) {
  const n = Number(s);
  if (!n) throw new RangeError(s);
  return n;
}
```

#### Do

```js
/**
 * Parses a port.
 * @param {string} s The text.
 * @returns {number} The port.
 * @throws {RangeError} When the text is not a number.
 */
function port(s) {
  const n = Number(s);
  if (!n) throw new RangeError(s);
  return n;
}
```

### JSX

#### Don't

```jsx
/**
 * Parses a port.
 * @param {string} s The text.
 * @returns {number} The port.
 */
function port(s) {
  const n = Number(s);
  if (!n) throw new RangeError(s);
  return n;
}
```

#### Do

```jsx
/**
 * Parses a port.
 * @param {string} s The text.
 * @returns {number} The port.
 * @throws {RangeError} When the text is not a number.
 */
function port(s) {
  const n = Number(s);
  if (!n) throw new RangeError(s);
  return n;
}
```

### TypeScript

#### Don't

```ts
/**
 * Parses a port.
 * @param {string} s The text.
 * @returns {number} The port.
 */
function port(s) {
  const n = Number(s);
  if (!n) throw new RangeError(s);
  return n;
}
```

#### Do

```ts
/**
 * Parses a port.
 * @param {string} s The text.
 * @returns {number} The port.
 * @throws {RangeError} When the text is not a number.
 */
function port(s) {
  const n = Number(s);
  if (!n) throw new RangeError(s);
  return n;
}
```

### TSX

#### Don't

```tsx
/**
 * Parses a port.
 * @param {string} s The text.
 * @returns {number} The port.
 */
function port(s) {
  const n = Number(s);
  if (!n) throw new RangeError(s);
  return n;
}
```

#### Do

```tsx
/**
 * Parses a port.
 * @param {string} s The text.
 * @returns {number} The port.
 * @throws {RangeError} When the text is not a number.
 */
function port(s) {
  const n = Number(s);
  if (!n) throw new RangeError(s);
  return n;
}
```
