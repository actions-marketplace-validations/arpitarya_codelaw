---
id: js/prefer-array-slice
title: "slice() rather than splice() when only the removed part is read"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`splice` also removes the elements from the array; dropping that is the point, but code that relied on it changes, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-slice]
since: 0.5.0
---

`array.splice(start)[i]` and `array.splice(start).at(i)` read one element of what `splice` cut off, and throw the rest away along with the array's tail. `array.slice(start)` reads the same element without changing the array. A `splice` with a delete count is left, since it may mean to remove a few.

**Why:** The mutation reads as an accident next to a read; `slice` says the code only reads.

```codelaw
match:
  any:
    - pattern: "$A.splice($S)[$I]"
    - pattern: "$A.splice($S).at($I)"
  has: { stopBy: end, kind: property_identifier, regex: '^splice$', pattern: $P }
  not:
    any:
      - inside: { kind: assignment_expression, field: left }
      - inside: { kind: augmented_assignment_expression, field: left }
constraints:
  S: { not: { kind: spread_element } }
message: "Read with `slice`, which leaves the array alone"
fix:
  - replace: { target: $P, with: slice }
```

## Examples

### JavaScript

#### Don't

```js
const first = process.argv.splice(2)[0];
```

#### Do

```js
const first = process.argv.slice(2)[0];
```


### JSX

#### Don't

```jsx
const first = process.argv.splice(2)[0];
```

#### Do

```jsx
const first = process.argv.slice(2)[0];
```


### TypeScript

#### Don't

```ts
const first = process.argv.splice(2)[0];
```

#### Do

```ts
const first = process.argv.slice(2)[0];
```


### TSX

#### Don't

```tsx
const first = process.argv.splice(2)[0];
```

#### Do

```tsx
const first = process.argv.slice(2)[0];
```
