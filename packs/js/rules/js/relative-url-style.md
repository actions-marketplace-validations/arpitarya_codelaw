---
id: js/relative-url-style
title: "No ./ in a relative URL"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "stripping the prefix is a computed edit"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/relative-url-style]
since: 0.5.0
---

The first argument of `new URL(path, base)` does not start with `./` (the upstream default, `"never"`).

**Why:** `./x` and `x` resolve to the same URL against a base.

```codelaw
match:
  kind: new_expression
  has:
    field: constructor
    regex: ^URL$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: string
        regex: ^.\./
      all:
      - has:
          nthChild: 2
message: A relative URL needs no leading `./`
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const u = new URL("./api/users", base);
```

#### Do

```js
const u = new URL("api/users", base);
```

### JSX

#### Don't

```jsx
const u = new URL("./api/users", base);
```

#### Do

```jsx
const u = new URL("api/users", base);
```

### TypeScript

#### Don't

```ts
const u = new URL("./api/users", base);
```

#### Do

```ts
const u = new URL("api/users", base);
```

### TSX

#### Don't

```tsx
const u = new URL("./api/users", base);
```

#### Do

```tsx
const u = new URL("api/users", base);
```
