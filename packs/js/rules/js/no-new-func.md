---
id: js/no-new-func
title: "No Function constructor"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the function the string held has to be written as code"
fix_pending: "suggest"
stability: preview
ported_from: [eslint/no-new-func]
since: 0.5.0
---

Functions are not built from strings with `Function` or `new Function`.

**Why:** It is `eval` under another name: slow, unanalysable and an injection risk.

```codelaw
match:
  any:
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
```

## How to fix

1. Write the function as code.

## Examples

### JavaScript

#### Don't

```js
const add = new Function('a', 'b', 'return a + b');
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

#### Do

```jsx
const add = (a, b) => a + b;
```

### TypeScript

#### Don't

```ts
const add = new Function('a', 'b', 'return a + b');
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

#### Do

```tsx
const add = (a, b) => a + b;
```
