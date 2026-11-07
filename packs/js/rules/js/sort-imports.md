---
id: js/sort-imports
title: "The members of an import in ascending order"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/sort-imports]
since: 0.5.0
---

The names in an `import { … }` are in ascending (code-unit) order by their local name, ESLint's default. Moving specifiers changes nothing at run time, so the fix sorts them. ESLint also orders the import declarations by member syntax; that order is not checked here.

**Why:** Sorted members are easy to scan and merge.

```codelaw
native: js/sort-imports
```

## Examples

### JavaScript

#### Don't

```js
import { useState, useEffect } from "react";
```

#### Do

```js
import { useEffect, useState } from "react";
```


### JSX

#### Don't

```jsx
import { useState, useEffect } from "react";
```

#### Do

```jsx
import { useEffect, useState } from "react";
```


### TypeScript

#### Don't

```ts
import { useState, useEffect } from "react";
```

#### Do

```ts
import { useEffect, useState } from "react";
```


### TSX

#### Don't

```tsx
import { useState, useEffect } from "react";
```

#### Do

```tsx
import { useEffect, useState } from "react";
```
