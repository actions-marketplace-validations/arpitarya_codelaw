---
id: js/prefer-set-has
title: "A `Set` for a list that is only searched"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { minimumItems: { type: "integer", default: 0, description: "report only arrays known to hold at least this many items; 0 reports any size" } }
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-set-has]
since: 0.5.0
---

A variable initialised with an array (a literal, `Array()`, `Array.from()`/`Array.of()`, or a method that returns a new array: `map`, `filter`, `split`, …) whose every use is `x.includes(value)` is reported at its name. A single `includes()` counts only when it runs repeatedly — in a loop or a function below the declaration. For a literal of unique primitives, `x.length`, `for…of`, spreading and `forEach` with one parameter may be used too. The fix wraps the value in `new Set(…)`, renames `includes` to `has` and `length` to `size`, and turns a `T[]` or `Array<T>` annotation into `Set<T>` (another annotation makes it a suggestion). An exported or reassigned variable is left alone.

**Why:** `includes()` scans the whole array each time; `Set.has()` is a lookup.

```codelaw
native: js/prefer-set-has
```

## Examples

### JavaScript

#### Don't

```js
const allowed = ["a", "b"];
const ok = (v) => allowed.includes(v);
```

#### Do

```js
const allowed = new Set(["a", "b"]);
const ok = (v) => allowed.has(v);
```

#### Fixed

```js
const allowed = new Set(["a", "b"]);
const ok = (v) => allowed.has(v);
```


### JSX

#### Don't

```jsx
const allowed = ["a", "b"];
const ok = (v) => allowed.includes(v);
```

#### Do

```jsx
const allowed = new Set(["a", "b"]);
const ok = (v) => allowed.has(v);
```

#### Fixed

```jsx
const allowed = new Set(["a", "b"]);
const ok = (v) => allowed.has(v);
```


### TypeScript

#### Don't

```ts
const allowed = ["a", "b"];
const ok = (v) => allowed.includes(v);
```

#### Do

```ts
const allowed = new Set(["a", "b"]);
const ok = (v) => allowed.has(v);
```

#### Fixed

```ts
const allowed = new Set(["a", "b"]);
const ok = (v) => allowed.has(v);
```


### TSX

#### Don't

```tsx
const allowed = ["a", "b"];
const ok = (v) => allowed.includes(v);
```

#### Do

```tsx
const allowed = new Set(["a", "b"]);
const ok = (v) => allowed.has(v);
```

#### Fixed

```tsx
const allowed = new Set(["a", "b"]);
const ok = (v) => allowed.has(v);
```
