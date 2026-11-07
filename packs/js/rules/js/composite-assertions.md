---
id: js/composite-assertions
title: "One condition per assertion, not a && b"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "two assertions stop at the first failure and report each condition apart; that is the intent, but the messages change"
stability: preview
ported_from: [eslint-plugin-sonarjs/composite-assertions]
since: 0.5.0
---

`assert(a && b)`, `assert.ok(a && b)` and `expect(a && b).toBeTruthy()` fail without saying which condition was false. As statements they are two assertions, one per condition. Upstream also splits negated `||` groups; those are not reported here.

**Why:** The failure message tells you which condition broke.

```codelaw
match:
  kind: expression_statement
  any:
    - all:
        - any:
            - pattern: "assert($A && $B);"
            - pattern: "assert.ok($A && $B);"
        - has: { kind: call_expression, has: { field: function, pattern: $FN } }
        - pattern: $W1
    - all:
        - pattern: "expect($C && $D).toBeTruthy();"
        - pattern: $W2
message: "Assert each condition on its own"
fix:
  - replace: { target: $W1, with: "$FN($A);\n$FN($B);" }
  - replace: { target: $W2, with: "expect($C).toBeTruthy();\nexpect($D).toBeTruthy();" }
```

## Examples

### JavaScript

#### Don't

```js
assert.ok(user && user.active);
```

#### Do

```js
assert.ok(user);
assert.ok(user.active);
```


### JSX

#### Don't

```jsx
assert.ok(user && user.active);
```

#### Do

```jsx
assert.ok(user);
assert.ok(user.active);
```


### TypeScript

#### Don't

```ts
assert.ok(user && user.active);
```

#### Do

```ts
assert.ok(user);
assert.ok(user.active);
```


### TSX

#### Don't

```tsx
assert.ok(user && user.active);
```

#### Do

```tsx
assert.ok(user);
assert.ok(user.active);
```
