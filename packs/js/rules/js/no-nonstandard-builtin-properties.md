---
id: js/no-nonstandard-builtin-properties
title: "No property a built-in does not have"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a typo is renamed to the one standard name within two edits, which the author confirms; a property that is simply not there (an extension added at run time, a value called as a function) has nothing to rename to"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-nonstandard-builtin-properties]
since: 0.5.0
---

A property read, assigned or called on a built-in whose members the language defines is reported when the name is not one of them: `Object.entires()`, `Math.NON_EXISTS_PROPERTY`, `Promise.nonExistsMethod()`, `Array.prototype.customMethod = …`, `'text'.foo()`. A member that exists but is a value (`Math.PI()`, `'text'.length()`, `Symbol.iterator()`) is reported when called, and a static that is not a constructor when used with `new`. Checked: the namespaces and constructors `Object`, `Math`, `Number`, `JSON`, `Reflect`, `Promise`, `Array`, `String`, `Symbol`, `Date`, `BigInt`, `Map` and `Iterator` (also as `globalThis.X`, while the name is not shadowed), the `prototype` of `Object`, `Promise`, `Array`, `String` and `Map`, and string and array literals. Names follow the latest ECMAScript edition and its Stage 3 and 4 proposals; upstream's table of further built-ins (typed arrays, `Intl`, `Temporal`, `URL`, …) is not built, and aliases and dynamic keys are not followed.

**Why:** A misspelt built-in is `undefined` and fails when called, and a method added to a built-in at run time is a global change every other piece of code has to know about.

```codelaw
native: js/no-nonstandard-builtin-properties
```

## How to fix

1. Use the standard name: the built-in has the method under another spelling.
2. If a helper is meant, write it as a function that takes the value, instead of adding a property to a built-in.

## Examples

### JavaScript

#### Don't

```js
const pairs = Object.entires(source);
```

#### Do

```js
const pairs = Object.entries(source);
```

#### Fixed

```js
const pairs = Object.entries(source);
```


### JSX

#### Don't

```jsx
const pairs = Object.entires(source);
```

#### Do

```jsx
const pairs = Object.entries(source);
```

#### Fixed

```jsx
const pairs = Object.entries(source);
```


### TypeScript

#### Don't

```ts
const pairs = Object.entires(source);
```

#### Do

```ts
const pairs = Object.entries(source);
```

#### Fixed

```ts
const pairs = Object.entries(source);
```


### TSX

#### Don't

```tsx
const pairs = Object.entires(source);
```

#### Do

```tsx
const pairs = Object.entries(source);
```

#### Fixed

```tsx
const pairs = Object.entries(source);
```
