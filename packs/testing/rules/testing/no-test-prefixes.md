---
id: testing/no-test-prefixes
title: "`.skip` and `.only`, not `x` and `f` prefixes"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/no-test-prefixes", "@vitest/eslint-plugin/no-test-prefixes"]
since: 0.5.0
---

A skipped or focused test is written with `.skip` or `.only` — `it.skip`, `describe.only` — not with Jasmine's prefixes `xit`, `xtest`, `xdescribe`, `fit`, `fdescribe`.

**Why:** One spelling is easier to search for and to catch in review; the prefixes are easy to miss.

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
  kind: identifier
  all:
    - any:
      - inside: { kind: call_expression, field: function }
      - inside: { kind: member_expression, field: object, inside: { kind: call_expression, field: function } }
    - any:
      - { regex: '^xit$', pattern: $X0 }
      - { regex: '^xtest$', pattern: $X1 }
      - { regex: '^xdescribe$', pattern: $X2 }
      - { regex: '^fit$', pattern: $X3 }
      - { regex: '^fdescribe$', pattern: $X4 }
message: "Use `$$MATCH`'s `.skip` or `.only` form"
fix:
  - replace: { target: $X0, with: it.skip }
  - replace: { target: $X1, with: test.skip }
  - replace: { target: $X2, with: describe.skip }
  - replace: { target: $X3, with: it.only }
  - replace: { target: $X4, with: describe.only }
```

## Examples

### JavaScript

#### Don't

```js
xit("rounds", () => {
  expect(round(1.5)).toBe(2);
});
```

#### Do

```js
it.skip("rounds", () => {
  expect(round(1.5)).toBe(2);
});
```

### JSX

#### Don't

```jsx
xit("rounds", () => {
  expect(round(1.5)).toBe(2);
});
```

#### Do

```jsx
it.skip("rounds", () => {
  expect(round(1.5)).toBe(2);
});
```

### TypeScript

#### Don't

```ts
xit("rounds", () => {
  expect(round(1.5)).toBe(2);
});
```

#### Do

```ts
it.skip("rounds", () => {
  expect(round(1.5)).toBe(2);
});
```

### TSX

#### Don't

```tsx
xit("rounds", () => {
  expect(round(1.5)).toBe(2);
});
```

#### Do

```tsx
it.skip("rounds", () => {
  expect(round(1.5)).toBe(2);
});
```
