---
id: testing/prefer-expect-resolves
title: "`await expect(p).resolves` over `expect(await p)`"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: auto
stability: preview
ported_from: ["eslint-plugin-jest/prefer-expect-resolves", "@vitest/eslint-plugin/prefer-expect-resolves"]
since: 0.5.0
---

A promise's value is asserted with `await expect(promise).resolves.<matcher>(…)`, not `expect(await promise).<matcher>(…)`.

**Why:** `resolves` keeps the promise inside the assertion, so a rejection fails with the assertion's message, and the form matches `rejects`.

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
  any:
    - all: [{ pattern: "expect(await $P).$M($$$A)" }, { pattern: $W0 }]
    - all: [{ pattern: "expect(await $P).not.$M($$$A)" }, { pattern: $W1 }]
constraints:
  M: { regex: '^to' }
message: "Use `await expect(…).resolves`"
fix:
  - replace: { target: $W0, with: 'await expect($P).resolves.$M($$$A)' }
  - replace: { target: $W1, with: 'await expect($P).resolves.not.$M($$$A)' }
```

## Examples

### JavaScript

#### Don't

```js
test("loads", async () => {
  expect(await load()).toBe(1);
});
```

#### Do

```js
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### JSX

#### Don't

```jsx
test("loads", async () => {
  expect(await load()).toBe(1);
});
```

#### Do

```jsx
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### TypeScript

#### Don't

```ts
test("loads", async () => {
  expect(await load()).toBe(1);
});
```

#### Do

```ts
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```

### TSX

#### Don't

```tsx
test("loads", async () => {
  expect(await load()).toBe(1);
});
```

#### Do

```tsx
test("loads", async () => {
  await expect(load()).resolves.toBe(1);
});
```
