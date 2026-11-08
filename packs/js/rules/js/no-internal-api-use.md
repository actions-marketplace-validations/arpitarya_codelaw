---
id: js/no-internal-api-use
title: "No import from inside `node_modules`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the package's public entry that exports the same thing has to be found"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-internal-api-use]
since: 0.5.0
---

An `import` or `require()` whose path contains `node_modules` reaches into a dependency's own files, and is reported at the path.

**Why:** Those files are the dependency's internals: they move between versions and break the import without warning.

```codelaw
native: js/no-internal-api-use
```

## How to fix

1. Import the package by its name, from its public entry.

## Examples

### JavaScript

#### Don't

```js
import helper from "../node_modules/pkg/lib/helper";
```

#### Do

```js
import helper from "pkg/lib/helper";
```


### JSX

#### Don't

```jsx
import helper from "../node_modules/pkg/lib/helper";
```

#### Do

```jsx
import helper from "pkg/lib/helper";
```


### TypeScript

#### Don't

```ts
import helper from "../node_modules/pkg/lib/helper";
```

#### Do

```ts
import helper from "pkg/lib/helper";
```


### TSX

#### Don't

```tsx
import helper from "../node_modules/pkg/lib/helper";
```

#### Do

```tsx
import helper from "pkg/lib/helper";
```
