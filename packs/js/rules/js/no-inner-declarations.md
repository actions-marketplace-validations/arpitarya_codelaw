---
id: js/no-inner-declarations
title: "No function declaration in a nested block"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the function changes what it can see"
stability: preview
ported_from: [eslint/no-inner-declarations]
since: 0.5.0
---

A function declaration sits at the top level of a file or of a function body, not inside an `if` or loop block (the upstream default, `"functions"`).

**Why:** Block-level function declarations behave differently between strict and sloppy mode and between engines.

```codelaw

match:
  kind: function_declaration
  inside: { kind: statement_block, not: { inside: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}, {kind: class_static_block}], stopBy: neighbor } } }
message: "Declare the function at the top of the file or the function, not inside a block"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
if (debug) {
  function log(x) {
    print(x);
  }
}
```

#### Do

```js
function log(x) {
  print(x);
}
```

### JSX

#### Don't

```jsx
if (debug) {
  function log(x) {
    print(x);
  }
}
```

#### Do

```jsx
function log(x) {
  print(x);
}
```

### TypeScript

#### Don't

```ts
if (debug) {
  function log(x) {
    print(x);
  }
}
```

#### Do

```ts
function log(x) {
  print(x);
}
```

### TSX

#### Don't

```tsx
if (debug) {
  function log(x) {
    print(x);
  }
}
```

#### Do

```tsx
function log(x) {
  print(x);
}
```
