---
id: testing/testing-library-no-manual-cleanup
title: "No manual `cleanup`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the import and the hook both go; the author checks no other teardown hangs on them"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-testing-library/no-manual-cleanup"]
since: 0.5.0
---

A test does not import or call Testing Library's `cleanup`: the framework packages run it after each test.

**Why:** Calling it again is redundant at best, and with some runners it unmounts before the test's own `afterEach` runs.

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
  kind: import_specifier
  all:
    - has: { field: name, regex: '^cleanup$' }
    - inside: { stopBy: end, kind: import_statement, has: { field: source, regex: '@testing-library/' } }
message: "Testing Library cleans up by itself"
```

## How to fix

1. Delete the `cleanup` import and the `afterEach(cleanup)` call.

## Examples

### JavaScript

#### Don't

```js
import { cleanup, render } from "@testing-library/react";
afterEach(cleanup);
```

#### Do

```js
import { render } from "@testing-library/react";
```

### JSX

#### Don't

```jsx
import { cleanup, render } from "@testing-library/react";
afterEach(cleanup);
```

#### Do

```jsx
import { render } from "@testing-library/react";
```

### TypeScript

#### Don't

```ts
import { cleanup, render } from "@testing-library/react";
afterEach(cleanup);
```

#### Do

```ts
import { render } from "@testing-library/react";
```

### TSX

#### Don't

```tsx
import { cleanup, render } from "@testing-library/react";
afterEach(cleanup);
```

#### Do

```tsx
import { render } from "@testing-library/react";
```
