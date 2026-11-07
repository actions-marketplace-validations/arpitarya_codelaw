---
id: js/prefer-flat-math-min-max
title: "One Math.min or Math.max call rather than nested ones"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-flat-math-min-max]
since: 0.5.0
---

`Math.max` and `Math.min` take any number of arguments, so one call is not nested in another of the same: `Math.max(Math.max(a, b), c)` is `Math.max(a, b, c)`.

**Why:** The nesting adds a call and parentheses and computes the same value.

```codelaw
match:
  any:
    - pattern: "Math.$F(Math.$F($$$A), $B)"
      all: [{ pattern: $LEFT }]
    - pattern: "Math.$F($B, Math.$F($$$A))"
      all: [{ pattern: $RIGHT }]
constraints:
  F: { regex: '^(min|max)$' }
message: "Flatten the nested `Math.$F`"
fix:
  - replace: { target: $LEFT, with: "Math.$F($$$A, $B)" }
  - replace: { target: $RIGHT, with: "Math.$F($B, $$$A)" }
```

## Examples

### JavaScript

#### Don't

```js
const top = Math.max(Math.max(a, b), c);
```

#### Do

```js
const top = Math.max(a, b, c);
```


### JSX

#### Don't

```jsx
const top = Math.max(Math.max(a, b), c);
```

#### Do

```jsx
const top = Math.max(a, b, c);
```


### TypeScript

#### Don't

```ts
const top = Math.max(Math.max(a, b), c);
```

#### Do

```ts
const top = Math.max(a, b, c);
```


### TSX

#### Don't

```tsx
const top = Math.max(Math.max(a, b), c);
```

#### Do

```tsx
const top = Math.max(a, b, c);
```
