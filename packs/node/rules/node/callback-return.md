---
id: node/callback-return
title: "Return after calling the callback"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`return cb(...)` also makes the function return what the callback returns; callers of a callback-style function rarely read it"
stability: preview
ported_from: [eslint-plugin-n/callback-return]
since: 0.5.0
---

A call to `callback`, `cb` or `next` inside a function is returned, or is the last thing the function does.

**Why:** Code after the callback keeps running: the classic bug is `if (err) cb(err);` that goes on to call `cb` a second time with a result.

```codelaw
match:
  kind: expression_statement
  has:
    kind: call_expression
    pattern: $CB($$$ARGS)
  all:
    - any:
        - inside: { kind: statement_block }
        - inside: { kind: if_statement }
        - inside: { kind: else_clause }
    - inside: { stopBy: end, any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}] }
  not:
    any:
      - precedes: { kind: return_statement }
      - all:
          - not: { precedes: { stopBy: end, any: [{kind: expression_statement}, {kind: return_statement}, {kind: if_statement}, {kind: lexical_declaration}, {kind: variable_declaration}, {kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}, {kind: try_statement}, {kind: switch_statement}, {kind: throw_statement}, {kind: function_declaration}, {kind: class_declaration}, {kind: statement_block}] } }
          - inside: { kind: statement_block, inside: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}] } }
constraints:
  CB: { regex: ^(callback|cb|next)$ }
message: "Code after `$CB(...)` still runs; return it: `return $CB(...)`"
fix: "return $CB($$$ARGS);"
```

## Examples

### JavaScript

#### Don't

```js
function load(path, cb) {
  if (!path) {
    cb(new Error("no path"));
  }
  read(path, cb);
}
```

#### Do

```js
function load(path, cb) {
  if (!path) {
    return cb(new Error("no path"));
  }
  read(path, cb);
}
```

### JSX

#### Don't

```jsx
function load(path, cb) {
  if (!path) {
    cb(new Error("no path"));
  }
  read(path, cb);
}
```

#### Do

```jsx
function load(path, cb) {
  if (!path) {
    return cb(new Error("no path"));
  }
  read(path, cb);
}
```

### TypeScript

#### Don't

```ts
function load(path, cb) {
  if (!path) {
    cb(new Error("no path"));
  }
  read(path, cb);
}
```

#### Do

```ts
function load(path, cb) {
  if (!path) {
    return cb(new Error("no path"));
  }
  read(path, cb);
}
```

### TSX

#### Don't

```tsx
function load(path, cb) {
  if (!path) {
    cb(new Error("no path"));
  }
  read(path, cb);
}
```

#### Do

```tsx
function load(path, cb) {
  if (!path) {
    return cb(new Error("no path"));
  }
  read(path, cb);
}
```
