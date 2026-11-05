---
id: testing/testing-library-consistent-data-testid
title: "`data-testid` values follow one pattern"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "a new test id has to be chosen, and every query that names the old one changes"
fix_exception: value
stability: preview
ported_from: ["eslint-plugin-testing-library/consistent-data-testid"]
since: 0.5.0
---

A `data-testid` value is made of letters, digits, `-` and `_` only (a project pattern such as `<component>-<element>` is the source's option and is not configurable here). Runs on every file, since test ids are written in components.

**Why:** Test ids are an API between components and tests; one shape makes them predictable to query and to search for.

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
  kind: jsx_attribute
  all:
    - has: { kind: property_identifier, regex: '^data-testid$' }
    - has: { kind: string, regex: '[^A-Za-z0-9_\-"'']' }
message: "Use letters, digits, `-` and `_` in a test id"
```

## How to fix

1. Rename the test id to the project's pattern, and update every query that names it.

## Examples

### JavaScript

#### Don't

```js
const b = <button data-testid="save button">Save</button>;
```

#### Do

```js
const b = <button data-testid="save-button">Save</button>;
```

### JSX

#### Don't

```jsx
const b = <button data-testid="save button">Save</button>;
```

#### Do

```jsx
const b = <button data-testid="save-button">Save</button>;
```

### TSX

#### Don't

```tsx
const b = <button data-testid="save button">Save</button>;
```

#### Do

```tsx
const b = <button data-testid="save-button">Save</button>;
```
