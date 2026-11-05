---
id: ts/no-require-imports
title: "No require() calls"
languages: [typescript, tsx]
fix: suggest
fix_reason: "default import, namespace import and `import x = require()` differ in interop and hoisting, so the reviewer picks one (offered for a top-level `const x = require('m')`)"
stability: preview
ported_from: [typescript-eslint/no-require-imports]
since: 0.1.0
---

Modules are loaded with `import`, not `require()`.

**Why:** `require()` is untyped and invisible to TypeScript's module checks.

```codelaw
match:
  pattern: require($$$ARGS)
  any:
    - has: { field: arguments, has: { kind: string, pattern: $M } }
      inside:
        kind: variable_declarator
        stopBy: neighbor
        has: { field: name, kind: identifier, pattern: $X }
        inside:
          kind: lexical_declaration
          stopBy: neighbor
          pattern: $DECL
          regex: '^const\b'
          not: { has: { kind: variable_declarator, nthChild: 2 } }
          inside: { kind: program, stopBy: neighbor }
    - pattern: require($$$ARGS)
message: "Use `import` instead of `require()`"
fix:
  choices:
    - label: default import
      default: true
      fix:
        - replace: { target: $DECL, with: "import $X from $M;" }
    - label: namespace import
      fix:
        - replace: { target: $DECL, with: "import * as $X from $M;" }
    - label: import equals
      fix:
        - replace: { target: $DECL, with: "import $X = require($M);" }
```

## How to fix

1. Replace the call with an `import` of the names the code uses.

## Examples

### TypeScript

#### Don't

```ts
const fs = require("node:fs");
```

#### Fixed (default import)

```ts
import fs from "node:fs";
```

#### Fixed (namespace import)

```ts
import * as fs from "node:fs";
```

#### Fixed (import equals)

```ts
import fs = require("node:fs");
```

#### Do

```ts
import fs from "node:fs";
```

### TSX

#### Don't

```tsx
const fs = require("node:fs");
```

#### Fixed (default import)

```tsx
import fs from "node:fs";
```

#### Fixed (namespace import)

```tsx
import * as fs from "node:fs";
```

#### Fixed (import equals)

```tsx
import fs = require("node:fs");
```

#### Do

```tsx
import fs from "node:fs";
```
