---
id: js/jsdoc-require-description-complete-sentence
title: "A JSDoc description is complete sentences"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the sentence is the author's to finish"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-description-complete-sentence]
since: 0.5.0
---

The description of a function's JSDoc block starts with a capital letter and ends with `.`, `?` or `!`. Only the main description is checked; upstream also checks tag descriptions.

**Why:** A description in fragments reads as an unfinished note in hovers and generated docs.

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
    - any:
        - regex: '^/\*\*[ \t]*(\n[ \t]*\*[ \t]*)*[a-z]'
        - regex: '^/\*\*([^@]|\{@)*[^.?!\s*][ \t]*((\n[ \t]*\*?[ \t]*)*\*/$|(\n[ \t]*\*?[ \t]*)+@)'
    - precedes: {matches: fn}
message: "Write the description as complete sentences: a capital letter at the start and a full stop at the end"
```

## How to fix

1. Capitalise the description's first word.
2. End the description with `.`, `?` or `!`.

## Examples

### JavaScript

#### Don't

```js
/**
 * fetches the user
 */
function fetchUser() {}
```

#### Do

```js
/**
 * Fetches the user.
 */
function fetchUser() {}
```

### JSX

#### Don't

```jsx
/**
 * fetches the user
 */
function fetchUser() {}
```

#### Do

```jsx
/**
 * Fetches the user.
 */
function fetchUser() {}
```

### TypeScript

#### Don't

```ts
/**
 * fetches the user
 */
function fetchUser() {}
```

#### Do

```ts
/**
 * Fetches the user.
 */
function fetchUser() {}
```

### TSX

#### Don't

```tsx
/**
 * fetches the user
 */
function fetchUser() {}
```

#### Do

```tsx
/**
 * Fetches the user.
 */
function fetchUser() {}
```
