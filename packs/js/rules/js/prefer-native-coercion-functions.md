---
id: js/prefer-native-coercion-functions
title: "String rather than x => String(x)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "called by `map`, the function also receives the index, which `String` and `Boolean` ignore but `Number` and `BigInt` also ignore; only a custom caller could tell"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-native-coercion-functions]
since: 0.5.0
---

An arrow function that only calls `String`, `Number`, `BigInt`, `Boolean` or `Symbol` on its parameter is replaced by the function.

**Why:** The wrapper adds nothing; the conversion functions take one argument.

```codelaw
match:
  kind: arrow_function
  has:
    field: parameters
    has:
      any:
      - kind: identifier
        pattern: $X
      - kind: required_parameter
        not:
          has:
            field: type
            regex: .
        has:
          field: pattern
          kind: identifier
          pattern: $X
    not:
      has:
        nthChild: 2
  all:
  - has:
      field: body
      pattern: $F($X)
constraints:
  F:
    regex: ^(String|Number|BigInt|Boolean|Symbol)$
message: Pass `$F` itself
fix: $F
```

## Examples

### JavaScript

#### Don't

```js
const names = ids.map((x) => String(x));
```

#### Do

```js
const names = ids.map(String);
```

### JSX

#### Don't

```jsx
const names = ids.map((x) => String(x));
```

#### Do

```jsx
const names = ids.map(String);
```

### TypeScript

#### Don't

```ts
const names = ids.map((x) => String(x));
```

#### Do

```ts
const names = ids.map(String);
```

### TSX

#### Don't

```tsx
const names = ids.map((x) => String(x));
```

#### Do

```tsx
const names = ids.map(String);
```
