---
id: testing/testing-library-prefer-user-event
title: "`userEvent` over `fireEvent`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "`userEvent` calls are async and need a `setup()` instance, so the test changes shape"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-testing-library/prefer-user-event"]
since: 0.5.0
---

A test simulates input with `userEvent` (`await user.click(…)`), not `fireEvent`. Opt-in.

**Why:** `fireEvent` dispatches one DOM event; `userEvent` plays the whole interaction a user makes (focus, pointer, keyboard), so it catches what a single event misses.

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
  has: { field: function, regex: '^fireEvent\.(click|dblClick|change|input|keyDown|keyUp|keyPress|mouseOver|mouseOut|mouseEnter|mouseLeave|focus|blur|select|paste|copy|cut)$' }
message: "Use `userEvent` instead of `fireEvent`"
```

## How to fix

1. Create `const user = userEvent.setup()` at the start of the test, make it `async`, and replace `fireEvent.click(x)` with `await user.click(x)`.

## Examples

### JavaScript

#### Don't

```js
test("t", () => {
  fireEvent.click(button);
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
test("t", () => {
  fireEvent.click(button);
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
test("t", () => {
  fireEvent.click(button);
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
test("t", () => {
  fireEvent.click(button);
});
```

#### Do

```tsx
test("t", async () => {
  const user = userEvent.setup();
  await user.click(button);
});
```
