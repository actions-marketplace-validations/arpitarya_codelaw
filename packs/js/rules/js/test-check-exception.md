---
id: js/test-check-exception
title: "A test that expects a throw says what is thrown"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which error is expected is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/test-check-exception]
since: 0.5.0
---

`expect(fn).toThrow()`, `toThrowError()`, chai's `to.throw()` and `assert.throws(fn)` name the error class or message they expect.

**Why:** A bare check passes for any error, including the `TypeError` of a typo in the test itself.

```codelaw
match:
  any:
    - pattern: "expect($F).toThrow()"
    - pattern: "expect($F).toThrowError()"
    - pattern: "expect($F).to.throw()"
    - pattern: "assert.throws($F)"
message: "Say which error is expected"
```

## How to fix

1. Pass the error class or a message pattern: `toThrow(RangeError)`, `toThrow(/too large/)`.

## Examples

### JavaScript

#### Don't

```js
expect(() => parse("{")).toThrow();
```

#### Do

```js
expect(() => parse("{")).toThrow(SyntaxError);
```


### JSX

#### Don't

```jsx
expect(() => parse("{")).toThrow();
```

#### Do

```jsx
expect(() => parse("{")).toThrow(SyntaxError);
```


### TypeScript

#### Don't

```ts
expect(() => parse("{")).toThrow();
```

#### Do

```ts
expect(() => parse("{")).toThrow(SyntaxError);
```


### TSX

#### Don't

```tsx
expect(() => parse("{")).toThrow();
```

#### Do

```tsx
expect(() => parse("{")).toThrow(SyntaxError);
```
