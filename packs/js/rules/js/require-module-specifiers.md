---
id: js/require-module-specifiers
title: "No empty import specifier list"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/require-module-specifiers]
since: 0.5.0
---

An import does not carry an empty `{}`: `import {} from "m"` is the side-effect import `import "m"`, and `import a, {} from "m"` is `import a from "m"`.

**Why:** An empty list reads as if names were forgotten; the plain form says the module is loaded for its effects.

```codelaw
match:
  any:
    - pattern: "import {} from $S"
      all: [{ pattern: $BARE }]
    - pattern: "import $D, {} from $S"
      all: [{ pattern: $DEF }]
message: "Remove the empty `{}`"
fix:
  - replace: { target: $BARE, with: "import $S;" }
  - replace: { target: $DEF, with: "import $D from $S;" }
```

## Examples

### JavaScript

#### Don't

```js
import {} from "./polyfills.js";
import React, {} from "react";
```

#### Do

```js
import "./polyfills.js";
import React from "react";
```


### JSX

#### Don't

```jsx
import {} from "./polyfills.js";
import React, {} from "react";
```

#### Do

```jsx
import "./polyfills.js";
import React from "react";
```


### TypeScript

#### Don't

```ts
import {} from "./polyfills.js";
import React, {} from "react";
```

#### Do

```ts
import "./polyfills.js";
import React from "react";
```


### TSX

#### Don't

```tsx
import {} from "./polyfills.js";
import React, {} from "react";
```

#### Do

```tsx
import "./polyfills.js";
import React from "react";
```
