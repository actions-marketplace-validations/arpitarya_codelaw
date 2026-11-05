---
id: testing/no-importing-vitest-globals
title: "No imports of Vitest's globals"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["@vitest/eslint-plugin/no-importing-vitest-globals"]
since: 0.5.0
---

With Vitest's `globals: true`, a test file does not import `describe`, `it`, `test`, `expect`, `vi` or the hooks from `vitest`: the import is removed when it names only those. Opt-in, for projects that use globals.

**Why:** The import is dead weight under globals and makes files inconsistent with those that rely on them.

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
  kind: import_statement
  all:
    - has: { field: source, regex: '^["'']vitest["'']$' }
    - has:
        kind: import_clause
        has:
          kind: named_imports
          not: { has: { kind: import_specifier, not: { regex: '^(describe|it|test|expect|vi|beforeAll|beforeEach|afterAll|afterEach|suite|assert|expectTypeOf|assertType)$' } } }
message: "Vitest's globals need no import"
fix: [remove]
```

## Examples

### JavaScript

#### Don't

```js
import { describe, expect, it } from "vitest";
it("t", () => {});
```

#### Do

```js
it("t", () => {});
```

### JSX

#### Don't

```jsx
import { describe, expect, it } from "vitest";
it("t", () => {});
```

#### Do

```jsx
it("t", () => {});
```

### TypeScript

#### Don't

```ts
import { describe, expect, it } from "vitest";
it("t", () => {});
```

#### Do

```ts
it("t", () => {});
```

### TSX

#### Don't

```tsx
import { describe, expect, it } from "vitest";
it("t", () => {});
```

#### Do

```tsx
it("t", () => {});
```
