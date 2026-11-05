---
id: testing/require-awaited-expect-poll
title: "`expect.poll` is awaited"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["@vitest/eslint-plugin/require-awaited-expect-poll"]
since: 0.5.0
---

An `expect.poll(…)` assertion (and an `expect.element(…)` one) is awaited or returned. The fix adds `await` inside an `async` function; elsewhere it is reported for a hand edit.

**Why:** `expect.poll` returns a promise; without `await` the test ends before the polling does, and a failure is lost.

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
  pattern: $AW
  inside: { kind: expression_statement }
  all:
    - has: { field: function, kind: member_expression, has: { field: object, stopBy: end, kind: call_expression, has: { field: function, regex: '^expect\.(poll|element)$' } } }
    - inside: { stopBy: end, matches: test-fn, regex: '^async' }
message: "Await `expect.poll`"
fix:
  - insert-before: { target: $AW, text: 'await ' }
```

## Examples

### JavaScript

#### Don't

```js
test("loads", async () => {
  expect.poll(() => count()).toBe(3);
});
```

#### Do

```js
test("loads", async () => {
  await expect.poll(() => count()).toBe(3);
});
```

### JSX

#### Don't

```jsx
test("loads", async () => {
  expect.poll(() => count()).toBe(3);
});
```

#### Do

```jsx
test("loads", async () => {
  await expect.poll(() => count()).toBe(3);
});
```

### TypeScript

#### Don't

```ts
test("loads", async () => {
  expect.poll(() => count()).toBe(3);
});
```

#### Do

```ts
test("loads", async () => {
  await expect.poll(() => count()).toBe(3);
});
```

### TSX

#### Don't

```tsx
test("loads", async () => {
  expect.poll(() => count()).toBe(3);
});
```

#### Do

```tsx
test("loads", async () => {
  await expect.poll(() => count()).toBe(3);
});
```
