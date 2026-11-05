---
id: testing/testing-library-render-result-naming-convention
title: "The render result is called `view` or destructured"
languages: [javascript, jsx, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: suggest
fix_reason: "renaming the variable is local, but the author may prefer to destructure it"
stability: preview
ported_from: ["eslint-plugin-testing-library/render-result-naming-convention"]
since: 0.5.0
---

The value `render(…)` returns is named `view` or `utils`, or destructured. The suggested edit renames it to `view` throughout the file.

**Why:** A name like `wrapper` or `component` suggests it is the component; `view` says it is the rendered output.

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
  not: { regex: '^(view|utils)$' }
  all:
    - inside: { kind: variable_declarator, field: name, has: { field: value, kind: call_expression, has: { field: function, regex: '^(render|renderHook)$' } } }
    - inside: { stopBy: end, kind: program, has: { kind: import_statement, has: { field: source, regex: '@testing-library/' } } }
message: "Name the render result `view`, or destructure it"
fix:
  - rename-symbol-local: { target: $$MATCH, to: view }
```

## Examples

### JavaScript

#### Don't

```js
import { render } from "@testing-library/react";
test("t", () => {
  const wrapper = render(<App />);
  wrapper.unmount();
});
```

#### Do

```js
import { render } from "@testing-library/react";
test("t", () => {
  const view = render(<App />);
  view.unmount();
});
```

### JSX

#### Don't

```jsx
import { render } from "@testing-library/react";
test("t", () => {
  const wrapper = render(<App />);
  wrapper.unmount();
});
```

#### Do

```jsx
import { render } from "@testing-library/react";
test("t", () => {
  const view = render(<App />);
  view.unmount();
});
```

### TSX

#### Don't

```tsx
import { render } from "@testing-library/react";
test("t", () => {
  const wrapper = render(<App />);
  wrapper.unmount();
});
```

#### Do

```tsx
import { render } from "@testing-library/react";
test("t", () => {
  const view = render(<App />);
  view.unmount();
});
```
