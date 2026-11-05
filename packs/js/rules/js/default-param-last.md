---
id: js/default-param-last
title: "Default parameters come last"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "reordering parameters changes every call site"
fix_exception: restructure
stability: preview
ported_from: [eslint/default-param-last]
since: 0.5.0
---

Parameters with default values come after the ones without.

**Why:** A default before a required parameter only applies when the caller passes `undefined` explicitly, so it is effectively never used.

```codelaw

utils:
  defaulted: { any: [{kind: assignment_pattern}, {kind: required_parameter, has: { field: value, regex: "." }}] }
  plain:
    any:
      - kind: identifier
      - kind: object_pattern
      - kind: array_pattern
      - all: [{kind: required_parameter}, {not: {has: { field: value, regex: "." }}}]
match:
  matches: defaulted
  inside: { kind: formal_parameters }
  precedes: { stopBy: end, matches: plain }
message: "A parameter with a default comes before one without; move it after"
```

## How to fix

1. Move the defaulted parameters to the end and update every call.

## Examples

### JavaScript

#### Don't

```js
function greet(greeting = "hi", name) {
  return greeting + name;
}
```

#### Do

```js
function greet(name, greeting = "hi") {
  return greeting + name;
}
```

### JSX

#### Don't

```jsx
function greet(greeting = "hi", name) {
  return greeting + name;
}
```

#### Do

```jsx
function greet(name, greeting = "hi") {
  return greeting + name;
}
```

### TypeScript

#### Don't

```ts
function greet(greeting = "hi", name) {
  return greeting + name;
}
```

#### Do

```ts
function greet(name, greeting = "hi") {
  return greeting + name;
}
```

### TSX

#### Don't

```tsx
function greet(greeting = "hi", name) {
  return greeting + name;
}
```

#### Do

```tsx
function greet(name, greeting = "hi") {
  return greeting + name;
}
```
