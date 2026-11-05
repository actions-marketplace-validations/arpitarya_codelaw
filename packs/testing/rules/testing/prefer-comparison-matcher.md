---
id: testing/prefer-comparison-matcher
title: "Comparison matchers for comparisons"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/prefer-comparison-matcher", "@vitest/eslint-plugin/prefer-comparison-matcher"]
since: 0.5.0
---

`expect(a > b).toBe(true)` is written `expect(a).toBeGreaterThan(b)`, and likewise for `>=`, `<` and `<=` and their `false` forms.

**Why:** The comparison matchers print both values on failure; a boolean prints only `true` and `false`.

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
    - all: [{ pattern: "expect($A > $B).$M(true)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect($A >= $B).$M(true)" }, { pattern: $W1 }]
    - all: [{ pattern: "expect($A < $B).$M(true)" }, { pattern: $W2 }]
    - all: [{ pattern: "expect($A <= $B).$M(true)" }, { pattern: $W3 }]
    - all: [{ pattern: "expect($A > $B).$M(false)" }, { pattern: $W4 }]
    - all: [{ pattern: "expect($A >= $B).$M(false)" }, { pattern: $W5 }]
    - all: [{ pattern: "expect($A < $B).$M(false)" }, { pattern: $W6 }]
    - all: [{ pattern: "expect($A <= $B).$M(false)" }, { pattern: $W7 }]
constraints:
  M: { regex: '^(toBe|toEqual|toStrictEqual)$' }
message: "Use a comparison matcher"
fix:
  - replace: { target: $W0, with: 'expect($A).toBeGreaterThan($B)' }
  - replace: { target: $W1, with: 'expect($A).toBeGreaterThanOrEqual($B)' }
  - replace: { target: $W2, with: 'expect($A).toBeLessThan($B)' }
  - replace: { target: $W3, with: 'expect($A).toBeLessThanOrEqual($B)' }
  - replace: { target: $W4, with: 'expect($A).not.toBeGreaterThan($B)' }
  - replace: { target: $W5, with: 'expect($A).not.toBeGreaterThanOrEqual($B)' }
  - replace: { target: $W6, with: 'expect($A).not.toBeLessThan($B)' }
  - replace: { target: $W7, with: 'expect($A).not.toBeLessThanOrEqual($B)' }
```

## Examples

### JavaScript

#### Don't

```js
test("grows", () => {
  expect(size >= 2).toBe(true);
});
```

#### Do

```js
test("grows", () => {
  expect(size).toBeGreaterThanOrEqual(2);
});
```

### JSX

#### Don't

```jsx
test("grows", () => {
  expect(size >= 2).toBe(true);
});
```

#### Do

```jsx
test("grows", () => {
  expect(size).toBeGreaterThanOrEqual(2);
});
```

### TypeScript

#### Don't

```ts
test("grows", () => {
  expect(size >= 2).toBe(true);
});
```

#### Do

```ts
test("grows", () => {
  expect(size).toBeGreaterThanOrEqual(2);
});
```

### TSX

#### Don't

```tsx
test("grows", () => {
  expect(size >= 2).toBe(true);
});
```

#### Do

```tsx
test("grows", () => {
  expect(size).toBeGreaterThanOrEqual(2);
});
```
