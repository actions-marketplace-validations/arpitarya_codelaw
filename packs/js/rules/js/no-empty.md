---
id: js/no-empty
title: "No empty blocks"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "an empty block standing alone as a statement has no effect and is removed; an empty catch, if, loop or switch body carries no edit (it needs a reason or a body)"
stability: preview
ported_from: [eslint/no-empty]
since: 0.1.0
---

A block or `switch` has at least one statement, or a comment saying why it is empty. Function bodies
may be empty.

**Why:** An empty block is usually unfinished code, and an empty `catch` hides errors.

```codelaw
match:
  any:
    - kind: statement_block
      pattern: $B
      regex: ^\{\s*\}$
      inside:
        stopBy: neighbor
        any: [{kind: program}, {kind: statement_block}]
    - kind: statement_block
      regex: ^\{\s*\}$
      not:
        inside:
          stopBy: neighbor
          any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}, { kind: class_static_block }]
    - kind: switch_statement
      has: { field: body, regex: '^\{\s*\}$' }
message: "Empty block; remove it or say why it's empty"
fix:
  - remove: $B
```

## How to fix

1. If the block is unfinished, write its code.
2. If it's meant to be empty, put a comment in it saying why.

## Examples

### JavaScript

#### Don't

```js
function f() {
  {
  }
  run();
}
```

#### Do

```js
function f() {
  run();
}
```

### JSX

#### Don't

```jsx
function f() {
  {
  }
  run();
}
```

#### Do

```jsx
function f() {
  run();
}
```

### TypeScript

#### Don't

```ts
function f() {
  {
  }
  run();
}
```

#### Do

```ts
function f() {
  run();
}
```

### TSX

#### Don't

```tsx
function f() {
  {
  }
  run();
}
```

#### Do

```tsx
function f() {
  run();
}
```
