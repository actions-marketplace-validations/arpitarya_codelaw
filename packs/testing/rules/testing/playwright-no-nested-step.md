---
id: testing/playwright-no-nested-step
title: "No `test.step` inside a step"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "flattening steps changes how the report groups them"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-nested-step"]
since: 0.5.0
---

`test.step(…)` is not called inside another step's callback.

**Why:** Nested steps make the report a tree that is hard to read; flat steps read as the test's script.

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
    - has: { field: function, regex: '^test\.step$' }
    - inside: { stopBy: end, kind: call_expression, has: { field: function, regex: '^test\.step$' } }
message: "Do not nest steps"
```

## How to fix

1. Move the inner step out to the same level as its parent.

## Examples

### JavaScript

#### Don't

```js
test("t", async () => {
  await test.step("a", async () => {
    await test.step("b", async () => {});
  });
});
```

#### Do

```js
test("t", async () => {
  await test.step("a", async () => {});
  await test.step("b", async () => {});
});
```

### JSX

#### Don't

```jsx
test("t", async () => {
  await test.step("a", async () => {
    await test.step("b", async () => {});
  });
});
```

#### Do

```jsx
test("t", async () => {
  await test.step("a", async () => {});
  await test.step("b", async () => {});
});
```

### TypeScript

#### Don't

```ts
test("t", async () => {
  await test.step("a", async () => {
    await test.step("b", async () => {});
  });
});
```

#### Do

```ts
test("t", async () => {
  await test.step("a", async () => {});
  await test.step("b", async () => {});
});
```

### TSX

#### Don't

```tsx
test("t", async () => {
  await test.step("a", async () => {
    await test.step("b", async () => {});
  });
});
```

#### Do

```tsx
test("t", async () => {
  await test.step("a", async () => {});
  await test.step("b", async () => {});
});
```
