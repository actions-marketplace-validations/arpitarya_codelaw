---
id: testing/testing-library-await-async-queries
title: "`await` `findBy` queries"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-testing-library/await-async-queries"]
since: 0.5.0
---

A `findBy…` or `findAllBy…` query, which returns a promise, is awaited. The fix adds `await` inside an `async` function.

**Why:** An unawaited `findBy` is a pending promise: assigning it gives a promise, not the element, and its failure is never reported.

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
    - has: { field: function, regex: '(^|\.)find(All)?By(Role|LabelText|PlaceholderText|Text|DisplayValue|AltText|Title|TestId)$' }
    - any:
      - inside: { kind: expression_statement }
      - inside: { kind: variable_declarator, field: value }
    - any:
      - { pattern: $AW, inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }], regex: '^async' } }
      - not: { inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }], regex: '^async' } }
message: "Await this `findBy` query"
fix:
  - insert-before: { target: $AW, text: 'await ' }
```

## Examples

### JavaScript

#### Don't

```js
test("shows", async () => {
  const title = screen.findByText("Hi");
});
```

#### Do

```js
test("shows", async () => {
  const title = await screen.findByText("Hi");
});
```

### JSX

#### Don't

```jsx
test("shows", async () => {
  const title = screen.findByText("Hi");
});
```

#### Do

```jsx
test("shows", async () => {
  const title = await screen.findByText("Hi");
});
```

### TypeScript

#### Don't

```ts
test("shows", async () => {
  const title = screen.findByText("Hi");
});
```

#### Do

```ts
test("shows", async () => {
  const title = await screen.findByText("Hi");
});
```

### TSX

#### Don't

```tsx
test("shows", async () => {
  const title = screen.findByText("Hi");
});
```

#### Do

```tsx
test("shows", async () => {
  const title = await screen.findByText("Hi");
});
```
