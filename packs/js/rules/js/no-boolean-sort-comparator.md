---
id: js/no-boolean-sort-comparator
title: "A sort comparator returns a number"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a numeric difference or a string comparison suits different values; the author picks by what is sorted"
stability: preview
ported_from: [eslint-plugin-unicorn/no-boolean-sort-comparator]
since: 0.5.0
---

The comparator given to `sort` or `toSorted` does not return the result of a comparison operator.

**Why:** A boolean becomes 1 or 0, never negative, so the sort order is wrong in engines that rely on it.

```codelaw
match:
  kind: arrow_function
  all:
    - has: { field: body, kind: binary_expression, has: { field: operator, regex: '^(<|>|<=|>=|===|!==)$' } }
    - inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^(sort|toSorted)$' } } } }
    - any:
        - all: [{ pattern: $ASC }, { any: [{ pattern: "($A, $B) => $A > $B" }, { pattern: "($A, $B) => $A >= $B" }] }]
        - all: [{ pattern: $DESC }, { any: [{ pattern: "($A, $B) => $A < $B" }, { pattern: "($A, $B) => $A <= $B" }] }]
        - { kind: arrow_function }
message: "A comparator must return a negative, zero or positive number, not a boolean"
fix:
  choices:
    - label: numeric difference
      default: true
      fix:
        - replace: { target: $ASC, with: "($A, $B) => $A - $B" }
        - replace: { target: $DESC, with: "($A, $B) => $B - $A" }
    - label: string comparison
      fix:
        - replace: { target: $ASC, with: "($A, $B) => $A.localeCompare($B)" }
        - replace: { target: $DESC, with: "($A, $B) => $B.localeCompare($A)" }
```

## How to fix

1. Return `a - b` for numbers, `a.localeCompare(b)` for strings.

## Examples

### JavaScript

#### Don't

```js
items.sort((a, b) => a > b);
```

#### Fixed (numeric difference)

```js
items.sort((a, b) => a - b);
```

#### Fixed (string comparison)

```js
items.sort((a, b) => a.localeCompare(b));
```

#### Do

```js
items.sort((a, b) => a - b);
```

### JSX

#### Don't

```jsx
items.sort((a, b) => a > b);
```

#### Fixed (numeric difference)

```jsx
items.sort((a, b) => a - b);
```

#### Fixed (string comparison)

```jsx
items.sort((a, b) => a.localeCompare(b));
```

#### Do

```jsx
items.sort((a, b) => a - b);
```

### TypeScript

#### Don't

```ts
items.sort((a, b) => a > b);
```

#### Fixed (numeric difference)

```ts
items.sort((a, b) => a - b);
```

#### Fixed (string comparison)

```ts
items.sort((a, b) => a.localeCompare(b));
```

#### Do

```ts
items.sort((a, b) => a - b);
```

### TSX

#### Don't

```tsx
items.sort((a, b) => a > b);
```

#### Fixed (numeric difference)

```tsx
items.sort((a, b) => a - b);
```

#### Fixed (string comparison)

```tsx
items.sort((a, b) => a.localeCompare(b));
```

#### Do

```tsx
items.sort((a, b) => a - b);
```
