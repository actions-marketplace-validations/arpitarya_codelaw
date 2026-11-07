---
id: js/jsdoc-convert-to-jsdoc-comments
title: "Functions documented in JSDoc, not // comments"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a plain comment above a function may be a note to the maintainer rather than its documentation; the author confirms it is meant for readers of the API"
stability: preview
ported_from: [eslint-plugin-jsdoc/convert-to-jsdoc-comments]
since: 0.5.0
---

A plain `//` or `/* */` comment directly above a function, method or function-valued `const` is written as a `/** */` JSDoc block. Directive comments (`// eslint-…`, `// @ts-…`, `// prettier-…`, coverage pragmas, `///` references) are left alone. Upstream's variable and property contexts are left out: this port covers functions only, and a comment separated by a blank line still counts.

**Why:** Editors, TypeScript and documentation tools show a JSDoc block on hover and in generated docs; a line comment above a function is invisible to all of them.

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
    - regex: '^(//|/\*([^*]|$))'
    - not: {regex: '^(///|(//|/\*)[ \t]*(@ts-|istanbul |c8 |v8 |eslint|prettier-))'}
    - precedes: {matches: fn}
message: "Write this function's comment as a `/** … */` JSDoc block so editors and doc tools show it"
fix:
  - edit-text: { pattern: '\A/\*([^*])', replace: '/**${1}' }
  - edit-text: { pattern: '\A//[ \t]*((?:[^*\n]|\*[^/\n])*\*?)[ \t]*\z', replace: '/** ${1} */' }
```

## How to fix

1. Turn the comment into a JSDoc block: `// Adds two numbers.` becomes `/** Adds two numbers. */`, and several `//` lines become the lines of one `/** … */` block.

## Examples

### JavaScript

#### Don't

```js
// Adds two numbers.
function add(a, b) {
  return a + b;
}
```

#### Do

```js
/** Adds two numbers. */
function add(a, b) {
  return a + b;
}
```

### JSX

#### Don't

```jsx
// Adds two numbers.
function add(a, b) {
  return a + b;
}
```

#### Do

```jsx
/** Adds two numbers. */
function add(a, b) {
  return a + b;
}
```

### TypeScript

#### Don't

```ts
// Adds two numbers.
function add(a, b) {
  return a + b;
}
```

#### Do

```ts
/** Adds two numbers. */
function add(a, b) {
  return a + b;
}
```

### TSX

#### Don't

```tsx
// Adds two numbers.
function add(a, b) {
  return a + b;
}
```

#### Do

```tsx
/** Adds two numbers. */
function add(a, b) {
  return a + b;
}
```
