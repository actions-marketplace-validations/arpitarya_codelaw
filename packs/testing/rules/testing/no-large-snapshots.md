---
id: testing/no-large-snapshots
title: "No inline snapshot longer than 50 lines"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "what to assert instead of the whole value is the author's choice"
fix_exception: authored
stability: preview
ported_from: ["eslint-plugin-jest/no-large-snapshots", "@vitest/eslint-plugin/no-large-snapshots"]
since: 0.5.0
---

An inline snapshot is at most 50 lines long. (The source's default; it also checks `.snap` files, which this rule does not read.)

**Why:** Nobody reviews a 200-line snapshot; it gets updated with `-u` whatever changed, and stops testing anything.

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
  kind: template_string
  regex: '(\n[^\n]*){50,}'
  inside:
    kind: arguments
    inside: { matches: matcher-call, has: { field: function, has: { field: property, regex: '^(toMatchInlineSnapshot|toThrowErrorMatchingInlineSnapshot)$' } } }
message: "This inline snapshot is over 50 lines; assert on the part that matters"
```

## How to fix

1. Assert on the few fields the test is about (`toMatchObject`, `toHaveProperty`).
2. Or snapshot a smaller piece of the value.

## Examples

### JavaScript

#### Don't

```js
test("big", () => {
  expect(page).toMatchInlineSnapshot(`  line 0
  line 1
  line 2
  line 3
  line 4
  line 5
  line 6
  line 7
  line 8
  line 9
  line 10
  line 11
  line 12
  line 13
  line 14
  line 15
  line 16
  line 17
  line 18
  line 19
  line 20
  line 21
  line 22
  line 23
  line 24
  line 25
  line 26
  line 27
  line 28
  line 29
  line 30
  line 31
  line 32
  line 33
  line 34
  line 35
  line 36
  line 37
  line 38
  line 39
  line 40
  line 41
  line 42
  line 43
  line 44
  line 45
  line 46
  line 47
  line 48
  line 49
  line 50
  line 51
`);
});
```

#### Do

```js
test("big", () => {
  expect(page.title).toMatchInlineSnapshot(`"Home"`);
});
```

### JSX

#### Don't

```jsx
test("big", () => {
  expect(page).toMatchInlineSnapshot(`  line 0
  line 1
  line 2
  line 3
  line 4
  line 5
  line 6
  line 7
  line 8
  line 9
  line 10
  line 11
  line 12
  line 13
  line 14
  line 15
  line 16
  line 17
  line 18
  line 19
  line 20
  line 21
  line 22
  line 23
  line 24
  line 25
  line 26
  line 27
  line 28
  line 29
  line 30
  line 31
  line 32
  line 33
  line 34
  line 35
  line 36
  line 37
  line 38
  line 39
  line 40
  line 41
  line 42
  line 43
  line 44
  line 45
  line 46
  line 47
  line 48
  line 49
  line 50
  line 51
`);
});
```

#### Do

```jsx
test("big", () => {
  expect(page.title).toMatchInlineSnapshot(`"Home"`);
});
```

### TypeScript

#### Don't

```ts
test("big", () => {
  expect(page).toMatchInlineSnapshot(`  line 0
  line 1
  line 2
  line 3
  line 4
  line 5
  line 6
  line 7
  line 8
  line 9
  line 10
  line 11
  line 12
  line 13
  line 14
  line 15
  line 16
  line 17
  line 18
  line 19
  line 20
  line 21
  line 22
  line 23
  line 24
  line 25
  line 26
  line 27
  line 28
  line 29
  line 30
  line 31
  line 32
  line 33
  line 34
  line 35
  line 36
  line 37
  line 38
  line 39
  line 40
  line 41
  line 42
  line 43
  line 44
  line 45
  line 46
  line 47
  line 48
  line 49
  line 50
  line 51
`);
});
```

#### Do

```ts
test("big", () => {
  expect(page.title).toMatchInlineSnapshot(`"Home"`);
});
```

### TSX

#### Don't

```tsx
test("big", () => {
  expect(page).toMatchInlineSnapshot(`  line 0
  line 1
  line 2
  line 3
  line 4
  line 5
  line 6
  line 7
  line 8
  line 9
  line 10
  line 11
  line 12
  line 13
  line 14
  line 15
  line 16
  line 17
  line 18
  line 19
  line 20
  line 21
  line 22
  line 23
  line 24
  line 25
  line 26
  line 27
  line 28
  line 29
  line 30
  line 31
  line 32
  line 33
  line 34
  line 35
  line 36
  line 37
  line 38
  line 39
  line 40
  line 41
  line 42
  line 43
  line 44
  line 45
  line 46
  line 47
  line 48
  line 49
  line 50
  line 51
`);
});
```

#### Do

```tsx
test("big", () => {
  expect(page.title).toMatchInlineSnapshot(`"Home"`);
});
```
