---
id: js/prefer-set-methods
title: "Set methods rather than filtering a spread"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the methods exist only on a `Set`; whether the left-hand value is one is not known here, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-set-methods]
since: 0.5.0
---

The intersection, difference or union of two sets is computed with `a.intersection(b)`, `a.difference(b)` or `a.union(b)`, not by spreading into an array and filtering or concatenating.

**Why:** The methods name the operation and skip the intermediate array.

```codelaw
match:
  any:
    - pattern: "new Set([...$A].filter(($X) => $B.has($X)))"
      all: [{ pattern: $INT }]
    - pattern: "new Set([...$A].filter(($X) => !$B.has($X)))"
      all: [{ pattern: $DIF }]
    - pattern: "new Set([...$A, ...$B])"
      all: [{ pattern: $UNI }]
message: "Use the Set method"
fix:
  - replace: { target: $INT, with: "$A.intersection($B)" }
  - replace: { target: $DIF, with: "$A.difference($B)" }
  - replace: { target: $UNI, with: "$A.union($B)" }
```

## Examples

### JavaScript

#### Don't

```js
const both = new Set([...mine].filter((x) => theirs.has(x)));
```

#### Do

```js
const both = mine.intersection(theirs);
```


### JSX

#### Don't

```jsx
const both = new Set([...mine].filter((x) => theirs.has(x)));
```

#### Do

```jsx
const both = mine.intersection(theirs);
```


### TypeScript

#### Don't

```ts
const both = new Set([...mine].filter((x) => theirs.has(x)));
```

#### Do

```ts
const both = mine.intersection(theirs);
```


### TSX

#### Don't

```tsx
const both = new Set([...mine].filter((x) => theirs.has(x)));
```

#### Do

```tsx
const both = mine.intersection(theirs);
```
