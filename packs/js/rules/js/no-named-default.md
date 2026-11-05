---
id: js/no-named-default
title: "import x rather than import { default as x }"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-named-default]
since: 0.5.0
---

A default import is not written `import { default as x }`.

**Why:** The plain default import says the same and is what every reader expects.

```codelaw
match:
  kind: import_statement
  has:
    field: source
    pattern: $S
  not:
    regex: ^import\s+type
  all:
  - has:
      kind: import_clause
      not:
        has:
          kind: identifier
      has:
        kind: named_imports
        not:
          has:
            nthChild: 2
        has:
          kind: import_specifier
          has:
            field: name
            regex: ^default$
          all:
          - has:
              field: alias
              pattern: $X
message: Write the default import as `import $X from $S`
fix: import $X from $S;
```

## Examples

### JavaScript

#### Don't

```js
import { default as React } from "react";
```

#### Do

```js
import React from "react";
```

### JSX

#### Don't

```jsx
import { default as React } from "react";
```

#### Do

```jsx
import React from "react";
```

### TypeScript

#### Don't

```ts
import { default as React } from "react";
```

#### Do

```ts
import React from "react";
```

### TSX

#### Don't

```tsx
import { default as React } from "react";
```

#### Do

```tsx
import React from "react";
```
