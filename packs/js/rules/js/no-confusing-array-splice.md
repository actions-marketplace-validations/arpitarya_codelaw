---
id: js/no-confusing-array-splice
title: "Index assignment or with() rather than splice() replacing one element"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`splice` and `toSpliced` count a negative index from the end and clamp one past the end; assignment and `with()` do not, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-confusing-array-splice]
since: 0.5.0
---

`array.splice(index, 1, element)` used as a statement replaces one element in place: it is `array[index] = element`. `array.toSpliced(index, 1, element)` makes a copy with one element replaced: it is `array.with(index, element)`.

**Why:** The splice form makes a reader decode three arguments to see a one-element replacement.

```codelaw
match:
  any:
    - all:
        - kind: expression_statement
        - pattern: "$A.splice($I, 1, $E);"
        - pattern: $W1
    - all:
        - kind: call_expression
        - pattern: "$A.toSpliced($I, 1, $E)"
        - pattern: $W2
constraints:
  I: { not: { any: [{ kind: unary_expression, regex: '^-' }, { kind: spread_element }] } }
  E: { not: { kind: spread_element } }
message: "Replace the element directly"
fix:
  - replace: { target: $W1, with: "$A[$I] = $E;" }
  - replace: { target: $W2, with: "$A.with($I, $E)" }
```

## Examples

### JavaScript

#### Don't

```js
items.splice(index, 1, item);
const next = items.toSpliced(index, 1, item);
```

#### Do

```js
items[index] = item;
const next = items.with(index, item);
```


### JSX

#### Don't

```jsx
items.splice(index, 1, item);
const next = items.toSpliced(index, 1, item);
```

#### Do

```jsx
items[index] = item;
const next = items.with(index, item);
```


### TypeScript

#### Don't

```ts
items.splice(index, 1, item);
const next = items.toSpliced(index, 1, item);
```

#### Do

```ts
items[index] = item;
const next = items.with(index, item);
```


### TSX

#### Don't

```tsx
items.splice(index, 1, item);
const next = items.toSpliced(index, 1, item);
```

#### Do

```tsx
items[index] = item;
const next = items.with(index, item);
```
