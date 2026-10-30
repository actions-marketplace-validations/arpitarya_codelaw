---
id: node/global-require
title: "require() at the top of the module"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a require inside a function may be deliberate lazy loading or a way round a require cycle; moving it changes when the module loads"
stability: preview
ported_from: [eslint-plugin-n/global-require]
since: 0.5.0
---

`require()` is called at the top level of the module, not inside a function, block, loop or class. A `require()` inside a top-level object or array literal is not reported (upstream reports it).

**Why:** A `require()` buried in a function hides a dependency from a reader of the file's head and runs synchronous file I/O the first time the function is called, often on a hot path.

```codelaw
match:
  pattern: $R($$$A)
  inside:
    stopBy: end
    any: [{kind: statement_block}, {kind: arrow_function}, {kind: class_body}, {kind: if_statement}, {kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}, {kind: switch_body}]
constraints:
  R: { regex: ^require$ }
where:
  $R: undeclared
message: "Move this `require()` to the top of the module"
```

## How to fix

1. Move the `require()` to the top of the file and bind it to a `const`.
2. If it is lazy on purpose (a heavy or optional module, a require cycle), keep it and add the file to the baseline.

## Examples

### JavaScript

#### Don't

```js
function render(md) {
  const marked = require("marked");
  return marked.parse(md);
}
```

#### Do

```js
const marked = require("marked");

function render(md) {
  return marked.parse(md);
}
```

### JSX

#### Don't

```jsx
function render(md) {
  const marked = require("marked");
  return marked.parse(md);
}
```

#### Do

```jsx
const marked = require("marked");

function render(md) {
  return marked.parse(md);
}
```

### TypeScript

#### Don't

```ts
function render(md) {
  const marked = require("marked");
  return marked.parse(md);
}
```

#### Do

```ts
const marked = require("marked");

function render(md) {
  return marked.parse(md);
}
```

### TSX

#### Don't

```tsx
function render(md) {
  const marked = require("marked");
  return marked.parse(md);
}
```

#### Do

```tsx
const marked = require("marked");

function render(md) {
  return marked.parse(md);
}
```
