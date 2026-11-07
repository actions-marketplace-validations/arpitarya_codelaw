---
id: js/no-trivial-assertions
title: "No assertion that compares two literals"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the test should check is the author's to write"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/no-trivial-assertions]
since: 0.5.0
---

`expect(true).toBeTruthy()`, `expect(1).not.toBe(2)` and `assert.notStrictEqual(1, 2)` check a constant against a constant: they pass whatever the code does. An assertion whose actual value is a literal (and, for a comparison, whose expected value is one too) is reported.

**Why:** A test that can't fail gives a false sense of coverage.

```codelaw
utils:
  lit:
    any:
      - kind: "true"
      - kind: "false"
      - kind: number
      - kind: string
      - kind: "null"
match:
  kind: call_expression
  any:
    - all:
        - has: { field: function, kind: member_expression, regex: '^expect\(' }
        - has: { stopBy: end, kind: call_expression, regex: '^expect\(', has: { field: arguments, has: { nthChild: 1, matches: lit } } }
        - any:
            - has: { field: function, has: { field: property, regex: '^(toBeTruthy|toBeFalsy|toBeDefined|toBeUndefined|toBeNull)$' } }
            - has: { field: arguments, has: { nthChild: 1, matches: lit } }
    - all:
        - has: { field: function, regex: '^assert(\.(ok|equal|notEqual|strictEqual|notStrictEqual|deepEqual|deepStrictEqual))?$' }
        - has: { field: arguments, has: { nthChild: 1, matches: lit } }
        - any:
            - has: { field: function, regex: '^assert(\.ok)?$' }
            - has: { field: arguments, has: { nthChild: 2, matches: lit } }
message: "This assertion compares constants and can't fail"
```

## How to fix

1. Assert on a value the code under test produced: `expect(result).toBe(2)`.

## Examples

### JavaScript

#### Don't

```js
expect(true).toBeTruthy();
```

#### Do

```js
expect(isReady()).toBeTruthy();
```


### JSX

#### Don't

```jsx
expect(true).toBeTruthy();
```

#### Do

```jsx
expect(isReady()).toBeTruthy();
```


### TypeScript

#### Don't

```ts
expect(true).toBeTruthy();
```

#### Do

```ts
expect(isReady()).toBeTruthy();
```


### TSX

#### Don't

```tsx
expect(true).toBeTruthy();
```

#### Do

```tsx
expect(isReady()).toBeTruthy();
```
