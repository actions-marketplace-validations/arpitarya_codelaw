---
id: testing/testing-library-no-promise-in-fire-event
title: "No promise passed to `fireEvent`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "adding `await` needs the function to be `async`; the author confirms the order"
stability: preview
ported_from: ["eslint-plugin-testing-library/no-promise-in-fire-event"]
since: 0.5.0
---

`fireEvent.<event>(…)` is given an element, not a `findBy…` promise; the suggested edit awaits the query.

**Why:** `fireEvent` does not await its argument: given a promise it throws, or fires on nothing.

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
  pattern: $P
  nthChild: 1
  has: { field: function, regex: '(^|\.)find(All)?By(Role|LabelText|PlaceholderText|Text|DisplayValue|AltText|Title|TestId)$' }
  inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^fireEvent(\.\w+)?$' } } }
message: "Await the element before firing the event"
fix: 'await $P'
```

## Examples

### JavaScript

#### Don't

```js
test("t", async () => {
  fireEvent.click(screen.findByText("Save"));
});
```

#### Do

```js
test("t", async () => {
  fireEvent.click(await screen.findByText("Save"));
});
```

### JSX

#### Don't

```jsx
test("t", async () => {
  fireEvent.click(screen.findByText("Save"));
});
```

#### Do

```jsx
test("t", async () => {
  fireEvent.click(await screen.findByText("Save"));
});
```

### TypeScript

#### Don't

```ts
test("t", async () => {
  fireEvent.click(screen.findByText("Save"));
});
```

#### Do

```ts
test("t", async () => {
  fireEvent.click(await screen.findByText("Save"));
});
```

### TSX

#### Don't

```tsx
test("t", async () => {
  fireEvent.click(screen.findByText("Save"));
});
```

#### Do

```tsx
test("t", async () => {
  fireEvent.click(await screen.findByText("Save"));
});
```
