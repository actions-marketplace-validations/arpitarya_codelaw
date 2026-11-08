---
id: js/avoid-mutating-nested-properties-of-shallow-clones
title: "No changing a nested object of a shallow clone"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a deep clone copies more than the shallow one did and drops prototypes the shallow clone kept, which the author should accept"
stability: preview
ported_from: [eslint-plugin-sonarjs/avoid-mutating-nested-properties-of-shallow-clones]
since: 0.5.0
---

A variable made once by lodash's, lodash-es's or underscore's `clone()` (imported by name, as `_`, or as `lodash/clone`) and never reassigned shares its nested objects with the original. Assigning, incrementing or deleting two or more levels down (`copy.meta.tag = …`) is reported, unless an earlier statement gave that level a fresh value (`copy.meta = {}`, a clone, or a deep `structuredClone()`/`cloneDeep()` above it). The suggestion makes the clone `structuredClone(…)`.

**Why:** The change reaches the original object too, which the clone was made to protect.

```codelaw
native: js/avoid-mutating-nested-properties-of-shallow-clones
```

## Examples

### JavaScript

#### Don't

```js
import _ from "lodash";
const copy = _.clone(settings);
copy.theme.color = "red";
```

#### Do

```js
import _ from "lodash";
const copy = _.clone(settings);
copy.theme = { ...copy.theme, color: "red" };
```

#### Fixed

```js
import _ from "lodash";
const copy = structuredClone(settings);
copy.theme.color = "red";
```


### JSX

#### Don't

```jsx
import _ from "lodash";
const copy = _.clone(settings);
copy.theme.color = "red";
```

#### Do

```jsx
import _ from "lodash";
const copy = _.clone(settings);
copy.theme = { ...copy.theme, color: "red" };
```

#### Fixed

```jsx
import _ from "lodash";
const copy = structuredClone(settings);
copy.theme.color = "red";
```


### TypeScript

#### Don't

```ts
import _ from "lodash";
const copy = _.clone(settings);
copy.theme.color = "red";
```

#### Do

```ts
import _ from "lodash";
const copy = _.clone(settings);
copy.theme = { ...copy.theme, color: "red" };
```

#### Fixed

```ts
import _ from "lodash";
const copy = structuredClone(settings);
copy.theme.color = "red";
```


### TSX

#### Don't

```tsx
import _ from "lodash";
const copy = _.clone(settings);
copy.theme.color = "red";
```

#### Do

```tsx
import _ from "lodash";
const copy = _.clone(settings);
copy.theme = { ...copy.theme, color: "red" };
```

#### Fixed

```tsx
import _ from "lodash";
const copy = structuredClone(settings);
copy.theme.color = "red";
```
