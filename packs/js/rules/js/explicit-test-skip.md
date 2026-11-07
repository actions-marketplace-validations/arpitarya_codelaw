---
id: js/explicit-test-skip
title: "Skip a test with the framework, not an early return"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a skipped test is reported as skipped instead of passed; that is the point, but the run's totals change"
stability: preview
ported_from: [eslint-plugin-sonarjs/explicit-test-skip]
since: 0.5.0
---

A test that starts with `if (condition) { return; }` reports a pass it never earned. In a Mocha `function` callback the `return` is `this.skip()`; in a Playwright test it is `test.skip(condition)`, which marks the test skipped. Bun's `test.skipIf` needs the condition moved out of the test, and is not suggested here.

**Why:** The report shows which tests ran.

```codelaw
match:
  kind: if_statement
  pattern: "if ($C) { return; }"
  nthChild: 1
  not: { has: { kind: else_clause } }
  any:
    - all:
        - inside:
            kind: statement_block
            inside:
              kind: function_expression
              inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^(it|test|specify)(\.only)?$' } } }
        - has: { stopBy: end, kind: return_statement, pattern: $R1 }
    - all:
        - inside:
            kind: statement_block
            inside:
              kind: arrow_function
              has: { field: parameters, has: { stopBy: end, kind: object_pattern } }
              inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^test(\.only)?$' } } }
        - pattern: $W2
message: "Mark the test skipped instead of returning early"
fix:
  - replace: { target: $R1, with: "this.skip();" }
  - replace: { target: $W2, with: "test.skip($C);" }
```

## Examples

### JavaScript

#### Don't

```js
it("reorders columns", function () {
  if (readOnly) {
    return;
  }
  db.reorder();
});
```

#### Do

```js
it("reorders columns", function () {
  if (readOnly) {
    this.skip();
  }
  db.reorder();
});
```


### JSX

#### Don't

```jsx
it("reorders columns", function () {
  if (readOnly) {
    return;
  }
  db.reorder();
});
```

#### Do

```jsx
it("reorders columns", function () {
  if (readOnly) {
    this.skip();
  }
  db.reorder();
});
```


### TypeScript

#### Don't

```ts
it("reorders columns", function () {
  if (readOnly) {
    return;
  }
  db.reorder();
});
```

#### Do

```ts
it("reorders columns", function () {
  if (readOnly) {
    this.skip();
  }
  db.reorder();
});
```


### TSX

#### Don't

```tsx
it("reorders columns", function () {
  if (readOnly) {
    return;
  }
  db.reorder();
});
```

#### Do

```tsx
it("reorders columns", function () {
  if (readOnly) {
    this.skip();
  }
  db.reorder();
});
```
