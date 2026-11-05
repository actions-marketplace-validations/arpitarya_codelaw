---
id: testing/testing-library-prefer-screen-queries
title: "Queries through `screen`"
languages: [javascript, jsx, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "the render result's queries are bound to its container, which `screen` is not when `render` was given another one"
stability: preview
ported_from: ["eslint-plugin-testing-library/prefer-screen-queries"]
since: 0.5.0
---

A test queries through `screen` (`screen.getByText(…)`), not through queries destructured from `render`. The suggested edit adds `screen.` when the file imports `screen`; otherwise it is a hand edit.

**Why:** `screen` is always the document; the destructured queries make each test carry the render result around and are easy to take from the wrong render.

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
  kind: identifier
  regex: '^(get|getAll|query|queryAll|find|findAll)By(Role|LabelText|PlaceholderText|Text|DisplayValue|AltText|Title|TestId)$'
  all:
    - inside: { kind: call_expression, field: function }
    - inside: { stopBy: end, kind: program, has: { kind: import_statement, has: { field: source, regex: '@testing-library/' } } }
    - any:
      - { pattern: $Q, inside: { stopBy: end, kind: program, has: { kind: import_statement, has: { stopBy: end, kind: import_specifier, regex: '^screen$' } } } }
      - not: { inside: { stopBy: end, kind: program, has: { kind: import_statement, has: { stopBy: end, kind: import_specifier, regex: '^screen$' } } } }
message: "Query through `screen`"
fix:
  - insert-before: { target: $Q, text: 'screen.' }
```

## Examples

### JavaScript

#### Don't

```js
import { render, screen } from "@testing-library/react";
test("t", () => {
  const { getByText } = render(<App />);
  getByText("Hi");
});
```

#### Do

```js
import { render, screen } from "@testing-library/react";
test("t", () => {
  const { getByText } = render(<App />);
  screen.getByText("Hi");
});
```

### JSX

#### Don't

```jsx
import { render, screen } from "@testing-library/react";
test("t", () => {
  const { getByText } = render(<App />);
  getByText("Hi");
});
```

#### Do

```jsx
import { render, screen } from "@testing-library/react";
test("t", () => {
  const { getByText } = render(<App />);
  screen.getByText("Hi");
});
```

### TSX

#### Don't

```tsx
import { render, screen } from "@testing-library/react";
test("t", () => {
  const { getByText } = render(<App />);
  getByText("Hi");
});
```

#### Do

```tsx
import { render, screen } from "@testing-library/react";
test("t", () => {
  const { getByText } = render(<App />);
  screen.getByText("Hi");
});
```
