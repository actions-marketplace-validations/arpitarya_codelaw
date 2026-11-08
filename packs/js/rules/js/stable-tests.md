---
id: js/stable-tests
title: "No retrying flaky tests"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the fix is to find what makes the test flaky, which no edit does"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/stable-tests]
since: 0.5.0
---

`this.retries(n)` in Mocha and `jest.retryTimes(n)` re-run a failing test until it passes, which hides the instability instead of fixing it. A call with a count above zero is reported.

**Why:** A retried test passes on luck, and the race it hides ships.

```codelaw
match:
  kind: call_expression
  any:
    - pattern: "this.retries($N)"
    - pattern: "jest.retryTimes($N)"
constraints:
  N: { not: { kind: number, regex: '^0$' } }
message: "Make the test stable instead of retrying it"
```

## How to fix

1. Find the source of the flakiness (timing, shared state, network) and remove it, then drop the retry.

## Examples

### JavaScript

#### Don't

```js
describe("checkout", function () {
  this.retries(3);
});
```

#### Do

```js
describe("checkout", function () {
  beforeEach(resetDatabase);
});
```


### JSX

#### Don't

```jsx
describe("checkout", function () {
  this.retries(3);
});
```

#### Do

```jsx
describe("checkout", function () {
  beforeEach(resetDatabase);
});
```


### TypeScript

#### Don't

```ts
describe("checkout", function () {
  this.retries(3);
});
```

#### Do

```ts
describe("checkout", function () {
  beforeEach(resetDatabase);
});
```


### TSX

#### Don't

```tsx
describe("checkout", function () {
  this.retries(3);
});
```

#### Do

```tsx
describe("checkout", function () {
  beforeEach(resetDatabase);
});
```
