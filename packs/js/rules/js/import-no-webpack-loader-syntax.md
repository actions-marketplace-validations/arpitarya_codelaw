---
id: js/import-no-webpack-loader-syntax
title: "No webpack loaders in import paths"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the loaders move to the bundler config, which is another file"
stability: preview
ported_from: [eslint-plugin-import/no-webpack-loader-syntax]
since: 0.5.0
---

A module path names only the module (`./styles.css`), never a chain of webpack loaders (`style-loader!css-loader!./styles.css`).

**Why:** Inline loaders tie the code to webpack: another bundler, a test runner or a type checker cannot resolve the path, and the loaders are configured in two places.

```codelaw
utils:
  source:
    kind: string
    any:
      - inside: { kind: import_statement, field: source }
      - inside: { kind: export_statement, field: source }
      - inside:
          kind: arguments
          inside:
            kind: call_expression
            has: { field: function, regex: ^(require|import)$ }
match:
  matches: source
  has: { kind: string_fragment, regex: '!', pattern: $P }
message: "`$P` carries webpack loader syntax; configure the loaders in the bundler config and import the plain path"
```

## How to fix

1. Add a rule for the file type to the bundler config (`module.rules` in `webpack.config.js`) with the loaders named in the path.
2. Import the plain path: drop everything up to and including the last `!`.

## Examples

### JavaScript

#### Don't

```js
import styles from "style-loader!css-loader!./styles.css";
use(styles);
```

#### Do

```js
import styles from "./styles.css";
use(styles);
```

### JSX

#### Don't

```jsx
import styles from "style-loader!css-loader!./styles.css";
use(styles);
```

#### Do

```jsx
import styles from "./styles.css";
use(styles);
```

### TypeScript

#### Don't

```ts
import styles from "style-loader!css-loader!./styles.css";
use(styles);
```

#### Do

```ts
import styles from "./styles.css";
use(styles);
```

### TSX

#### Don't

```tsx
import styles from "style-loader!css-loader!./styles.css";
use(styles);
```

#### Do

```tsx
import styles from "./styles.css";
use(styles);
```
