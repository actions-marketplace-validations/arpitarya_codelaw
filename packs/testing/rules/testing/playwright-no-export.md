---
id: testing/playwright-no-export
title: "No exports from a test file"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the exported code moves to a module of its own, and its importers change"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/no-export"]
since: 0.5.0
---

A file that declares tests exports nothing. The Playwright Test counterpart of `testing/no-export`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** Importing a test file from another runs its tests again there; shared helpers belong in a module of their own.

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
  kind: export_statement
  inside: { kind: program, has: { stopBy: end, matches: test-call } }
message: "A test file should not export; move this to a helper module"
```

## How to fix

1. Move the exported helper to its own module (`test-utils.ts`).
2. Import it from there in every test file that uses it.

## Examples

### JavaScript

#### Don't

```js
export const user = { id: 1 };
test("a", () => {});
```

#### Do

```js
const user = { id: 1 };
test("a", () => {});
```

### JSX

#### Don't

```jsx
export const user = { id: 1 };
test("a", () => {});
```

#### Do

```jsx
const user = { id: 1 };
test("a", () => {});
```

### TypeScript

#### Don't

```ts
export const user = { id: 1 };
test("a", () => {});
```

#### Do

```ts
const user = { id: 1 };
test("a", () => {});
```

### TSX

#### Don't

```tsx
export const user = { id: 1 };
test("a", () => {});
```

#### Do

```tsx
const user = { id: 1 };
test("a", () => {});
```
