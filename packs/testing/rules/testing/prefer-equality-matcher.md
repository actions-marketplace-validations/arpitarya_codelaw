---
id: testing/prefer-equality-matcher
title: "An equality matcher for an equality"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "`toBe` compares with `Object.is`, not `===`: `NaN` and `-0` compare differently, so the author confirms the edit"
stability: preview
ported_from: ["eslint-plugin-jest/prefer-equality-matcher", "@vitest/eslint-plugin/prefer-equality-matcher"]
since: 0.5.0
---

`expect(a === b).toBe(true)` is written `expect(a).toBe(b)` (and `!==` or `false` as `not.toBe`). The suggested edit uses `toBe`; `toEqual` or `toStrictEqual` fits when the values are objects.

**Why:** An equality matcher prints both values when it fails; a boolean prints only `true` and `false`.

```codelaw
utils:
  expect-call:
    kind: call_expression
    has:
      field: function
      any:
        - { kind: identifier, regex: '^expect$' }
        - kind: member_expression
          all:
            - has: { field: property, regex: '^(soft|poll)$' }
            - has: { field: object, kind: identifier, regex: '^expect$' }
  expect-chain:
    any:
      - matches: expect-call
      - kind: member_expression
        has:
          field: object
          any:
            - matches: expect-call
            - { kind: member_expression, has: { field: object, matches: expect-call } }
  matcher-call:
    kind: call_expression
    has:
      field: function
      kind: member_expression
      has: { field: object, matches: expect-chain }
  test-name:
    any:
      - { kind: identifier, regex: '^(it|test|xit|xtest|fit)$' }
      - kind: member_expression
        all:
          - has: { field: property, regex: '^(only|skip|concurrent|failing|todo|fixme|fail|slow)$' }
          - regex: '^(it|test|xit|xtest|fit)(\.(only|skip|concurrent|failing|todo|fixme|fail|slow))+$'
  describe-name:
    any:
      - { kind: identifier, regex: '^(describe|xdescribe|fdescribe|suite)$' }
      - kind: member_expression
        all:
          - has: { field: property, regex: '^(describe|only|skip|serial|parallel|concurrent|fixme)$' }
          - regex: '^(describe|xdescribe|fdescribe|test\.describe|suite)(\.(only|skip|serial|parallel|concurrent|fixme))*$'
  test-call:
    kind: call_expression
    has:
      field: function
      any:
        - matches: test-name
        - kind: call_expression
          has:
            field: function
            kind: member_expression
            all:
              - has: { field: property, regex: '^each$' }
              - regex: '^(it|test|xit|xtest|fit)(\.(only|skip|concurrent|failing))*\.each$'
  describe-call:
    kind: call_expression
    has:
      field: function
      any:
        - matches: describe-name
        - kind: call_expression
          has:
            field: function
            kind: member_expression
            all:
              - has: { field: property, regex: '^each$' }
              - regex: '^(describe|xdescribe|fdescribe|suite)(\.(only|skip|concurrent))*\.each$'
  hook-call:
    kind: call_expression
    has:
      field: function
      any:
        - { kind: identifier, regex: '^(beforeAll|beforeEach|afterAll|afterEach)$' }
        - kind: member_expression
          all:
            - has: { field: property, regex: '^(beforeAll|beforeEach|afterAll|afterEach)$' }
            - has: { field: object, kind: identifier, regex: '^test$' }
  test-fn:
    any: [{ kind: arrow_function }, { kind: function_expression }]
    inside: { kind: arguments, inside: { matches: test-call } }
  describe-fn:
    any: [{ kind: arrow_function }, { kind: function_expression }]
    inside: { kind: arguments, inside: { matches: describe-call } }
  in-test:
    inside: { stopBy: end, any: [{ matches: test-call }, { matches: hook-call }] }
  block-level:
    kind: expression_statement
    any:
      - inside: { kind: program }
      - inside: { kind: statement_block, inside: { matches: describe-fn } }
match:
  kind: call_expression
  any:
    - all: [{ pattern: "expect($A === $B).$M(true)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($A === $B).$M(false)" }, { pattern: $W1 }]
    - all: [{ pattern: "expect($A !== $B).$M(true)" }, { pattern: $W2 }]
    - all: [{ pattern: "expect($A !== $B).$M(false)" }, { pattern: $W3 }]
constraints:
  M: { regex: '^(toBe|toEqual|toStrictEqual)$' }
message: "Use an equality matcher"
fix:
  - replace: { target: $W0, with: 'expect($A).toBe($B)' }
  - replace: { target: $W1, with: 'expect($A).not.toBe($B)' }
  - replace: { target: $W2, with: 'expect($A).not.toBe($B)' }
  - replace: { target: $W3, with: 'expect($A).toBe($B)' }
```

## Examples

### JavaScript

#### Don't

```js
test("ids", () => {
  expect(a.id === b.id).toBe(true);
});
```

#### Do

```js
test("ids", () => {
  expect(a.id).toBe(b.id);
});
```

### JSX

#### Don't

```jsx
test("ids", () => {
  expect(a.id === b.id).toBe(true);
});
```

#### Do

```jsx
test("ids", () => {
  expect(a.id).toBe(b.id);
});
```

### TypeScript

#### Don't

```ts
test("ids", () => {
  expect(a.id === b.id).toBe(true);
});
```

#### Do

```ts
test("ids", () => {
  expect(a.id).toBe(b.id);
});
```

### TSX

#### Don't

```tsx
test("ids", () => {
  expect(a.id === b.id).toBe(true);
});
```

#### Do

```tsx
test("ids", () => {
  expect(a.id).toBe(b.id);
});
```
