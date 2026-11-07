---
id: js/vars-on-top
title: "var declarations at the top of their scope"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving a declaration up may move an initialiser's side effects; the author moves it"
fix_exception: restructure
stability: preview
ported_from: [eslint/vars-on-top]
since: 0.5.0
---

A `var` declaration sits at the top of its function or file — after directives and imports, before any other statement — and not inside a nested block, where it would still belong to the whole function. A `var` in a `for` head is left alone.

**Why:** `var` is hoisted to the top of the function anyway; writing it there shows its real scope. A matter of taste, so in the opt-in `js/taste` preset.

```codelaw
utils:
  top-scope:
    any:
      - kind: program
      - kind: statement_block
        inside:
          field: body
          any:
            - kind: function_declaration
            - kind: function_expression
            - kind: arrow_function
            - kind: method_definition
            - kind: generator_function
            - kind: generator_function_declaration
  allowed-before:
    any:
      - kind: variable_declaration
      - kind: import_statement
      - kind: comment
      - kind: empty_statement
      - regex: '^\{$'
      - kind: expression_statement
        has: { kind: string }
match:
  kind: variable_declaration
  not:
    any:
      - inside: { kind: for_statement }
      - inside: { kind: for_in_statement }
  any:
    - not: { inside: { any: [{ matches: top-scope }, { kind: export_statement }] } }
    - follows: { stopBy: end, not: { matches: allowed-before } }
message: "Move this `var` to the top of its scope"
```

## How to fix

1. Move the declaration to the top of the function (or file), and leave the assignment where it is if it must happen later.

## Examples

### JavaScript

#### Don't

```js
function run() {
  start();
  var count = 0;
  return count;
}
```

#### Do

```js
function run() {
  var count = 0;
  start();
  return count;
}
```


### JSX

#### Don't

```jsx
function run() {
  start();
  var count = 0;
  return count;
}
```

#### Do

```jsx
function run() {
  var count = 0;
  start();
  return count;
}
```


### TypeScript

#### Don't

```ts
function run() {
  start();
  var count = 0;
  return count;
}
```

#### Do

```ts
function run() {
  var count = 0;
  start();
  return count;
}
```


### TSX

#### Don't

```tsx
function run() {
  start();
  var count = 0;
  return count;
}
```

#### Do

```tsx
function run() {
  var count = 0;
  start();
  return count;
}
```
