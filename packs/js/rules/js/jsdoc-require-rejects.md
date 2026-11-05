---
id: js/jsdoc-require-rejects
title: "A function that rejects documents @rejects"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "when and why the promise rejects is the author's to describe"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/require-rejects]
since: 0.5.0
---

A JSDoc block on an async function that throws, or on a function that returns `Promise.reject(...)`, has a `@rejects` tag. A throw or rejection inside a nested function does not count.

**Why:** Callers of a promise-returning function need to know which failures to `catch`.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - not:
      regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@rejects\b
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
      - any:
        - all:
          - regex: ^(?:(?:static|public|private|protected|override|readonly)\s+)*async\b
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
                kind: throw_statement
        - has:
            field: body
            kind: statement_block
            has:
              stopBy: *id001
              kind: call_expression
              regex: ^Promise\.reject\s*\(
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
message: This function can reject; add a `@rejects` tag saying when
```

## How to fix

1. Add `@rejects {ErrorType} When ...` describing when the returned promise rejects.

## Examples

### JavaScript

#### Don't

```js
/**
 * Loads a user.
 * @param {string} id The user.
 */
async function load(id) {
  if (!id) throw new Error("no id");
  return fetchUser(id);
}
```

#### Do

```js
/**
 * Loads a user.
 * @param {string} id The user.
 * @rejects {Error} When the id is empty.
 */
async function load(id) {
  if (!id) throw new Error("no id");
  return fetchUser(id);
}
```

### JSX

#### Don't

```jsx
/**
 * Loads a user.
 * @param {string} id The user.
 */
async function load(id) {
  if (!id) throw new Error("no id");
  return fetchUser(id);
}
```

#### Do

```jsx
/**
 * Loads a user.
 * @param {string} id The user.
 * @rejects {Error} When the id is empty.
 */
async function load(id) {
  if (!id) throw new Error("no id");
  return fetchUser(id);
}
```

### TypeScript

#### Don't

```ts
/**
 * Loads a user.
 * @param {string} id The user.
 */
async function load(id) {
  if (!id) throw new Error("no id");
  return fetchUser(id);
}
```

#### Do

```ts
/**
 * Loads a user.
 * @param {string} id The user.
 * @rejects {Error} When the id is empty.
 */
async function load(id) {
  if (!id) throw new Error("no id");
  return fetchUser(id);
}
```

### TSX

#### Don't

```tsx
/**
 * Loads a user.
 * @param {string} id The user.
 */
async function load(id) {
  if (!id) throw new Error("no id");
  return fetchUser(id);
}
```

#### Do

```tsx
/**
 * Loads a user.
 * @param {string} id The user.
 * @rejects {Error} When the id is empty.
 */
async function load(id) {
  if (!id) throw new Error("no id");
  return fetchUser(id);
}
```
