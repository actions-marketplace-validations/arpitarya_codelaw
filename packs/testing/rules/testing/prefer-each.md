---
id: testing/prefer-each
title: "`test.each` over a loop of tests"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the loop's data becomes a table and its body a parameterised test"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-jest/prefer-each", "@vitest/eslint-plugin/prefer-each"]
since: 0.5.0
---

Tests generated in a loop are written with `test.each` / `describe.each`.

**Why:** `each` names every case in the report and can run one case alone; a loop does neither.

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
  any: [{ kind: for_statement }, { kind: for_in_statement }, { kind: while_statement }]
  has: { field: body, stopBy: end, any: [{ matches: test-call }, { matches: describe-call }, { matches: hook-call }] }
  not: { inside: { stopBy: end, matches: test-call } }
message: "Use `test.each` instead of a loop"
```

## How to fix

1. Turn the loop's values into a table: `test.each([[1, 2], [3, 4]])`.
2. Move the loop body into the test callback, with the values as its parameters.

## Examples

### JavaScript

#### Don't

```js
for (const n of [1, 2, 3]) {
  test(`doubles ${n}`, () => {
    expect(double(n)).toBe(n * 2);
  });
}
```

#### Do

```js
test.each([1, 2, 3])("doubles %i", (n) => {
  expect(double(n)).toBe(n * 2);
});
```

### JSX

#### Don't

```jsx
for (const n of [1, 2, 3]) {
  test(`doubles ${n}`, () => {
    expect(double(n)).toBe(n * 2);
  });
}
```

#### Do

```jsx
test.each([1, 2, 3])("doubles %i", (n) => {
  expect(double(n)).toBe(n * 2);
});
```

### TypeScript

#### Don't

```ts
for (const n of [1, 2, 3]) {
  test(`doubles ${n}`, () => {
    expect(double(n)).toBe(n * 2);
  });
}
```

#### Do

```ts
test.each([1, 2, 3])("doubles %i", (n) => {
  expect(double(n)).toBe(n * 2);
});
```

### TSX

#### Don't

```tsx
for (const n of [1, 2, 3]) {
  test(`doubles ${n}`, () => {
    expect(double(n)).toBe(n * 2);
  });
}
```

#### Do

```tsx
test.each([1, 2, 3])("doubles %i", (n) => {
  expect(double(n)).toBe(n * 2);
});
```
