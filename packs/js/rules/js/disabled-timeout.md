---
id: js/disabled-timeout
title: "No timeout(0) in a test"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the right bound is the author's choice"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/disabled-timeout]
since: 0.5.0
---

A Mocha test does not turn its timeout off with `this.timeout(0)`.

**Why:** A hanging test then blocks CI until the job itself is killed.

```codelaw

match:
  pattern: this.timeout(0)
message: "A test without a timeout can hang the suite forever; set a bound"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
it("syncs", function () {
  this.timeout(0);
});
```

#### Do

```js
it("syncs", function () {
  this.timeout(10000);
});
```

### JSX

#### Don't

```jsx
it("syncs", function () {
  this.timeout(0);
});
```

#### Do

```jsx
it("syncs", function () {
  this.timeout(10000);
});
```

### TypeScript

#### Don't

```ts
it("syncs", function () {
  this.timeout(0);
});
```

#### Do

```ts
it("syncs", function () {
  this.timeout(10000);
});
```

### TSX

#### Don't

```tsx
it("syncs", function () {
  this.timeout(0);
});
```

#### Do

```tsx
it("syncs", function () {
  this.timeout(10000);
});
```
