---
id: js/import-no-empty-named-blocks
title: "No empty import braces"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "if no side effect was meant the whole import should go instead"
stability: preview
ported_from: [eslint-plugin-import/no-empty-named-blocks]
since: 0.5.0
---

An `import` does not list an empty set of names (`import {} from "x"`).

**Why:** Empty braces read like a half-finished import; a bare `import "x"` says the module is loaded for its side effects.

```codelaw
match:
  kind: import_statement
  has:
    field: source
    pattern: $S
  all:
  - has:
      kind: import_clause
      regex: ^\{\s*\}$
message: The braces import nothing; write `import $S` if the side effect is meant
fix: import $S;
```

## Examples

### JavaScript

#### Don't

```js
import {} from "./polyfills";
```

#### Do

```js
import "./polyfills";
```

### JSX

#### Don't

```jsx
import {} from "./polyfills";
```

#### Do

```jsx
import "./polyfills";
```

### TypeScript

#### Don't

```ts
import {} from "./polyfills";
```

#### Do

```ts
import "./polyfills";
```

### TSX

#### Don't

```tsx
import {} from "./polyfills";
```

#### Do

```tsx
import "./polyfills";
```
