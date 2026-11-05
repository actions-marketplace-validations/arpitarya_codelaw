---
id: js/no-new-func
title: "No Function constructor"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a function expression with the literal parameters and body is the closest equivalent; it closes over local scope instead of global scope, which the reviewer confirms (calls with computed or escaped strings carry no edit)"
stability: preview
ported_from: [eslint/no-new-func]
since: 0.5.0
---

Functions are not built from strings with `Function` or `new Function`.

**Why:** It is `eval` under another name: slow, unanalysable and an injection risk.

```codelaw
match:
  any:
    - all:
        - any: [{ pattern: "new $F($S1)" }, { pattern: "$F($S1)" }]
        - pattern: $M1
        - has:
            field: arguments
            has:
              kind: string
              nthChild: 1
              not: { has: { kind: escape_sequence } }
              has: { kind: string_fragment, pattern: $F11 }
    - all:
        - any: [{ pattern: "new $F($S1, $S2)" }, { pattern: "$F($S1, $S2)" }]
        - pattern: $M2
        - has:
            field: arguments
            has:
              kind: string
              nthChild: 1
              not: { has: { kind: escape_sequence } }
              has: { kind: string_fragment, pattern: $F21 }
        - has:
            field: arguments
            has:
              kind: string
              nthChild: 2
              not: { has: { kind: escape_sequence } }
              has: { kind: string_fragment, pattern: $F22 }
    - all:
        - any: [{ pattern: "new $F($S1, $S2, $S3)" }, { pattern: "$F($S1, $S2, $S3)" }]
        - pattern: $M3
        - has:
            field: arguments
            has:
              kind: string
              nthChild: 1
              not: { has: { kind: escape_sequence } }
              has: { kind: string_fragment, pattern: $F31 }
        - has:
            field: arguments
            has:
              kind: string
              nthChild: 2
              not: { has: { kind: escape_sequence } }
              has: { kind: string_fragment, pattern: $F32 }
        - has:
            field: arguments
            has:
              kind: string
              nthChild: 3
              not: { has: { kind: escape_sequence } }
              has: { kind: string_fragment, pattern: $F33 }
    - pattern: new $F($$$A)
    - pattern: $F($$$A)
    - pattern: $F.call($$$A)
    - pattern: $F.apply($$$A)
    - pattern: $F.bind($$$A)
constraints:
  F: { kind: identifier, regex: '^Function$' }
where:
  $F: builtin
message: "Do not build functions with the `Function` constructor"
fix:
  - replace: { target: $M1, with: "function () { $F11 }" }
  - replace: { target: $M2, with: "function ($F21) { $F22 }" }
  - replace: { target: $M3, with: "function ($F31, $F32) { $F33 }" }
```

## How to fix

1. Write the function as code.

## Examples

### JavaScript

#### Don't

```js
const add = new Function('a', 'b', 'return a + b');
```

#### Fixed

```js
const add = function (a, b) { return a + b };
```

#### Do

```js
const add = (a, b) => a + b;
```

### JSX

#### Don't

```jsx
const add = new Function('a', 'b', 'return a + b');
```

#### Fixed

```jsx
const add = function (a, b) { return a + b };
```

#### Do

```jsx
const add = (a, b) => a + b;
```

### TypeScript

#### Don't

```ts
const add = new Function('a', 'b', 'return a + b');
```

#### Fixed

```ts
const add = function (a, b) { return a + b };
```

#### Do

```ts
const add = (a, b) => a + b;
```

### TSX

#### Don't

```tsx
const add = new Function('a', 'b', 'return a + b');
```

#### Fixed

```tsx
const add = function (a, b) { return a + b };
```

#### Do

```tsx
const add = (a, b) => a + b;
```
