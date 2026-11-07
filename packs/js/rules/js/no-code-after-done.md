---
id: js/no-code-after-done
title: "No code after done() in a callback test"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the code belongs before `done()` or nowhere is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-code-after-done]
since: 0.5.0
---

In a test that takes a `done` callback, no statement follows the `done()` call in the same block.

**Why:** After `done()` the test is over; what runs next is outside it, and its failures land on another test.

```codelaw
match:
  kind: expression_statement
  pattern: "done();"
  precedes: { not: { kind: comment } }
  inside:
    stopBy: end
    any: [{ kind: arrow_function }, { kind: function_expression }]
    has: { field: parameters, has: { kind: identifier, regex: '^done$' } }
message: "Code after `done()` runs after the test has ended"
```

## How to fix

1. Move the statements before `done()`, or delete them.

## Examples

### JavaScript

#### Don't

```js
it("loads", (done) => {
  load(() => {
    done();
    cleanup();
  });
});
```

#### Do

```js
it("loads", (done) => {
  load(() => {
    cleanup();
    done();
  });
});
```


### JSX

#### Don't

```jsx
it("loads", (done) => {
  load(() => {
    done();
    cleanup();
  });
});
```

#### Do

```jsx
it("loads", (done) => {
  load(() => {
    cleanup();
    done();
  });
});
```


### TypeScript

#### Don't

```ts
it("loads", (done) => {
  load(() => {
    done();
    cleanup();
  });
});
```

#### Do

```ts
it("loads", (done) => {
  load(() => {
    cleanup();
    done();
  });
});
```


### TSX

#### Don't

```tsx
it("loads", (done) => {
  load(() => {
    done();
    cleanup();
  });
});
```

#### Do

```tsx
it("loads", (done) => {
  load(() => {
    cleanup();
    done();
  });
});
```
