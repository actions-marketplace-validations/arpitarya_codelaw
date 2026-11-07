---
id: js/no-default-utility-imports
title: "No default import of lodash or underscore"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the functions the file uses are listed by the author"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-default-utility-imports]
since: 0.5.0
---

`lodash`, `lodash-es` and `underscore` are imported by the functions used (`import { debounce } from "lodash-es"`), not as a default.

**Why:** The default import pulls in the whole library, which bundlers can't trim.

```codelaw
match:
  kind: import_statement
  has: { field: source, regex: '^["''](lodash|lodash-es|underscore)["'']$' }
  all:
    - has: { kind: import_clause, has: { kind: identifier } }
message: "Import the functions you use, not the whole library"
```

## How to fix

1. List the functions in braces and drop the `_.` prefix at each use.

## Examples

### JavaScript

#### Don't

```js
import _ from "lodash";
const run = _.debounce(save, 300);
```

#### Do

```js
import { debounce } from "lodash-es";
const run = debounce(save, 300);
```


### JSX

#### Don't

```jsx
import _ from "lodash";
const run = _.debounce(save, 300);
```

#### Do

```jsx
import { debounce } from "lodash-es";
const run = debounce(save, 300);
```


### TypeScript

#### Don't

```ts
import _ from "lodash";
const run = _.debounce(save, 300);
```

#### Do

```ts
import { debounce } from "lodash-es";
const run = debounce(save, 300);
```


### TSX

#### Don't

```tsx
import _ from "lodash";
const run = _.debounce(save, 300);
```

#### Do

```tsx
import { debounce } from "lodash-es";
const run = debounce(save, 300);
```
