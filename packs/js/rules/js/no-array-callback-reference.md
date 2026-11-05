---
id: js/no-array-callback-reference
title: "Wrap function references passed to array methods"
languages: [javascript, jsx]
fix: suggest
fix_reason: "the wrapped function no longer receives the index and the array, which it may have used"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-callback-reference]
since: 0.5.0
---

A bare function name is not passed as the callback of an array method (built-in converters like `Boolean` are allowed).

**Why:** Array methods pass `(element, index, array)`; a function with optional parameters, like `parseInt`, silently takes the index too.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    has:
      field: property
      regex: ^(every|filter|find|findLast|findIndex|findLastIndex|flatMap|forEach|map|reduce|reduceRight|some)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: identifier
        pattern: $F
        not:
          regex: ^(Boolean|String|Number|BigInt|Array|Object|Symbol)$
message: Wrap `$F` in an arrow function so it gets only the arguments it expects
fix:
- replace:
    target: $F
    with: (x) => $F(x)
```

## Examples

### JavaScript

#### Don't

```js
const n = inputs.map(parseInt);
```

#### Do

```js
const n = inputs.map((x) => parseInt(x));
```

### JSX

#### Don't

```jsx
const n = inputs.map(parseInt);
```

#### Do

```jsx
const n = inputs.map((x) => parseInt(x));
```
