---
id: js/jsdoc-match-description
title: "A function's description is a sentence"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "rewording the description is the author's"
stability: preview
ported_from: [eslint-plugin-jsdoc/match-description]
since: 0.5.0
---

The description of a function's JSDoc block, when it has one, starts with a capital letter, a digit, `_` or a backtick, and ends with `.`, `?`, `!` or a backtick — upstream's default pattern. Only the main description is checked, not `@description` tags.

**Why:** Descriptions are rendered as prose in hovers and generated docs; a consistent sentence form reads as finished documentation rather than a note.

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
        - regex: '^/\*\*[ \t]*(\n[ \t]*\*[ \t]*)*[^A-Z`0-9_\s*@/]'
        - regex: '^/\*\*([^@]|\{@)*[^.?!`\s*][ \t]*((\n[ \t]*\*?[ \t]*)*\*/$|(\n[ \t]*\*?[ \t]*)+@)'
    - precedes: {matches: fn}
message: "Start the description with a capital letter and end it with a full stop"
```

## How to fix

1. Start the description with a capital letter.
2. End it with `.`, `?` or `!`.

## Examples

### JavaScript

#### Don't

```js
/**
 * returns the total
 */
function total() {}
```

#### Do

```js
/**
 * Returns the total.
 */
function total() {}
```

### JSX

#### Don't

```jsx
/**
 * returns the total
 */
function total() {}
```

#### Do

```jsx
/**
 * Returns the total.
 */
function total() {}
```

### TypeScript

#### Don't

```ts
/**
 * returns the total
 */
function total() {}
```

#### Do

```ts
/**
 * Returns the total.
 */
function total() {}
```

### TSX

#### Don't

```tsx
/**
 * returns the total
 */
function total() {}
```

#### Do

```tsx
/**
 * Returns the total.
 */
function total() {}
```
