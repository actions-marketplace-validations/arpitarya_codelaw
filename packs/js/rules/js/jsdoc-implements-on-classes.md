---
id: js/jsdoc-implements-on-classes
title: "@implements only on classes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the tag moves to the class that implements the interface, or is removed; codelaw's edits replace whole nodes, and a comment is one node, so the edit inside its text is the author's"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [eslint-plugin-jsdoc/implements-on-classes]
since: 0.5.0
---

`@implements` documents a class or constructor; on a plain function it is reported unless the block also says `@class` or `@constructor`.

**Why:** A function does not implement an interface; tools that read the tag either ignore it or treat the function as a class it is not.

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
    - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@implements\b'
    - not: {regex: '@(class|constructor)\b'}
    - precedes:
        any:
          - matches: fn-decl
          - {kind: export_statement, has: {field: declaration, matches: fn-decl}}
message: "`@implements` belongs on a class or constructor; mark this function `@class` or move the tag to the class"
```

## How to fix

1. If the function is a constructor, add `@class` to its block.
2. Otherwise remove the `@implements` line, or move it to the class that implements the interface.

## Examples

### JavaScript

#### Don't

```js
/**
 * Makes a shape.
 * @implements {Shape}
 */
function makeShape() {}
```

#### Do

```js
/**
 * Makes a shape.
 * @class
 * @implements {Shape}
 */
function Shape2() {}
```

### JSX

#### Don't

```jsx
/**
 * Makes a shape.
 * @implements {Shape}
 */
function makeShape() {}
```

#### Do

```jsx
/**
 * Makes a shape.
 * @class
 * @implements {Shape}
 */
function Shape2() {}
```

### TypeScript

#### Don't

```ts
/**
 * Makes a shape.
 * @implements {Shape}
 */
function makeShape() {}
```

#### Do

```ts
/**
 * Makes a shape.
 * @class
 * @implements {Shape}
 */
function Shape2() {}
```

### TSX

#### Don't

```tsx
/**
 * Makes a shape.
 * @implements {Shape}
 */
function makeShape() {}
```

#### Do

```tsx
/**
 * Makes a shape.
 * @class
 * @implements {Shape}
 */
function Shape2() {}
```
