---
id: testing/testing-library-await-async-events
title: "`await` user-event calls"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-testing-library/await-async-events"]
since: 0.5.0
---

A `userEvent.<action>(…)` call — asynchronous since user-event 14 — is awaited. The fix adds `await` inside an `async` function; elsewhere the finding is a hand edit (make the function `async`). `userEvent.setup()` is synchronous and left alone.

**Why:** Without `await` the assertion after the event runs before the event has been dispatched, so it checks the old state.

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
    - has: { field: function, regex: '^userEvent\.(click|dblClick|tripleClick|hover|unhover|tab|type|clear|selectOptions|deselectOptions|upload|paste|keyboard|pointer|copy|cut)$' }
    - any:
      - inside: { kind: expression_statement }
      - inside: { kind: variable_declarator, field: value }
    - any:
      - { pattern: $AW, inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }], regex: '^async' } }
      - not: { inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }], regex: '^async' } }
message: "Await this user-event call"
fix:
  - insert-before: { target: $AW, text: 'await ' }
```

## Examples

### JavaScript

#### Don't

```js
test("submits", async () => {
  userEvent.click(button);
  expect(onSubmit).toHaveBeenCalled();
});
```

#### Do

```js
test("submits", async () => {
  await userEvent.click(button);
  expect(onSubmit).toHaveBeenCalled();
});
```

### JSX

#### Don't

```jsx
test("submits", async () => {
  userEvent.click(button);
  expect(onSubmit).toHaveBeenCalled();
});
```

#### Do

```jsx
test("submits", async () => {
  await userEvent.click(button);
  expect(onSubmit).toHaveBeenCalled();
});
```

### TypeScript

#### Don't

```ts
test("submits", async () => {
  userEvent.click(button);
  expect(onSubmit).toHaveBeenCalled();
});
```

#### Do

```ts
test("submits", async () => {
  await userEvent.click(button);
  expect(onSubmit).toHaveBeenCalled();
});
```

### TSX

#### Don't

```tsx
test("submits", async () => {
  userEvent.click(button);
  expect(onSubmit).toHaveBeenCalled();
});
```

#### Do

```tsx
test("submits", async () => {
  await userEvent.click(button);
  expect(onSubmit).toHaveBeenCalled();
});
```
