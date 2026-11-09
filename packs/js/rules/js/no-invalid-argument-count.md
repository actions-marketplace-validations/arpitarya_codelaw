---
id: js/no-invalid-argument-count
title: "No call with the wrong number of arguments"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which argument is missing or surplus is the author's to say"
fix_exception: value
options: { patterns: { type: "object", default: {}, description: "more APIs to check, as { \"path.to.fn\": count }, a count being a number, a list of allowed numbers or { min, max }; a path starting with `new ` checks a constructor" } }
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-argument-count]
since: 0.5.0
---

A call to one of the language's own functions or constructors with a number of arguments it does not take is reported — in both directions, so `Object.is(a)`, `Promise.allSettled()`, `Math.random(seed)` and `new Set(iterable, extra)` all are. The table is ECMAScript's globals and namespaces (`Object`, `Math`, `Number`, `Array`, `JSON`, `Promise`, `Reflect`, `Symbol`, `Date`, `BigInt`, `Map`/`Set`/`WeakMap`/`WeakSet`/`Proxy`/`DataView`/typed-array constructors, the error constructors, `parseInt`, `setTimeout`, `fetch`, …), only while the root name is the global (not shadowed) and a leading `globalThis.`/`window.` is ignored. A function declared in the file — a function declaration, a `const` function or arrow, or one called in place — is checked for **too many** arguments only (too few is usually a deliberately optional one): not when it has a rest parameter, uses `arguments`, is reassigned, has TypeScript overloads, or the call spreads. With a spread, a built-in is reported only when the arguments before it already exceed the most it takes. The `patterns` option adds APIs. Upstream's further table of web and Node APIs (the DOM, `process`, `Buffer`, events, …) is not built: only the language's own functions are checked.

**Why:** An extra argument is dropped without a word and a missing one arrives as `undefined`; both usually mean a signature changed and a call site did not.

```codelaw
native: js/no-invalid-argument-count
```

## How to fix

1. Pass the arguments the function takes: add the missing ones, or remove the surplus ones.
2. If the function really takes more, give it the parameter (or a rest parameter) so the extra argument is received.

## Examples

### JavaScript

#### Don't

```js
const sum = (a, b) => a + b;
sum(1, 2, 3);
Object.is(value);
```

#### Do

```js
const sum = (a, b) => a + b;
sum(1, 2);
Object.is(value, other);
```


### JSX

#### Don't

```jsx
const sum = (a, b) => a + b;
sum(1, 2, 3);
Object.is(value);
```

#### Do

```jsx
const sum = (a, b) => a + b;
sum(1, 2);
Object.is(value, other);
```


### TypeScript

#### Don't

```ts
const sum = (a, b) => a + b;
sum(1, 2, 3);
Object.is(value);
```

#### Do

```ts
const sum = (a, b) => a + b;
sum(1, 2);
Object.is(value, other);
```


### TSX

#### Don't

```tsx
const sum = (a, b) => a + b;
sum(1, 2, 3);
Object.is(value);
```

#### Do

```tsx
const sum = (a, b) => a + b;
sum(1, 2);
Object.is(value, other);
```
