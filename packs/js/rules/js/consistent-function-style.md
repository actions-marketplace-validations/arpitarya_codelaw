---
id: js/consistent-function-style
title: "Functions take the syntax their role asks for"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "converting a callback between a function expression and an arrow changes `this` and `arguments` when the body uses them; only a callback whose body uses neither is offered, and the reviewer confirms"
stability: preview
options:
  default: { type: "string", default: "", description: "style of a function with no other role: declaration, function-expression, arrow-function or ignore" }
  defaultExport: { type: "string", default: "", description: "style of `export default function`; unset, an anonymous default export is not read" }
  namedFunctions: { type: "string", default: "", description: "style of a declared function or a `const f = \u2026` function" }
  namedExports: { type: "string", default: "", description: "style of an exported function" }
  callbacks: { type: "string", default: "", description: "style of a function passed as an argument (function-expression, arrow-function or ignore)" }
  objectProperties: { type: "string", default: "", description: "style of a function that is an object property (method, function-expression, arrow-function or ignore)" }
  reassignedVariables: { type: "string", default: "", description: "style of a function assigned to a variable that is later reassigned" }
  typedVariables: { type: "string", default: "", description: "style of a function assigned to a variable with a type annotation" }
ported_from: [eslint-plugin-unicorn/consistent-function-style]
since: 0.5.2
---

Each role a function can have has a style the project may fix: `declaration` (`function f() {}`), `function-expression`, `arrow-function` or, for an object property, `method`; `ignore` (the default for every role) leaves it alone. The roles, in the order they are decided: a default export, a variable with a type annotation, a variable that is reassigned, a named export, a callback, an object property, a declared or variable function, and any other function. A function that is a class member, a parameter default or called on the spot is not read, and a generator is never asked to be an arrow. Behaviour of eslint-plugin-unicorn 79.0.0's rule, read and reimplemented. A finding offers the conversion only for a callback written as a function expression where an arrow is asked, or the reverse, and only when its body uses none of `this`, `arguments`, `super` or `new.target`, has no comment and has no type parameters; every other finding is reported without an edit.

Set the roles you care about: `[rules] "js/consistent-function-style" = { severity = "error", options = { namedFunctions = "declaration", callbacks = "arrow-function" } }`. With no `default`, `namedFunctions` or another role` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** One syntax for one kind of function lets a reader tell a declared function from a callback at a glance.

```codelaw
native: js/consistent-function-style
```

## Examples

### JavaScript

#### Don't

```js options={"callbacks":"arrow-function"}
items.map(function (x) {
  return x * 2;
});
```

#### Do

```js options={"callbacks":"arrow-function"}
items.map((x) => {
  return x * 2;
});
```

### JSX

#### Don't

```jsx options={"callbacks":"arrow-function"}
items.map(function (x) {
  return x * 2;
});
```

#### Do

```jsx options={"callbacks":"arrow-function"}
items.map((x) => {
  return x * 2;
});
```

### TypeScript

#### Don't

```ts options={"callbacks":"arrow-function"}
items.map(function (x) {
  return x * 2;
});
```

#### Do

```ts options={"callbacks":"arrow-function"}
items.map((x) => {
  return x * 2;
});
```

### TSX

#### Don't

```tsx options={"callbacks":"arrow-function"}
items.map(function (x) {
  return x * 2;
});
```

#### Do

```tsx options={"callbacks":"arrow-function"}
items.map((x) => {
  return x * 2;
});
```
