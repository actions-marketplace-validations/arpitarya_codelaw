---
id: js/no-lone-blocks
title: "No needless nested blocks"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the match already excludes blocks that declare a let, const, class or function, so unwrapping a bare block and keeping its statements is exact"
stability: preview
ported_from: [eslint/no-lone-blocks]
since: 0.5.0
---

A block `{ … }` is not written on its own unless it scopes a `let`, `const`, `class` or function declaration.

**Why:** A bare block does nothing in modern JavaScript except scope those; otherwise it only suggests a missing `if`.

```codelaw
match:
  kind: statement_block
  pattern: "{ $$$S }"
  any:
    - inside: { kind: statement_block }
    - inside: { kind: program }
    - inside: { kind: switch_case }
  not:
    has:
      any: [{kind: lexical_declaration}, {kind: class_declaration}, {kind: function_declaration}]
message: "This block is not needed"
fix: "$$$S"
```

## How to fix

1. Remove the braces, or restore the `if` they belonged to.

## Examples

### JavaScript

#### Don't

```js
function f() {
  {
    run();
  }
}
```

#### Fixed

```js
function f() {
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
    run();
  }
}
```

#### Fixed

```jsx
function f() {
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
    run();
  }
}
```

#### Fixed

```ts
function f() {
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
    run();
  }
}
```

#### Fixed

```tsx
function f() {
  run();
}
```

#### Do

```tsx
function f() {
  run();
}
```
