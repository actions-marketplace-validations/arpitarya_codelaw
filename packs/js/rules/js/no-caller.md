---
id: js/no-caller
title: "No arguments.callee or arguments.caller"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the function refers to itself by its own name, which code that reassigns that name, or shadows it inside, reads differently; an anonymous function is named the first of `self`, `self2`, … the file does not use; `arguments.caller` has no replacement and is removed by hand"
stability: preview
ported_from: [eslint/no-caller]
since: 0.5.0
---

`arguments.callee` and `arguments.caller` are not used.

**Why:** Both are forbidden in strict mode and block engine optimisations.

```codelaw
match:
  kind: member_expression
  regex: '^arguments\.(callee|caller)$'
  any:
    - all:
        - regex: '^arguments\.callee$'
        - pattern: $ANON
        - inside: { stopBy: { any: [{ kind: function_expression }, { kind: function_declaration }, { kind: method_definition }, { kind: generator_function }, { kind: generator_function_declaration }] }, kind: function_expression, not: { has: { field: name, regex: . } }, pattern: $FN }
    - all:
        - regex: '^arguments\.callee$'
        - inside: { stopBy: { any: [{ kind: function_expression }, { kind: function_declaration }, { kind: method_definition }, { kind: generator_function }, { kind: generator_function_declaration }] }, any: [{ kind: function_expression }, { kind: function_declaration }], has: { field: name, pattern: $NAME } }
        - pattern: $NAMED
    - regex: ''
message: "Do not use `$$MATCH`; name the function instead"
fix:
  - edit-text: { target: $FN, pattern: '\Afunction\s*(\*?)\s*\(', replace: "function${1} $FRESH(self)(" }
  - replace: { target: $ANON, with: "$FRESH(self)" }
  - replace: { target: $NAMED, with: "$NAME" }
```

## How to fix

1. Give the function a name and refer to it by that name.

## Examples

### JavaScript

#### Don't

```js
const f = function () { return arguments.callee; };
```

#### Do

```js
const f = function self() { return self; };
```

### JSX

#### Don't

```jsx
const f = function () { return arguments.callee; };
```

#### Do

```jsx
const f = function self() { return self; };
```

### TypeScript

#### Don't

```ts
const f = function () { return arguments.callee; };
```

#### Do

```ts
const f = function self() { return self; };
```

### TSX

#### Don't

```tsx
const f = function () { return arguments.callee; };
```

#### Do

```tsx
const f = function self() { return self; };
```
