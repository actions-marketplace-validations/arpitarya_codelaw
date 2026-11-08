---
id: js/no-repeated-call-wrappers
title: "No function whose every call is wrapped the same way"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the wrapper into the function changes what it returns, the author's API"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-repeated-call-wrappers]
since: 0.5.0
---

A local function (a declaration, or a `const`/`let` holding a function or arrow) that is called at least twice, only ever as a direct call, and every time as the sole argument of the same wrapper — `new Set(f(x))`, `new Wrapper(f(x))`, `String(f(x))`, `Number(…)`, `Boolean(…)`, `BigInt(…)` — is reported at its name. Exported, reassigned, async and generator functions, and recursive calls, are left alone.

**Why:** The wrapper is part of what the function produces; written at every call it repeats and can be forgotten.

```codelaw
native: js/no-repeated-call-wrappers
```

## How to fix

1. Do the wrapping inside the function and return its result.

## Examples

### JavaScript

#### Don't

```js
const tags = (text) => text.split(",");
const a = new Set(tags(x));
const b = new Set(tags(y));
```

#### Do

```js
const tags = (text) => new Set(text.split(","));
const a = tags(x);
const b = tags(y);
```


### JSX

#### Don't

```jsx
const tags = (text) => text.split(",");
const a = new Set(tags(x));
const b = new Set(tags(y));
```

#### Do

```jsx
const tags = (text) => new Set(text.split(","));
const a = tags(x);
const b = tags(y);
```


### TypeScript

#### Don't

```ts
const tags = (text) => text.split(",");
const a = new Set(tags(x));
const b = new Set(tags(y));
```

#### Do

```ts
const tags = (text) => new Set(text.split(","));
const a = tags(x);
const b = tags(y);
```


### TSX

#### Don't

```tsx
const tags = (text) => text.split(",");
const a = new Set(tags(x));
const b = new Set(tags(y));
```

#### Do

```tsx
const tags = (text) => new Set(text.split(","));
const a = tags(x);
const b = tags(y);
```
