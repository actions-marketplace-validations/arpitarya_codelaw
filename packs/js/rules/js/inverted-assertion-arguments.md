---
id: js/inverted-assertion-arguments
title: "Actual first, expected second"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the operands swap evaluation order"
stability: preview
ported_from: [eslint-plugin-sonarjs/inverted-assertion-arguments]
since: 0.5.0
---

An `assert.equal`-style call does not put a literal (the expected value) first.

**Why:** The failure message labels the arguments "actual" and "expected", so swapped ones mislead.

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
        any:
        - kind: string
        - kind: number
        - kind: 'true'
        - kind: 'false'
        - kind: 'null'
        pattern: $E
      all:
      - has:
          nthChild: 2
          not:
            any:
            - kind: string
            - kind: number
            - kind: 'true'
            - kind: 'false'
            - kind: 'null'
          pattern: $A
message: Pass the actual value first and the expected one second
fix:
- replace:
    target: $E
    with: $A
- replace:
    target: $A
    with: $E
```

## Examples

### JavaScript

#### Don't

```js
assert.equal(3, sum(1, 2));
```

#### Do

```js
assert.equal(sum(1, 2), 3);
```

### JSX

#### Don't

```jsx
assert.equal(3, sum(1, 2));
```

#### Do

```jsx
assert.equal(sum(1, 2), 3);
```

### TypeScript

#### Don't

```ts
assert.equal(3, sum(1, 2));
```

#### Do

```ts
assert.equal(sum(1, 2), 3);
```

### TSX

#### Don't

```tsx
assert.equal(3, sum(1, 2));
```

#### Do

```tsx
assert.equal(sum(1, 2), 3);
```
