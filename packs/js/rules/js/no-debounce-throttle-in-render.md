---
id: js/no-debounce-throttle-in-render
title: "No `debounce()` or `throttle()` made while rendering"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where the debounced function lives (module scope, `useMemo`, the constructor) is the author's design"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-debounce-throttle-in-render]
since: 0.5.0
---

A call of lodash's, lodash-es's or underscore's `debounce()` or `throttle()` directly in a function component or hook (a function named `Pascal` or `useX`, also inside `memo()`/`forwardRef()`) or in a React class's `render` is reported. Inside `useMemo`/`useCallback`, as the argument of `useRef()`, or as the lazy `if (!ref.current) ref.current = debounce(…)` it is made once, and is left alone; so are event handlers and effects, which are their own functions.

**Why:** Each render makes a new debounced function with its own timer, so the delay never takes effect.

```codelaw
native: js/no-debounce-throttle-in-render
```

## How to fix

1. Create it once: at module scope, in `useMemo`/`useRef`, or in the class's constructor.

## Examples

### JavaScript

#### Don't

```js
import { debounce } from "lodash";
function Search() {
  const run = debounce(query, 300);
  return run;
}
```

#### Do

```js
import { debounce } from "lodash";
const run = debounce(query, 300);
function Search() {
  return run;
}
```


### JSX

#### Don't

```jsx
import { debounce } from "lodash";
function Search() {
  const run = debounce(query, 300);
  return run;
}
```

#### Do

```jsx
import { debounce } from "lodash";
const run = debounce(query, 300);
function Search() {
  return run;
}
```


### TypeScript

#### Don't

```ts
import { debounce } from "lodash";
function Search() {
  const run = debounce(query, 300);
  return run;
}
```

#### Do

```ts
import { debounce } from "lodash";
const run = debounce(query, 300);
function Search() {
  return run;
}
```


### TSX

#### Don't

```tsx
import { debounce } from "lodash";
function Search() {
  const run = debounce(query, 300);
  return run;
}
```

#### Do

```tsx
import { debounce } from "lodash";
const run = debounce(query, 300);
function Search() {
  return run;
}
```
