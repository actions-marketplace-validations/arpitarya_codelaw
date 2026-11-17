---
id: js/import-prefer-namespace-import
title: "Namespace imports for the modules the project lists"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
options:
  patterns: { type: "string[]", default: [], description: "modules to import as a namespace: the name as written, or a regular expression as `/^react(-dom)?$/`" }
ported_from: [eslint-plugin-import-x/prefer-namespace-import]
since: 0.5.2
---

A default import from a module in `patterns` is written as a namespace import: `import React from "react"` becomes `import * as React from "react"`. An entry is the module name as written, or `/body/flags` for a regular expression (`i` is the only flag read). With a named import beside it, `import React, { useState } from "react"` becomes two statements, one `* as React` and one with the named imports; a default import beside a namespace import is reported without a fix. eslint-plugin-import-x 4.17.1's rule, read and reimplemented.

With no `patterns` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** Some modules (`react`, `react-dom`) are used through their namespace, and one import style for them keeps usages greppable.

```codelaw
native: js/import-prefer-namespace-import
```

## Examples

### JavaScript

#### Don't

```js options={"patterns":["react"]}
import React from "react";
```

#### Do

```js options={"patterns":["react"]}
import * as React from "react";
```

#### Fixed

```js options={"patterns":["react"]}
import * as React from "react";
```

### JSX

#### Don't

```jsx options={"patterns":["react"]}
import React from "react";
```

#### Do

```jsx options={"patterns":["react"]}
import * as React from "react";
```

#### Fixed

```jsx options={"patterns":["react"]}
import * as React from "react";
```

### TypeScript

#### Don't

```ts options={"patterns":["react"]}
import React from "react";
```

#### Do

```ts options={"patterns":["react"]}
import * as React from "react";
```

#### Fixed

```ts options={"patterns":["react"]}
import * as React from "react";
```

### TSX

#### Don't

```tsx options={"patterns":["react"]}
import React from "react";
```

#### Do

```tsx options={"patterns":["react"]}
import * as React from "react";
```

#### Fixed

```tsx options={"patterns":["react"]}
import * as React from "react";
```
