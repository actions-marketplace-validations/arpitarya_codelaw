---
id: js/no-lonely-if
title: "else { if } is else if"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/no-lonely-if, eslint-plugin-unicorn/no-lonely-if]
since: 0.5.0
---

An `else` block whose only statement is an `if` is written as `else if`.

**Why:** The extra block and indentation say nothing the `else if` form doesn't, and hide that the branches are one chain.

```codelaw

match:
  kind: else_clause
  has: { kind: statement_block, pattern: "{ if ($C) $$$BODY }" }
message: "An `else` holding only an `if` reads better as `else if`"
fix: "else if ($C) $$$BODY"
```

## Examples

### JavaScript

#### Don't

```js
if (a) {
  one();
} else {
  if (b) {
    two();
  }
}
```

#### Do

```js
if (a) {
  one();
} else if (b) {
  two();
}
```

#### Fixed

```js
if (a) {
  one();
} else if (b) {
    two();
  }
```

### JSX

#### Don't

```jsx
if (a) {
  one();
} else {
  if (b) {
    two();
  }
}
```

#### Do

```jsx
if (a) {
  one();
} else if (b) {
  two();
}
```

#### Fixed

```jsx
if (a) {
  one();
} else if (b) {
    two();
  }
```

### TypeScript

#### Don't

```ts
if (a) {
  one();
} else {
  if (b) {
    two();
  }
}
```

#### Do

```ts
if (a) {
  one();
} else if (b) {
  two();
}
```

#### Fixed

```ts
if (a) {
  one();
} else if (b) {
    two();
  }
```

### TSX

#### Don't

```tsx
if (a) {
  one();
} else {
  if (b) {
    two();
  }
}
```

#### Do

```tsx
if (a) {
  one();
} else if (b) {
  two();
}
```

#### Fixed

```tsx
if (a) {
  one();
} else if (b) {
    two();
  }
```
