---
id: js/promise-param-names
title: "A promise executor names its parameters resolve and reject"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "renaming a parameter is safe only when every use in the executor is renamed with it; rename it by hand or with the editor"
fix_pending: "auto-subset; needs scope-rename (W-162)"
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
    - any: [{kind: identifier}, {kind: required_parameter}, {kind: optional_parameter}]
      nthChild: 1
      not: { regex: '^_?resolve\b' }
      inside: { kind: formal_parameters, inside: { matches: executor } }
    - any: [{kind: identifier}, {kind: required_parameter}, {kind: optional_parameter}]
      nthChild: 2
      not: { regex: '^_?reject\b' }
      inside: { kind: formal_parameters, inside: { matches: executor } }
message: "Name the promise executor's parameters `resolve` and `reject`, in that order"
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
