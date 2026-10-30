---
id: js/no-unused-function-parameters
title: "No unused trailing parameter"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "renaming the parameter to `_name` marks it unused; removing it instead may change the signature callers rely on"
stability: preview
ported_from: [biome/correctness/noUnusedFunctionParameters]
since: 0.5.0
---

A function's last parameter is used in its body, or named with a leading underscore.

**Why:** An unused parameter suggests a forgotten use, or a signature that promises more than it does.

```codelaw
utils:
  fn:
    any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}]
    has: { field: body, regex: '.' }
match:
  kind: identifier
  regex: '^[^_]'
  any:
    - inside: { kind: formal_parameters, inside: { matches: fn } }
      nthChild: { position: 1, reverse: true }
    - inside: { kind: required_parameter, nthChild: { position: 1, reverse: true }, not: { has: { kind: accessibility_modifier } }, inside: { kind: formal_parameters, inside: { matches: fn } } }
      nthChild: 1
    - inside: { kind: arrow_function, field: parameter }
where:
  $$MATCH: unused
message: "`$$MATCH` is never used; use it, or rename it `_$$MATCH`"
fix: "_$$MATCH"
```

## Examples

### JavaScript

#### Don't

```js
function greet(name, punctuation) {
  return 'Hi ' + name;
}
```

#### Do

```js
function greet(name, _punctuation) {
  return 'Hi ' + name;
}
```

### JSX

#### Don't

```jsx
function greet(name, punctuation) {
  return 'Hi ' + name;
}
```

#### Do

```jsx
function greet(name, _punctuation) {
  return 'Hi ' + name;
}
```

### TypeScript

#### Don't

```ts
function greet(name, punctuation) {
  return 'Hi ' + name;
}
```

#### Do

```ts
function greet(name, _punctuation) {
  return 'Hi ' + name;
}
```

### TSX

#### Don't

```tsx
function greet(name, punctuation) {
  return 'Hi ' + name;
}
```

#### Do

```tsx
function greet(name, _punctuation) {
  return 'Hi ' + name;
}
```

## Differences from the source

Only the last parameter is checked — earlier unused ones are kept by the parameters after them; destructured parameters are not checked.
