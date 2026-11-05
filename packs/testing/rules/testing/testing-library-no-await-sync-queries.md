---
id: testing/testing-library-no-await-sync-queries
title: "No `await` on `getBy` and `queryBy` queries"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-testing-library/no-await-sync-queries"]
since: 0.5.0
---

`getBy…`, `getAllBy…`, `queryBy…` and `queryAllBy…` are synchronous and are not awaited. The fix drops the `await`.

**Why:** Awaiting a synchronous query suggests it waits for the element, which it does not; `findBy` is the waiting form.

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
  kind: await_expression
  has: { pattern: $C, kind: call_expression, has: { field: function, regex: '(^|\.)(get|getAll|query|queryAll)By(Role|LabelText|PlaceholderText|Text|DisplayValue|AltText|Title|TestId)$' } }
message: "This query is synchronous; do not await it"
fix: '$C'
```

## Examples

### JavaScript

#### Don't

```js
test("shows", async () => {
  const t = await screen.getByText("Hi");
});
```

#### Do

```js
test("shows", async () => {
  const t = screen.getByText("Hi");
});
```

### JSX

#### Don't

```jsx
test("shows", async () => {
  const t = await screen.getByText("Hi");
});
```

#### Do

```jsx
test("shows", async () => {
  const t = screen.getByText("Hi");
});
```

### TypeScript

#### Don't

```ts
test("shows", async () => {
  const t = await screen.getByText("Hi");
});
```

#### Do

```ts
test("shows", async () => {
  const t = screen.getByText("Hi");
});
```

### TSX

#### Don't

```tsx
test("shows", async () => {
  const t = await screen.getByText("Hi");
});
```

#### Do

```tsx
test("shows", async () => {
  const t = screen.getByText("Hi");
});
```
