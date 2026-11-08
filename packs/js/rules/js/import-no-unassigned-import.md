---
id: js/import-no-unassigned-import
title: "No import only for side effects"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the import is for, and what it should provide, is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-import/no-unassigned-import]
since: 0.5.0
---

A module is not imported or required only for its side effects (`import "./setup"`, `require("./setup")` as a statement).

**Why:** An import that binds nothing hides what it changes; a reader can't see what depends on it.

```codelaw
match:
  any:
    - kind: import_statement
      not: { has: { kind: import_clause } }
      has: { field: source, kind: string }
    - kind: expression_statement
      pattern: "require($S);"
message: "This import binds nothing; import what it provides"
```

## How to fix

1. Export a function from the module and call it here, or allow this module in the project's configuration if a side-effect import is meant.

## Examples

### JavaScript

#### Don't

```js
import "./setup-globals";
```

#### Do

```js
import { setupGlobals } from "./setup-globals";
setupGlobals();
```


### JSX

#### Don't

```jsx
import "./setup-globals";
```

#### Do

```jsx
import { setupGlobals } from "./setup-globals";
setupGlobals();
```


### TypeScript

#### Don't

```ts
import "./setup-globals";
```

#### Do

```ts
import { setupGlobals } from "./setup-globals";
setupGlobals();
```


### TSX

#### Don't

```tsx
import "./setup-globals";
```

#### Do

```tsx
import { setupGlobals } from "./setup-globals";
setupGlobals();
```
