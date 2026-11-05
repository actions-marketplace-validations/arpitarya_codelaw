---
id: js/no-this-in-exported-function
title: "No this in an exported function"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "replacing `this` with a parameter changes the function's signature and every caller"
fix_exception: restructure
stability: preview
ported_from: [oxlint/no-this-in-exported-function]
since: 0.5.0
---

An exported function declaration does not use `this`. Arrow functions inside it are checked as part of it; a TypeScript function that declares a `this` parameter is left alone.

**Why:** An imported function is called as `f()`, not as a method, so `this` is `undefined` (or the global object) and the code fails at its first property read.

```codelaw
match:
  kind: this
  inside:
    stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: generator_function_declaration}, {kind: generator_function}, {kind: method_definition}, {kind: class_body}] }
    any: [{kind: function_declaration}, {kind: generator_function_declaration}]
    inside: { kind: export_statement }
    not: { has: { field: parameters, has: { kind: required_parameter, regex: ^this\b } } }
message: "This function is exported, so `this` is not an object here; pass what you need as a parameter"
```

## How to fix

1. Add a parameter for the object the function reads, and pass it at each call site.
2. If the function is meant to be a method, move it into the class or object it belongs to.

## Examples

### JavaScript

#### Don't

```js
export function fullName() {
  return this.first + " " + this.last;
}
```

#### Do

```js
export function fullName(person) {
  return person.first + " " + person.last;
}
```

### JSX

#### Don't

```jsx
export function fullName() {
  return this.first + " " + this.last;
}
```

#### Do

```jsx
export function fullName(person) {
  return person.first + " " + person.last;
}
```

### TypeScript

#### Don't

```ts
export function fullName() {
  return this.first + " " + this.last;
}
```

#### Do

```ts
export function fullName(person) {
  return person.first + " " + person.last;
}
```

### TSX

#### Don't

```tsx
export function fullName() {
  return this.first + " " + this.last;
}
```

#### Do

```tsx
export function fullName(person) {
  return person.first + " " + person.last;
}
```
