---
id: js/use-top-level-regex
title: "Declare a regular expression literal at the top level"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new constant needs a name and a place the author picks"
fix_exception: restructure
stability: preview
ported_from: [biome/performance/useTopLevelRegex]
since: 0.5.1
---

A regular expression literal inside a function, method or arrow function is reported, unless it carries the `g` or `y` flag (those keep a position in `lastIndex`, so a shared one changes what the code does). Behaviour of Biome's `useTopLevelRegex`.

**Why:** A literal inside a function is compiled again on every call; a constant at the top of the module is compiled once.

```codelaw
match:
  kind: regex
  all:
    - inside:
        stopBy: end
        any:
          - kind: function_declaration
          - kind: generator_function_declaration
          - kind: function_expression
          - kind: generator_function
          - kind: arrow_function
          - kind: method_definition
    - not: { has: { kind: regex_flags, regex: '[gy]' } }
message: "Move this regular expression to a constant at the top of the module"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
function isId(value) {
  return /^[a-z]+$/.test(value);
}
```

#### Do

```js
const ID = /^[a-z]+$/;

function isId(value) {
  return ID.test(value);
}
```

### JSX

#### Don't

```jsx
function isId(value) {
  return /^[a-z]+$/.test(value);
}
```

#### Do

```jsx
const ID = /^[a-z]+$/;

function isId(value) {
  return ID.test(value);
}
```

### TypeScript

#### Don't

```ts
function isId(value) {
  return /^[a-z]+$/.test(value);
}
```

#### Do

```ts
const ID = /^[a-z]+$/;

function isId(value) {
  return ID.test(value);
}
```

### TSX

#### Don't

```tsx
function isId(value) {
  return /^[a-z]+$/.test(value);
}
```

#### Do

```tsx
const ID = /^[a-z]+$/;

function isId(value) {
  return ID.test(value);
}
```
