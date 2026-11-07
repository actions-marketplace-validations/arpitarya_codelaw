---
id: js/prefer-switch
title: "A switch rather than an if-else chain on one value"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the switch needs a break or return per case, which the author writes"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-switch]
since: 0.5.0
---

An `if`-`else if` chain that compares the same value with `===` three or more times is a `switch` (unicorn's default `minimumCases: 3`).

**Why:** The switch names the value once and lines up the cases.

```codelaw
match:
  kind: if_statement
  pattern: "if ($X === $A) { $$$S1 } else if ($X === $B) { $$$S2 } else if ($X === $C) { $$$S3 }"
  not: { inside: { kind: else_clause } }
message: "Write this chain as a `switch ($X)`"
```

## How to fix

1. Write `switch (value)` with one `case` per compared value, each ending in `break` or `return`, and the final `else` as `default`.

## Examples

### JavaScript

#### Don't

```js
if (key === "a") {
  left();
} else if (key === "d") {
  right();
} else if (key === "w") {
  up();
}
```

#### Do

```js
switch (key) {
  case "a":
    left();
    break;
  case "d":
    right();
    break;
  case "w":
    up();
    break;
}
```


### JSX

#### Don't

```jsx
if (key === "a") {
  left();
} else if (key === "d") {
  right();
} else if (key === "w") {
  up();
}
```

#### Do

```jsx
switch (key) {
  case "a":
    left();
    break;
  case "d":
    right();
    break;
  case "w":
    up();
    break;
}
```


### TypeScript

#### Don't

```ts
if (key === "a") {
  left();
} else if (key === "d") {
  right();
} else if (key === "w") {
  up();
}
```

#### Do

```ts
switch (key) {
  case "a":
    left();
    break;
  case "d":
    right();
    break;
  case "w":
    up();
    break;
}
```


### TSX

#### Don't

```tsx
if (key === "a") {
  left();
} else if (key === "d") {
  right();
} else if (key === "w") {
  up();
}
```

#### Do

```tsx
switch (key) {
  case "a":
    left();
    break;
  case "d":
    right();
    break;
  case "w":
    up();
    break;
}
```
