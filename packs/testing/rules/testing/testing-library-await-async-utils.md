---
id: testing/testing-library-await-async-utils
title: "`await` `waitFor` and its kin"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-testing-library/await-async-utils"]
since: 0.5.0
---

`waitFor(…)`, `waitForElementToBeRemoved(…)` and `waitForElement(…)` are awaited. The fix adds `await` inside an `async` function.

**Why:** Each returns a promise; without `await` the test moves on before the condition holds, and a timeout is never reported.

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
    - has: { field: function, regex: '^(waitFor|waitForElementToBeRemoved|waitForElement|waitForDomChange)$' }
    - any:
      - inside: { kind: expression_statement }
      - inside: { kind: variable_declarator, field: value }
    - any:
      - { pattern: $AW, inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }], regex: '^async' } }
      - not: { inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }], regex: '^async' } }
message: "Await this async utility"
fix:
  - insert-before: { target: $AW, text: 'await ' }
```

## Examples

### JavaScript

#### Don't

```js
test("loads", async () => {
  waitFor(() => expect(list).toHaveLength(3));
});
```

#### Do

```js
test("loads", async () => {
  await waitFor(() => expect(list).toHaveLength(3));
});
```

### JSX

#### Don't

```jsx
test("loads", async () => {
  waitFor(() => expect(list).toHaveLength(3));
});
```

#### Do

```jsx
test("loads", async () => {
  await waitFor(() => expect(list).toHaveLength(3));
});
```

### TypeScript

#### Don't

```ts
test("loads", async () => {
  waitFor(() => expect(list).toHaveLength(3));
});
```

#### Do

```ts
test("loads", async () => {
  await waitFor(() => expect(list).toHaveLength(3));
});
```

### TSX

#### Don't

```tsx
test("loads", async () => {
  waitFor(() => expect(list).toHaveLength(3));
});
```

#### Do

```tsx
test("loads", async () => {
  await waitFor(() => expect(list).toHaveLength(3));
});
```
