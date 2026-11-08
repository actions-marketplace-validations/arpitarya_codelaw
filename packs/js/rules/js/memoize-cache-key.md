---
id: js/memoize-cache-key
title: "No `memoize()` of a function of several arguments without a resolver"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what makes two calls the same (the key) depends on the arguments' meaning"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/memoize-cache-key]
since: 0.5.0
---

lodash's, lodash-es's or underscore's `memoize(fn)` with no resolver (or `undefined`/`null`) keys its cache by the first argument only. When `fn` — inline, a function declaration or a variable bound to one — takes two or more parameters, or a rest parameter, the call is reported at `memoize`.

**Why:** Calls that differ only in a later argument return the first call's cached result.

```codelaw
native: js/memoize-cache-key
```

## How to fix

1. Pass a resolver that builds the key from every argument: `memoize(fn, (a, b) => `${a}:${b}`)`.

## Examples

### JavaScript

#### Don't

```js
import _ from "lodash";
const add = _.memoize((a, b) => a + b);
```

#### Do

```js
import _ from "lodash";
const add = _.memoize((a, b) => a + b, (a, b) => `${a}:${b}`);
```


### JSX

#### Don't

```jsx
import _ from "lodash";
const add = _.memoize((a, b) => a + b);
```

#### Do

```jsx
import _ from "lodash";
const add = _.memoize((a, b) => a + b, (a, b) => `${a}:${b}`);
```


### TypeScript

#### Don't

```ts
import _ from "lodash";
const add = _.memoize((a, b) => a + b);
```

#### Do

```ts
import _ from "lodash";
const add = _.memoize((a, b) => a + b, (a, b) => `${a}:${b}`);
```


### TSX

#### Don't

```tsx
import _ from "lodash";
const add = _.memoize((a, b) => a + b);
```

#### Do

```tsx
import _ from "lodash";
const add = _.memoize((a, b) => a + b, (a, b) => `${a}:${b}`);
```
