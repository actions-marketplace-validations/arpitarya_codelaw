---
id: testing/testing-library-no-node-access
title: "No DOM traversal in Testing Library tests"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "the traversal becomes a query by role, label or text, which needs knowing what the user sees"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-testing-library/no-node-access"]
since: 0.5.0
---

In a file that imports Testing Library, a test does not walk the DOM — `.parentElement`, `.children`, `.firstChild`, `.closest(…)`, `.querySelector(…)` and their kin; it queries.

**Why:** Walking the DOM ties the test to markup structure; a harmless wrapper `div` breaks it, and it checks nothing a user perceives.

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
  kind: property_identifier
  regex: '^(parentElement|parentNode|children|childNodes|firstChild|lastChild|firstElementChild|lastElementChild|nextSibling|previousSibling|nextElementSibling|previousElementSibling|closest|querySelector|querySelectorAll)$'
  all:
    - inside: { kind: member_expression, field: property }
    - inside: { stopBy: end, kind: program, has: { kind: import_statement, has: { field: source, regex: '@testing-library/' } } }
message: "Query the element instead of walking the DOM"
```

## How to fix

1. Replace the traversal with a query: `within(row).getByRole("cell", { name: "Total" })`.

## Examples

### JavaScript

#### Don't

```js
import { screen } from "@testing-library/react";
test("t", () => {
  screen.getByText("Total").parentElement;
});
```

#### Do

```js
import { screen, within } from "@testing-library/react";
test("t", () => {
  within(screen.getByRole("row")).getByText("Total");
});
```

### JSX

#### Don't

```jsx
import { screen } from "@testing-library/react";
test("t", () => {
  screen.getByText("Total").parentElement;
});
```

#### Do

```jsx
import { screen, within } from "@testing-library/react";
test("t", () => {
  within(screen.getByRole("row")).getByText("Total");
});
```

### TypeScript

#### Don't

```ts
import { screen } from "@testing-library/react";
test("t", () => {
  screen.getByText("Total").parentElement;
});
```

#### Do

```ts
import { screen, within } from "@testing-library/react";
test("t", () => {
  within(screen.getByRole("row")).getByText("Total");
});
```

### TSX

#### Don't

```tsx
import { screen } from "@testing-library/react";
test("t", () => {
  screen.getByText("Total").parentElement;
});
```

#### Do

```tsx
import { screen, within } from "@testing-library/react";
test("t", () => {
  within(screen.getByRole("row")).getByText("Total");
});
```
