---
id: js/no-array-reverse
title: "toReversed() rather than reverse()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`toReversed()` leaves the original untouched, which code later reading it may rely on either way"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-reverse]
since: 0.5.0
---

The result of `.reverse()` is not used as a value; `.toReversed()` returns a reversed copy. A `.reverse()` call on its own line, used for its effect, is left alone.

**Why:** `.reverse()` reverses the array in place and returns it, so code that uses the result also changes the original.

```codelaw

match:
  pattern: $A.reverse()
  not: { inside: { kind: expression_statement } }
message: "`.reverse()` mutates `$A`; use `.toReversed()`"
fix: "$A.toReversed()"
```

## Examples

### JavaScript

#### Don't

```js
const latest = items.reverse()[0];
```

#### Do

```js
const latest = items.toReversed()[0];
```

### JSX

#### Don't

```jsx
const latest = items.reverse()[0];
```

#### Do

```jsx
const latest = items.toReversed()[0];
```

### TypeScript

#### Don't

```ts
const latest = items.reverse()[0];
```

#### Do

```ts
const latest = items.toReversed()[0];
```

### TSX

#### Don't

```tsx
const latest = items.reverse()[0];
```

#### Do

```tsx
const latest = items.toReversed()[0];
```
