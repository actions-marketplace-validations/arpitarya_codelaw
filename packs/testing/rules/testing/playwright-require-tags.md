---
id: testing/playwright-require-tags
title: "Every test is tagged"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "which tag fits the test is the author's choice"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-playwright/require-tags"]
since: 0.5.0
---

Every test carries a tag, in its `{ tag: … }` details or as `@tag` in its title. Opt-in.

**Why:** Tags select what CI runs (`--grep @smoke`); an untagged test is run by none of the selections.

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
  matches: test-call
  all:
    - has: { field: arguments, has: { nthChild: 1, kind: string, not: { regex: '@\w' } } }
    - not: { has: { field: arguments, has: { kind: object, has: { kind: pair, has: { field: key, regex: '^tag$' } } } } }
message: "Tag this test"
```

## How to fix

1. Add a tag: `test("…", { tag: "@smoke" }, async () => { … })`.

## Examples

### JavaScript

#### Don't

```js
test("logs in", async () => {});
```

#### Do

```js
test("logs in", { tag: "@smoke" }, async () => {});
```

### JSX

#### Don't

```jsx
test("logs in", async () => {});
```

#### Do

```jsx
test("logs in", { tag: "@smoke" }, async () => {});
```

### TypeScript

#### Don't

```ts
test("logs in", async () => {});
```

#### Do

```ts
test("logs in", { tag: "@smoke" }, async () => {});
```

### TSX

#### Don't

```tsx
test("logs in", async () => {});
```

#### Do

```tsx
test("logs in", { tag: "@smoke" }, async () => {});
```
