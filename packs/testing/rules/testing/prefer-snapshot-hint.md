---
id: testing/prefer-snapshot-hint
title: "A hint on each of several snapshots"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the hint names what the snapshot shows; the author writes it"
fix_exception: authored
stability: preview
ported_from: ["eslint-plugin-jest/prefer-snapshot-hint", "@vitest/eslint-plugin/prefer-snapshot-hint"]
since: 0.5.0
---

When a block has more than one `toMatchSnapshot()`, each is given a hint: `toMatchSnapshot("after save")`.

**Why:** Snapshots are keyed by test title and a counter; without hints, adding one in the middle renumbers the rest and the diff is unreadable.

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
  matches: matcher-call
  has: { field: arguments, not: { has: { nthChild: 1 } } }
  all:
    - has: { field: function, has: { field: property, regex: '^(toMatchSnapshot|toThrowErrorMatchingSnapshot)$' } }
    - inside:
        kind: expression_statement
        any:
          - follows: { stopBy: end, has: { stopBy: end, kind: property_identifier, regex: '^(toMatchSnapshot|toThrowErrorMatchingSnapshot)$' } }
          - precedes: { stopBy: end, has: { stopBy: end, kind: property_identifier, regex: '^(toMatchSnapshot|toThrowErrorMatchingSnapshot)$' } }
message: "Give each snapshot in this test a hint"
```

## How to fix

1. Pass a short hint to each snapshot matcher: `toMatchSnapshot("before save")`.

## Examples

### JavaScript

#### Don't

```js
test("form", () => {
  expect(a).toMatchSnapshot();
  expect(b).toMatchSnapshot();
});
```

#### Do

```js
test("form", () => {
  expect(a).toMatchSnapshot("empty");
  expect(b).toMatchSnapshot("filled");
});
```

### JSX

#### Don't

```jsx
test("form", () => {
  expect(a).toMatchSnapshot();
  expect(b).toMatchSnapshot();
});
```

#### Do

```jsx
test("form", () => {
  expect(a).toMatchSnapshot("empty");
  expect(b).toMatchSnapshot("filled");
});
```

### TypeScript

#### Don't

```ts
test("form", () => {
  expect(a).toMatchSnapshot();
  expect(b).toMatchSnapshot();
});
```

#### Do

```ts
test("form", () => {
  expect(a).toMatchSnapshot("empty");
  expect(b).toMatchSnapshot("filled");
});
```

### TSX

#### Don't

```tsx
test("form", () => {
  expect(a).toMatchSnapshot();
  expect(b).toMatchSnapshot();
});
```

#### Do

```tsx
test("form", () => {
  expect(a).toMatchSnapshot("empty");
  expect(b).toMatchSnapshot("filled");
});
```
