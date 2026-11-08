---
id: js/no-use-before-define
title: "No variable, function or class used before it is defined"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving a definition above its first use reorders the file"
fix_exception: restructure
options: { functions: { type: "boolean", default: true, description: "report functions used before their declaration" }, classes: { type: "boolean", default: true, description: "report classes used before their declaration from an inner function" }, variables: { type: "boolean", default: true, description: "report variables used before their declaration from an inner function" }, allowNamedExports: { type: "boolean", default: false, description: "allow `export { x }` above `x`" } }
stability: preview
ported_from: [eslint/no-use-before-define]
since: 0.5.0
---

A reference that comes, in source order, before the declaration it resolves to is reported — also one inside the declarator's own value (`const a = a + 1`). With ESLint's defaults that includes hoisted functions and `var`s and references from inner functions; the `functions`, `classes`, `variables` and `allowNamedExports` options relax it as ESLint's do. TypeScript type references, interfaces and type aliases are not this rule's.

**Why:** A `let`, `const` or `class` read before its line throws; a hoisted `var` reads `undefined`; and code is easier to follow when names are defined first.

```codelaw
native: js/no-use-before-define
```

## How to fix

1. Move the declaration above its first use.

## Examples

### JavaScript

#### Don't

```js
greet();
const name = "x";
function greet() {
  return name;
}
```

#### Do

```js
const name = "x";
function greet() {
  return name;
}
greet();
```


### JSX

#### Don't

```jsx
greet();
const name = "x";
function greet() {
  return name;
}
```

#### Do

```jsx
const name = "x";
function greet() {
  return name;
}
greet();
```


### TypeScript

#### Don't

```ts
greet();
const name = "x";
function greet() {
  return name;
}
```

#### Do

```ts
const name = "x";
function greet() {
  return name;
}
greet();
```


### TSX

#### Don't

```tsx
greet();
const name = "x";
function greet() {
  return name;
}
```

#### Do

```tsx
const name = "x";
function greet() {
  return name;
}
greet();
```
