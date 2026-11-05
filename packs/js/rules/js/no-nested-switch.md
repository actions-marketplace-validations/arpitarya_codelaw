---
id: js/no-nested-switch
title: "No switch inside a switch"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the extracted function needs a name"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-nested-switch]
since: 0.5.0
---

A `switch` is not nested in another `switch`.

**Why:** Nested `switch` statements are hard to read and their `break`s are easy to confuse.

```codelaw

match:
  kind: switch_statement
  inside: { stopBy: end, kind: switch_statement }
message: "Move the inner `switch` into a function"
```

## How to fix

1. Move the inner `switch` into a named function and call it.

## Examples

### JavaScript

#### Don't

```js
switch (a) {
  case 1:
    switch (b) {
      case 2:
        run();
    }
}
```

#### Do

```js
switch (a) {
  case 1:
    handleB(b);
}
```

### JSX

#### Don't

```jsx
switch (a) {
  case 1:
    switch (b) {
      case 2:
        run();
    }
}
```

#### Do

```jsx
switch (a) {
  case 1:
    handleB(b);
}
```

### TypeScript

#### Don't

```ts
switch (a) {
  case 1:
    switch (b) {
      case 2:
        run();
    }
}
```

#### Do

```ts
switch (a) {
  case 1:
    handleB(b);
}
```

### TSX

#### Don't

```tsx
switch (a) {
  case 1:
    switch (b) {
      case 2:
        run();
    }
}
```

#### Do

```tsx
switch (a) {
  case 1:
    handleB(b);
}
```
