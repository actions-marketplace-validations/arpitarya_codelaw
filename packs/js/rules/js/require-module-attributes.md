---
id: js/require-module-attributes
title: "JSON imports say with { type: \"json\" }"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a bundler that does not support import attributes may then reject the file"
stability: preview
ported_from: [eslint-plugin-unicorn/require-module-attributes]
since: 0.5.0
---

An `import` of a `.json` file carries the import attribute `with { type: "json" }`.

**Why:** Browsers and Node require the attribute to load JSON as a module.

```codelaw

match:
  kind: import_statement
  has: { field: source, regex: '\.json.$', pattern: $S }
  not: { regex: '\b(with|assert)\s*\{' }
message: "Import JSON with `with { type: \"json\" }`"
fix:
  - insert-after: { target: $S, text: ' with { type: "json" }' }
```

## Examples

### JavaScript

#### Don't

```js
import data from "./data.json";
```

#### Do

```js
import data from "./data.json" with { type: "json" };
```

### JSX

#### Don't

```jsx
import data from "./data.json";
```

#### Do

```jsx
import data from "./data.json" with { type: "json" };
```

### TypeScript

#### Don't

```ts
import data from "./data.json";
```

#### Do

```ts
import data from "./data.json" with { type: "json" };
```

### TSX

#### Don't

```tsx
import data from "./data.json";
```

#### Do

```tsx
import data from "./data.json" with { type: "json" };
```
