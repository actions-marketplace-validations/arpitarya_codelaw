---
id: js/no-useless-increment
title: "No x = x++"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the increment or the assignment was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-useless-increment]
since: 0.5.0
---

`x = x++` and `x = x--` are not written.

**Why:** The postfix operator returns the old value, which is assigned back: the variable never changes.

```codelaw

match:
  any: [{ pattern: "$X = $X++" }, { pattern: "$X = $X--" }]
message: "`$X = $X++` leaves `$X` unchanged; write `$X++`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
count = count++;
```

#### Do

```js
count++;
```

### JSX

#### Don't

```jsx
count = count++;
```

#### Do

```jsx
count++;
```

### TypeScript

#### Don't

```ts
count = count++;
```

#### Do

```ts
count++;
```

### TSX

#### Don't

```tsx
count = count++;
```

#### Do

```tsx
count++;
```
