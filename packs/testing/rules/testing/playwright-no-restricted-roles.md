---
id: testing/playwright-no-restricted-roles
title: "No `getByRole` roles the project restricts"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the role that fits is the author's to choose"
fix_exception: value
stability: preview
options: { roles: { type: "string[]", default: [] } }
ported_from: ["eslint-plugin-playwright/no-restricted-roles"]
since: 0.5.0
---

`getByRole("<role>")` with a role the project lists in `roles` is reported: `roles = ["presentation", "generic"]`. The list is empty by default.

**Why:** Some roles say nothing to assistive technology; a project bans them so tests target what users can perceive.

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
  kind: string
  pattern: $R
  nthChild: 1
  inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^getByRole$' } } } }
where:
  $R: { in: roles }
message: "The role $R is restricted in this project"
```

## How to fix

1. Locate the element by a meaningful role, or by its label.

## Examples

### JavaScript

#### Don't

```js options={"roles":["presentation"]}
test("t", async ({ page }) => {
  await page.getByRole("presentation").click();
});
```

#### Do

```js options={"roles":["presentation"]}
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Close" }).click();
});
```

### JSX

#### Don't

```jsx options={"roles":["presentation"]}
test("t", async ({ page }) => {
  await page.getByRole("presentation").click();
});
```

#### Do

```jsx options={"roles":["presentation"]}
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Close" }).click();
});
```

### TypeScript

#### Don't

```ts options={"roles":["presentation"]}
test("t", async ({ page }) => {
  await page.getByRole("presentation").click();
});
```

#### Do

```ts options={"roles":["presentation"]}
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Close" }).click();
});
```

### TSX

#### Don't

```tsx options={"roles":["presentation"]}
test("t", async ({ page }) => {
  await page.getByRole("presentation").click();
});
```

#### Do

```tsx options={"roles":["presentation"]}
test("t", async ({ page }) => {
  await page.getByRole("button", { name: "Close" }).click();
});
```
