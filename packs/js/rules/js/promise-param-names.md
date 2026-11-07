---
id: js/promise-param-names
title: "A promise executor names its parameters resolve and reject"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "each parameter is renamed with every use of it in the executor; where `resolve` or `reject` is already a name in the file, it is renamed by hand"
stability: preview
ported_from: [eslint-plugin-promise/param-names]
since: 0.5.0
---

The function passed to `new Promise` calls its first parameter `resolve` and its second `reject` (an underscore in front is allowed for one that is unused).

**Why:** Every reader expects those two names; `new Promise((ok, fail) => …)`, or the two swapped, makes each call site a puzzle and hides a resolve that should have been a reject.

```codelaw
utils:
  executor:
    any: [{kind: arrow_function}, {kind: function_expression}]
    inside:
      kind: arguments
      inside:
        kind: new_expression
        has: { field: constructor, regex: '^Promise$' }
match:
  any:
    - kind: identifier
      not: { regex: '^_?resolve$' }
      inside: { matches: executor, field: parameter }
      pattern: $RES
    - any: [{kind: identifier}, {kind: required_parameter}, {kind: optional_parameter}]
      nthChild: 1
      not: { regex: '^_?resolve\b' }
      inside: { kind: formal_parameters, inside: { matches: executor } }
      pattern: $RES
    - any: [{kind: identifier}, {kind: required_parameter}, {kind: optional_parameter}]
      nthChild: 2
      not: { regex: '^_?reject\b' }
      inside: { kind: formal_parameters, inside: { matches: executor } }
      pattern: $REJ
message: "Name the promise executor's parameters `resolve` and `reject`, in that order"
fix:
  - rename-symbol: { target: $RES, to: resolve, scope: binding }
  - rename-symbol: { target: $REJ, to: reject, scope: binding }
```

## How to fix

1. Rename the first parameter to `resolve` and the second to `reject`, and every use of each in the executor.

## Examples

### JavaScript

#### Don't

```js
const p = new Promise((ok, fail) => setTimeout(ok, 10));
```

#### Do

```js
const p = new Promise((resolve, reject) => setTimeout(resolve, 10));
```

### JSX

#### Don't

```jsx
const p = new Promise((ok, fail) => setTimeout(ok, 10));
```

#### Do

```jsx
const p = new Promise((resolve, reject) => setTimeout(resolve, 10));
```

### TypeScript

#### Don't

```ts
const p = new Promise((ok, fail) => setTimeout(ok, 10));
```

#### Do

```ts
const p = new Promise((resolve, reject) => setTimeout(resolve, 10));
```

### TSX

#### Don't

```tsx
const p = new Promise((ok, fail) => setTimeout(ok, 10));
```

#### Do

```tsx
const p = new Promise((resolve, reject) => setTimeout(resolve, 10));
```
