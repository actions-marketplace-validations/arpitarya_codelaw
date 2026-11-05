---
id: testing/no-mocks-import
title: "No imports from `__mocks__`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the module is mocked with `jest.mock` and imported by its real path; the import changes shape"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-jest/no-mocks-import", "@vitest/eslint-plugin/no-mocks-import"]
since: 0.5.0
---

A test does not import from a `__mocks__` folder; it imports the real module and lets `jest.mock` (or `vi.mock`) substitute the mock.

**Why:** Importing the mock directly gives a second copy of it, separate from the one the mocked module returns, so state set on one is not seen by the code under test.

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
  any:
    - { kind: string, regex: '(^["''`]|/)__mocks__(/|["''`]$)', inside: { kind: import_statement } }
    - { kind: string, regex: '(^["''`]|/)__mocks__(/|["''`]$)', inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^require$' } } } }
message: "Import the real module and mock it with `jest.mock`"
```

## How to fix

1. Import the module by its real path.
2. Call `jest.mock("<path>")` (or `vi.mock`) so the mock is used.

## Examples

### JavaScript

#### Don't

```js
import { api } from "../__mocks__/api";
```

#### Do

```js
import { api } from "../api";
jest.mock("../api");
```

### JSX

#### Don't

```jsx
import { api } from "../__mocks__/api";
```

#### Do

```jsx
import { api } from "../api";
jest.mock("../api");
```

### TypeScript

#### Don't

```ts
import { api } from "../__mocks__/api";
```

#### Do

```ts
import { api } from "../api";
jest.mock("../api");
```

### TSX

#### Don't

```tsx
import { api } from "../__mocks__/api";
```

#### Do

```tsx
import { api } from "../api";
jest.mock("../api");
```
