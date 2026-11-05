---
id: testing/testing-library-no-global-regexp-flag-in-query
title: "No `g` flag on a regular expression in a query"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-testing-library/no-global-regexp-flag-in-query"]
since: 0.5.0
---

A regular expression passed to a query has no `g` flag: `getByText(/save/i)`, not `/save/gi`. The fix drops the `g` (for the flag sets `g`, `gi`, `gm`, `gu` and their reverses; others are a hand edit).

**Why:** A global regular expression keeps `lastIndex` between calls, so the same query alternately matches and fails.

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
  kind: regex_flags
  regex: 'g'
  inside: { kind: regex, inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '(^|\.)(get|getAll|query|queryAll|find|findAll)By(Role|LabelText|PlaceholderText|Text|DisplayValue|AltText|Title|TestId)$' } } } }
  any:
    - { regex: '^g$', pattern: $G0 }
    - { regex: '^(gi|ig)$', pattern: $G1 }
    - { regex: '^(gm|mg)$', pattern: $G2 }
    - { regex: '^(gu|ug)$', pattern: $G3 }
    - { not: { regex: '^(g|gi|ig|gm|mg|gu|ug)$' } }
message: "Drop the `g` flag"
fix:
  - remove: $G0
  - replace: { target: $G1, with: i }
  - replace: { target: $G2, with: m }
  - replace: { target: $G3, with: u }
```

## Examples

### JavaScript

#### Don't

```js
test("t", () => {
  screen.getByText(/save/gi);
});
```

#### Do

```js
test("t", () => {
  screen.getByText(/save/i);
});
```

### JSX

#### Don't

```jsx
test("t", () => {
  screen.getByText(/save/gi);
});
```

#### Do

```jsx
test("t", () => {
  screen.getByText(/save/i);
});
```

### TypeScript

#### Don't

```ts
test("t", () => {
  screen.getByText(/save/gi);
});
```

#### Do

```ts
test("t", () => {
  screen.getByText(/save/i);
});
```

### TSX

#### Don't

```tsx
test("t", () => {
  screen.getByText(/save/gi);
});
```

#### Do

```tsx
test("t", () => {
  screen.getByText(/save/i);
});
```
