---
id: testing/playwright-max-nested-describe
title: "At most five nested `describe` blocks"
languages: [javascript, jsx, typescript, tsx]
files: ["**/*.test.*", "**/*.spec.*", "**/__tests__/**", "**/test/**", "**/tests/**", "**/e2e/**"]
fix: manual
fix_reason: "flattening the nesting moves tests between blocks and files"
fix_exception: restructure
stability: preview
ported_from: ["eslint-plugin-playwright/max-nested-describe"]
since: 0.5.0
---

`describe` blocks nest at most five deep. (The source's default; the limit is fixed here.) The Playwright Test counterpart of `testing/max-nested-describe`, for `test`, `test.describe` and `test.beforeEach`.

**Why:** Deep nesting spreads one test's setup across many hooks and makes its title a paragraph.

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
  matches: describe-call
  inside: { stopBy: end, all: [{ matches: describe-call, inside: { stopBy: end, all: [{ matches: describe-call, inside: { stopBy: end, all: [{ matches: describe-call, inside: { stopBy: end, all: [{ matches: describe-call, inside: { stopBy: end, all: [{ matches: describe-call }] } }] } }] } }] } }] }
message: "`describe` is nested more than five deep"
```

## How to fix

1. Merge levels that add no setup of their own, or move a deep group to its own file.

## Examples

### JavaScript

#### Don't

```js
describe("a", () => {
  describe("b", () => {
    describe("c", () => {
      describe("d", () => {
        describe("e", () => {
          describe("f", () => {});
        });
      });
    });
  });
});
```

#### Do

```js
describe("a", () => {
  describe("b", () => {
    describe("c", () => {
      describe("d", () => {
        describe("e", () => {});
      });
    });
  });
});
```

### JSX

#### Don't

```jsx
describe("a", () => {
  describe("b", () => {
    describe("c", () => {
      describe("d", () => {
        describe("e", () => {
          describe("f", () => {});
        });
      });
    });
  });
});
```

#### Do

```jsx
describe("a", () => {
  describe("b", () => {
    describe("c", () => {
      describe("d", () => {
        describe("e", () => {});
      });
    });
  });
});
```

### TypeScript

#### Don't

```ts
describe("a", () => {
  describe("b", () => {
    describe("c", () => {
      describe("d", () => {
        describe("e", () => {
          describe("f", () => {});
        });
      });
    });
  });
});
```

#### Do

```ts
describe("a", () => {
  describe("b", () => {
    describe("c", () => {
      describe("d", () => {
        describe("e", () => {});
      });
    });
  });
});
```

### TSX

#### Don't

```tsx
describe("a", () => {
  describe("b", () => {
    describe("c", () => {
      describe("d", () => {
        describe("e", () => {
          describe("f", () => {});
        });
      });
    });
  });
});
```

#### Do

```tsx
describe("a", () => {
  describe("b", () => {
    describe("c", () => {
      describe("d", () => {
        describe("e", () => {});
      });
    });
  });
});
```
