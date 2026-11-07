---
id: js/prefer-aggregate-error
title: "Throw an AggregateError of the collected errors"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the thrown type changes from `Error` to `AggregateError`; handlers that check the type need review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-aggregate-error]
since: 0.5.0
---

A guard like `if (errors.length > 0) { throw new Error(message); }` throws away the errors it just counted. `throw new AggregateError(errors, message)` keeps them on the thrown error. The collection's name ends in `errors` (`errors`, `validationErrors`); upstream also reads TypeScript types, which this rule does not.

**Why:** Whoever catches the error gets the details instead of a summary.

```codelaw
match:
  kind: throw_statement
  pattern: "throw new Error($M);"
  inside:
    kind: statement_block
    inside:
      kind: if_statement
      any:
        - pattern: "if ($E.length > 0) $B"
        - pattern: "if ($E.length !== 0) $B"
        - pattern: "if ($E.length) $B"
  has: { kind: new_expression, pattern: $NEW }
constraints:
  E: { kind: identifier, regex: '(^e|E)rrors$' }
  M: { not: { kind: spread_element } }
message: "Throw `AggregateError` with `$E`"
fix:
  - replace: { target: $NEW, with: "new AggregateError($E, $M)" }
```

## Examples

### JavaScript

#### Don't

```js
if (errors.length > 0) {
  throw new Error("Validation failed");
}
```

#### Do

```js
if (errors.length > 0) {
  throw new AggregateError(errors, "Validation failed");
}
```


### JSX

#### Don't

```jsx
if (errors.length > 0) {
  throw new Error("Validation failed");
}
```

#### Do

```jsx
if (errors.length > 0) {
  throw new AggregateError(errors, "Validation failed");
}
```


### TypeScript

#### Don't

```ts
if (errors.length > 0) {
  throw new Error("Validation failed");
}
```

#### Do

```ts
if (errors.length > 0) {
  throw new AggregateError(errors, "Validation failed");
}
```


### TSX

#### Don't

```tsx
if (errors.length > 0) {
  throw new Error("Validation failed");
}
```

#### Do

```tsx
if (errors.length > 0) {
  throw new AggregateError(errors, "Validation failed");
}
```
