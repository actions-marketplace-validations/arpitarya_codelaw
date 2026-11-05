---
id: testing/require-test-timeout
title: "Every test sets a timeout"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the right timeout depends on what the test does"
fix_exception: value
stability: preview
ported_from: ["@vitest/eslint-plugin/require-test-timeout"]
since: 0.5.0
---

Each test passes a timeout, as its third argument or `{ timeout }`. Opt-in, for suites where the global default is not wanted.

**Why:** A per-test timeout documents how long the test is expected to take and stops a hung one quickly.

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
  matches: test-call
  all:
    - not: { has: { field: function, regex: '\.todo$' } }
    - has: { field: arguments, all: [{ has: { nthChild: 2 } }, { not: { has: { nthChild: 3 } } }] }
message: "Give this test a timeout"
```

## How to fix

1. Pass the timeout in milliseconds as the last argument: `test("…", async () => { … }, 5_000)`.

## Examples

### JavaScript

#### Don't

```js
test("loads", async () => {
  await load();
});
```

#### Do

```js
test("loads", async () => {
  await load();
}, 5000);
```

### JSX

#### Don't

```jsx
test("loads", async () => {
  await load();
});
```

#### Do

```jsx
test("loads", async () => {
  await load();
}, 5000);
```

### TypeScript

#### Don't

```ts
test("loads", async () => {
  await load();
});
```

#### Do

```ts
test("loads", async () => {
  await load();
}, 5000);
```

### TSX

#### Don't

```tsx
test("loads", async () => {
  await load();
});
```

#### Do

```tsx
test("loads", async () => {
  await load();
}, 5000);
```
