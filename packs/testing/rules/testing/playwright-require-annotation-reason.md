---
id: testing/playwright-require-annotation-reason
title: "Annotations give a reason"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the reason is the author's to write"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-playwright/require-annotation-reason"]
since: 0.5.0
---

`test.skip()`, `test.fixme()`, `test.fail()` and `test.slow()` called inside a test are given a description: `test.skip(isMobile, "no hover on touch")`. Opt-in.

**Why:** An annotation without a reason is a skipped test nobody can judge; the description shows in the report.

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
  all:
    - has: { field: function, regex: '^test\.(skip|fixme|fail|slow)$' }
    - has: { field: arguments, not: { has: { any: [{ kind: string }, { kind: template_string }] } } }
message: "Say why"
```

## How to fix

1. Add the reason as the last argument: `test.fixme(true, "crashes on Safari, #123")`.

## Examples

### JavaScript

#### Don't

```js
test("t", async () => {
  test.skip();
});
```

#### Do

```js
test("t", async () => {
  test.skip(true, "needs the payments sandbox");
});
```

### JSX

#### Don't

```jsx
test("t", async () => {
  test.skip();
});
```

#### Do

```jsx
test("t", async () => {
  test.skip(true, "needs the payments sandbox");
});
```

### TypeScript

#### Don't

```ts
test("t", async () => {
  test.skip();
});
```

#### Do

```ts
test("t", async () => {
  test.skip(true, "needs the payments sandbox");
});
```

### TSX

#### Don't

```tsx
test("t", async () => {
  test.skip();
});
```

#### Do

```tsx
test("t", async () => {
  test.skip(true, "needs the payments sandbox");
});
```
