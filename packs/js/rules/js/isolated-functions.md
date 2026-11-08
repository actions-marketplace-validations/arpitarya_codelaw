---
id: js/isolated-functions
title: "No outside variables in a function that runs somewhere else"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the function needs must be passed in as arguments, which changes its signature"
fix_exception: restructure
options: { functions: { type: "string[]", default: ["makeSynchronous", "workerize"], description: "calls whose function arguments are isolated" }, comments: { type: "string[]", default: ["@isolated"], description: "a comment just before a function that marks it isolated" } }
stability: preview
ported_from: [eslint-plugin-unicorn/isolated-functions]
since: 0.5.0
---

A function that is serialised and run elsewhere — passed to `makeSynchronous()` or `workerize()` (the `functions` option), the first argument of `page.evaluate()` or `browser.execute()`, the `func` of `chrome.scripting.executeScript()`, or marked with an `@isolated` comment — refers to nothing outside itself. Each reference to a variable declared outside it, each write to a global, and each `this` or `super` is reported. Globals may be read: the language's own (`Math`, `JSON`, …) and, since codelaw does not know which environment the function runs in, any name the file does not declare (`document` in a page, `self` in a worker) — upstream reports those unless the ESLint config declares them. ESLint's `selectors` and `overrideGlobals` options are not taken.

**Why:** The function's source text runs in another realm (a worker, a page, a tab), where the outer variables do not exist.

```codelaw
native: js/isolated-functions
```

## How to fix

1. Pass what the function needs as arguments, or define it inside the function.

## Examples

### JavaScript

#### Don't

```js
const factor = 2;
makeSynchronous(async (n) => n * factor);
```

#### Do

```js
makeSynchronous(async (n, factor) => n * factor);
```


### JSX

#### Don't

```jsx
const factor = 2;
makeSynchronous(async (n) => n * factor);
```

#### Do

```jsx
makeSynchronous(async (n, factor) => n * factor);
```


### TypeScript

#### Don't

```ts
const factor = 2;
makeSynchronous(async (n) => n * factor);
```

#### Do

```ts
makeSynchronous(async (n, factor) => n * factor);
```


### TSX

#### Don't

```tsx
const factor = 2;
makeSynchronous(async (n) => n * factor);
```

#### Do

```tsx
makeSynchronous(async (n, factor) => n * factor);
```
