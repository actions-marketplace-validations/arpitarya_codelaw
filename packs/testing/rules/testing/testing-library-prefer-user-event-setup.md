---
id: testing/testing-library-prefer-user-event-setup
title: "`userEvent.setup()` before user-event calls"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the calls move onto an instance created at the start of the test"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-testing-library/prefer-user-event-setup"]
since: 0.5.0
---

user-event's actions are called on an instance from `userEvent.setup()`, not on `userEvent` directly.

**Why:** The direct calls each start a fresh session, so held keys and pointer state do not carry from one call to the next as they do for a real user.

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
  has: { field: function, regex: '^userEvent\.(click|dblClick|tripleClick|hover|unhover|tab|type|clear|selectOptions|deselectOptions|upload|paste|keyboard|pointer|copy|cut)$' }
message: "Call user-event through `userEvent.setup()`"
```

## How to fix

1. Add `const user = userEvent.setup()` at the start of the test and call `user.click(…)` and the rest on it.

## Examples

### JavaScript

#### Don't

```js
test("t", async () => {
  await userEvent.click(button);
});
```

#### Do

```js
test("t", async () => {
  const user = userEvent.setup();
  await user.click(button);
});
```

### JSX

#### Don't

```jsx
test("t", async () => {
  await userEvent.click(button);
});
```

#### Do

```jsx
test("t", async () => {
  const user = userEvent.setup();
  await user.click(button);
});
```

### TypeScript

#### Don't

```ts
test("t", async () => {
  await userEvent.click(button);
});
```

#### Do

```ts
test("t", async () => {
  const user = userEvent.setup();
  await user.click(button);
});
```

### TSX

#### Don't

```tsx
test("t", async () => {
  await userEvent.click(button);
});
```

#### Do

```tsx
test("t", async () => {
  const user = userEvent.setup();
  await user.click(button);
});
```
