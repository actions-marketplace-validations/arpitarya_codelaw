---
id: js/require-array-sort-compare
title: "A compare function for sort()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the order wanted — numeric, by locale, or the default code-unit order — is the author's; the two common compare functions are offered"
stability: preview
ported_from: [eslint-plugin-unicorn/require-array-sort-compare]
since: 0.5.0
---

`sort()` and `toSorted()` are called with a compare function. Without one they compare the elements as strings, so `[10, 9, 1].sort()` is `[1, 10, 9]`.

**Why:** The default order is right only for strings of code units, and is a common source of wrong numeric sorts.

```codelaw
match:
  pattern: "$A.$M()"
constraints:
  M: { regex: '^(sort|toSorted)$' }
message: "Pass a compare function to `$M`"
fix:
  choices:
    - label: numbers
      default: true
      fix: "$A.$M((a, b) => a - b)"
    - label: text
      fix: "$A.$M((a, b) => a.localeCompare(b))"
```

## Examples

### JavaScript

#### Don't

```js
scores.sort();
```

#### Do

```js
scores.sort((a, b) => a - b);
```

#### Fixed (numbers)

```js
scores.sort((a, b) => a - b);
```

#### Fixed (text)

```js
scores.sort((a, b) => a.localeCompare(b));
```


### JSX

#### Don't

```jsx
scores.sort();
```

#### Do

```jsx
scores.sort((a, b) => a - b);
```

#### Fixed (numbers)

```jsx
scores.sort((a, b) => a - b);
```

#### Fixed (text)

```jsx
scores.sort((a, b) => a.localeCompare(b));
```


### TypeScript

#### Don't

```ts
scores.sort();
```

#### Do

```ts
scores.sort((a, b) => a - b);
```

#### Fixed (numbers)

```ts
scores.sort((a, b) => a - b);
```

#### Fixed (text)

```ts
scores.sort((a, b) => a.localeCompare(b));
```


### TSX

#### Don't

```tsx
scores.sort();
```

#### Do

```tsx
scores.sort((a, b) => a - b);
```

#### Fixed (numbers)

```tsx
scores.sort((a, b) => a - b);
```

#### Fixed (text)

```tsx
scores.sort((a, b) => a.localeCompare(b));
```
