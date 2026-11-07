---
id: js/no-break-in-nested-loop
title: "No unlabelled break out of a loop or switch nested in a loop"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which construct the break should leave, and where the inner part moves, is a design choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-break-in-nested-loop]
since: 0.5.0
---

A `break` without a label, whose target — the nearest enclosing loop or `switch` — is itself inside another loop of the same function, is reported. A labelled `break` names its target and is allowed, and so is a `break` whose target is the outermost loop.

**Why:** Inside nested control flow a reader has to count braces to see which statement a bare `break` leaves; a helper function or a label makes the target plain.

```codelaw
utils:
  target:
    any:
      - kind: for_statement
      - kind: for_in_statement
      - kind: while_statement
      - kind: do_statement
      - kind: switch_statement
  loop:
    any:
      - kind: for_statement
      - kind: for_in_statement
      - kind: while_statement
      - kind: do_statement
  fn:
    any:
      - kind: function_declaration
      - kind: function_expression
      - kind: arrow_function
      - kind: method_definition
      - kind: generator_function_declaration
      - kind: generator_function
match:
  kind: break_statement
  not: { has: { kind: statement_identifier } }
  inside:
    stopBy: { any: [{ matches: target }, { matches: fn }] }
    matches: target
    inside:
      stopBy: { any: [{ matches: loop }, { matches: fn }] }
      matches: loop
message: "This `break` is inside nested control flow; move the inner part into a function or label the target"
```

## How to fix

1. Move the inner loop or `switch` into a function and `return` from it, or
2. label the loop the `break` should leave (`outer: for (…) { … break outer; }`).

## Examples

### JavaScript

#### Don't

```js
for (const row of rows) {
  for (const cell of row) {
    if (cell.done) {
      break;
    }
  }
}
```

#### Do

```js
function scan(row) {
  for (const cell of row) {
    if (cell.done) {
      break;
    }
  }
}
for (const row of rows) {
  scan(row);
}
```


### JSX

#### Don't

```jsx
for (const row of rows) {
  for (const cell of row) {
    if (cell.done) {
      break;
    }
  }
}
```

#### Do

```jsx
function scan(row) {
  for (const cell of row) {
    if (cell.done) {
      break;
    }
  }
}
for (const row of rows) {
  scan(row);
}
```


### TypeScript

#### Don't

```ts
for (const row of rows) {
  for (const cell of row) {
    if (cell.done) {
      break;
    }
  }
}
```

#### Do

```ts
function scan(row) {
  for (const cell of row) {
    if (cell.done) {
      break;
    }
  }
}
for (const row of rows) {
  scan(row);
}
```


### TSX

#### Don't

```tsx
for (const row of rows) {
  for (const cell of row) {
    if (cell.done) {
      break;
    }
  }
}
```

#### Do

```tsx
function scan(row) {
  for (const cell of row) {
    if (cell.done) {
      break;
    }
  }
}
for (const row of rows) {
  scan(row);
}
```
