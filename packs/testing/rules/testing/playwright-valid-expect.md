---
id: testing/playwright-valid-expect
title: "`expect` is called right and awaited when async"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-playwright/valid-expect"]
since: 0.5.0
---

`expect` takes one argument (and, in Vitest and Playwright, an optional message string) and is followed by a matcher call; an assertion with `.resolves` or `.rejects` is awaited or returned. The fix adds `await` to an unawaited `.resolves`/`.rejects` assertion inside an `async` test; the other mistakes are reported for a hand edit. The Playwright Test counterpart of `testing/valid-expect`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** `expect(x)` with no matcher, `expect(x).toBe` without the call, or an unawaited `.resolves` all pass without checking anything.

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
  any:
    - { matches: expect-call, all: [{ has: { field: function, regex: '^expect$' } }, { has: { field: arguments, not: { has: { nthChild: 1 } } } }] }
    - { matches: expect-call, all: [{ has: { field: function, regex: '^expect$' } }, { has: { field: arguments, any: [{ has: { nthChild: 3 } }, { has: { nthChild: 2, not: { any: [{ kind: string }, { kind: template_string }] } } }] } }] }
    - { kind: expression_statement, has: { nthChild: 1, matches: expect-chain } }
    - kind: call_expression
      pattern: $AW
      inside: { kind: expression_statement }
      has: { field: function, kind: member_expression, regex: '\.(resolves|rejects)\.', has: { field: object, matches: expect-chain } }
      all:
        - inside: { stopBy: end, matches: test-fn, regex: '^async' }
message: "This `expect` does not assert, or is not awaited"
fix:
  - insert-before: { target: $AW, text: 'await ' }
```

## Examples

### JavaScript

#### Don't

```js
test("loads", async () => {
  expect(load()).resolves.toBe(1);
});
```

#### Do

```js
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### JSX

#### Don't

```jsx
test("loads", async () => {
  expect(load()).resolves.toBe(1);
});
```

#### Do

```jsx
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### TypeScript

#### Don't

```ts
test("loads", async () => {
  expect(load()).resolves.toBe(1);
});
```

#### Do

```ts
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### TSX

#### Don't

```tsx
test("loads", async () => {
  expect(load()).resolves.toBe(1);
});
```

#### Do

```tsx
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```
