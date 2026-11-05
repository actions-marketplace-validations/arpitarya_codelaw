---
id: testing/hoisted-apis-on-top
title: "`vi.mock` and `vi.hoisted` at the top level"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "the call already runs first; moving it changes nothing at run time but may move code the author placed for a reason"
stability: preview
ported_from: ["@vitest/eslint-plugin/hoisted-apis-on-top"]
since: 0.5.0
---

`vi.mock`, `vi.unmock` and `vi.hoisted` are called at the top level of the file, not inside a function or block. The suggested edit moves the call after the imports.

**Why:** Vitest hoists these calls to the top of the file whatever their position; written inside a test they read as if they ran there.

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
  kind: expression_statement
  has: { kind: call_expression, has: { field: function, regex: '^vi\.(mock|unmock|hoisted)$' } }
  inside: { stopBy: end, any: [{ kind: arrow_function }, { kind: function_expression }, { kind: function_declaration }, { kind: statement_block }] }
message: "`vi.mock` is hoisted; call it at the top level"
fix:
  - move-to: after-imports
```

## Examples

### JavaScript

#### Don't

```js
import { api } from "./api";
test("t", () => {
  vi.mock("./api");
});
```

#### Do

```js
import { api } from "./api";
vi.mock("./api");
test("t", () => {
});
```

### JSX

#### Don't

```jsx
import { api } from "./api";
test("t", () => {
  vi.mock("./api");
});
```

#### Do

```jsx
import { api } from "./api";
vi.mock("./api");
test("t", () => {
});
```

### TypeScript

#### Don't

```ts
import { api } from "./api";
test("t", () => {
  vi.mock("./api");
});
```

#### Do

```ts
import { api } from "./api";
vi.mock("./api");
test("t", () => {
});
```

### TSX

#### Don't

```tsx
import { api } from "./api";
test("t", () => {
  vi.mock("./api");
});
```

#### Do

```tsx
import { api } from "./api";
vi.mock("./api");
test("t", () => {
});
```
