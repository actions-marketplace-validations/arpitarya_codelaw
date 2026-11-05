---
id: testing/playwright-prefer-lowercase-title
title: "Test titles start in lower case"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-playwright/prefer-lowercase-title"]
since: 0.5.0
---

A test or `describe` title starts with a lower-case letter: `test("adds")`, not `test("Adds")`. A title that starts with an acronym (`"API returns"`, two capitals) is left alone. The fix lowers the first letter. The Playwright Test counterpart of `testing/prefer-lowercase-title`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** Titles read as a sentence after the block name in reports (`cart › adds an item`); one style keeps them readable.

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
  kind: string_fragment
  regex: '^[A-Z]([^A-Z]|$)'
  pattern: $T
  inside: { kind: string, nthChild: 1, inside: { kind: arguments, inside: { any: [{ matches: test-call }, { matches: describe-call }] } } }
message: "Start the title in lower case"
fix: '$T|lower-first'
```

## Examples

### JavaScript

#### Don't

```js
describe("Cart", () => {
  it("Adds an item", () => {});
});
```

#### Do

```js
describe("cart", () => {
  it("adds an item", () => {});
});
```

### JSX

#### Don't

```jsx
describe("Cart", () => {
  it("Adds an item", () => {});
});
```

#### Do

```jsx
describe("cart", () => {
  it("adds an item", () => {});
});
```

### TypeScript

#### Don't

```ts
describe("Cart", () => {
  it("Adds an item", () => {});
});
```

#### Do

```ts
describe("cart", () => {
  it("adds an item", () => {});
});
```

### TSX

#### Don't

```tsx
describe("Cart", () => {
  it("Adds an item", () => {});
});
```

#### Do

```tsx
describe("cart", () => {
  it("adds an item", () => {});
});
```
