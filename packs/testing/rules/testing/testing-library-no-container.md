---
id: testing/testing-library-no-container
title: "No DOM queries through `container`"
languages: [javascript, jsx, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the query becomes a Testing Library query by role, label or text, which needs knowing what the user sees"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-testing-library/no-container"]
since: 0.5.0
---

A test does not query the DOM through the render result's `container` (`container.querySelector(…)`); it uses Testing Library's queries.

**Why:** `container` queries test markup structure, not what the user sees, so they break on harmless refactors and miss accessibility regressions.

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
  all:
    - has: { field: function, kind: member_expression, regex: '(^|\.)container\.(querySelector|querySelectorAll|getElementsByClassName|getElementsByTagName|getElementById)$' }
    - inside: { stopBy: end, kind: program, has: { kind: import_statement, has: { field: source, regex: '@testing-library/' } } }
message: "Query with Testing Library, not through `container`"
```

## How to fix

1. Replace the selector with a query on what the user sees: `screen.getByRole("button", { name: "Save" })`.

## Examples

### JavaScript

#### Don't

```js
import { render } from "@testing-library/react";
test("t", () => {
  const { container } = render(<App />);
  container.querySelector(".save");
});
```

#### Do

```js
import { render, screen } from "@testing-library/react";
test("t", () => {
  render(<App />);
  screen.getByRole("button", { name: "Save" });
});
```

### JSX

#### Don't

```jsx
import { render } from "@testing-library/react";
test("t", () => {
  const { container } = render(<App />);
  container.querySelector(".save");
});
```

#### Do

```jsx
import { render, screen } from "@testing-library/react";
test("t", () => {
  render(<App />);
  screen.getByRole("button", { name: "Save" });
});
```

### TSX

#### Don't

```tsx
import { render } from "@testing-library/react";
test("t", () => {
  const { container } = render(<App />);
  container.querySelector(".save");
});
```

#### Do

```tsx
import { render, screen } from "@testing-library/react";
test("t", () => {
  render(<App />);
  screen.getByRole("button", { name: "Save" });
});
```
