---
id: js/no-same-argument-assert
title: "No assertion of a value against itself"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the expected value is the author's"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-same-argument-assert]
since: 0.5.0
---

An equality assertion does not compare an expression with itself.

**Why:** It always passes (or always fails, for `notEqual`), so the test checks nothing.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: ^assert\.(equal|strictEqual|deepEqual|deepStrictEqual|notEqual|notStrictEqual)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        pattern: $A
      all:
      - has:
          nthChild: 2
          pattern: $A
message: Both arguments are the same; the assertion tests nothing
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
assert.equal(result, result);
```

#### Do

```js
assert.equal(result, expected);
```

### JSX

#### Don't

```jsx
assert.equal(result, result);
```

#### Do

```jsx
assert.equal(result, expected);
```

### TypeScript

#### Don't

```ts
assert.equal(result, result);
```

#### Do

```ts
assert.equal(result, expected);
```

### TSX

#### Don't

```tsx
assert.equal(result, result);
```

#### Do

```tsx
assert.equal(result, expected);
```
