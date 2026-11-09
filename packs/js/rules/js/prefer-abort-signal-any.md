---
id: js/prefer-abort-signal-any
title: "`AbortSignal.any()` for a controller that only forwards abort events"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the controller becomes a signal and may be renamed, its `controller.signal` reads change, and the forwarding statements are removed; the author accepts that"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-abort-signal-any]
since: 0.5.0
---

`const controller = new AbortController()` (alone in its `const`, with no argument, the global `AbortController` and `AbortSignal`) followed directly by statements that only forward `abort` events into it is reported at its name: either two or more `source.addEventListener('abort', () => controller.abort())` statements, or one `for (const signal of [a, b]) signal.addEventListener('abort', () => controller.abort())` over an array the syntax or an annotation shows (a literal, `Array.of`, `Array.from`, `new Array`, a `const` of those, `x as AbortSignal[]`). A source is a name that is `signal` or ends in `Signal`, or a call of `AbortSignal.any()`/`timeout()` whose arguments do nothing; the listener may carry `capture`, `once`, `passive` or a boolean as its options, and its callback may pass `source.reason` to `abort()`. The controller must otherwise be read only as `controller.signal` — a read of its `reason` or `throwIfAborted`, a copy of the signal, a write, or an already aborted source leaves it alone. The suggestion makes the declaration `const signal = AbortSignal.any([a, b])` (`abortController` to `abortSignal`, `controller` to `signal`, when the name is free), reads `.signal` as the new name and removes the forwarding statements; a comment in any of them leaves the finding without an edit. Behaviour of eslint-plugin-unicorn v79.0.0 as it runs with no type information: a source that is not named like a signal, or an iterable that is not shown to be an array, is not reported.

**Why:** One `AbortSignal.any()` says "aborted when any of these is" without an extra controller, forwarding listeners that are never removed, or the risk that the reason is lost on the way.

```codelaw
native: js/prefer-abort-signal-any
```

## Examples

### JavaScript

#### Don't

```js
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
await fetch(url, { signal: abortController.signal });
```

#### Do

```js
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
await fetch(url, { signal: abortSignal });
```

#### Fixed

```js
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
await fetch(url, { signal: abortSignal });
```


### JSX

#### Don't

```jsx
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
await fetch(url, { signal: abortController.signal });
```

#### Do

```jsx
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
await fetch(url, { signal: abortSignal });
```

#### Fixed

```jsx
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
await fetch(url, { signal: abortSignal });
```


### TypeScript

#### Don't

```ts
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
await fetch(url, { signal: abortController.signal });
```

#### Do

```ts
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
await fetch(url, { signal: abortSignal });
```

#### Fixed

```ts
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
await fetch(url, { signal: abortSignal });
```


### TSX

#### Don't

```tsx
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
await fetch(url, { signal: abortController.signal });
```

#### Do

```tsx
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
await fetch(url, { signal: abortSignal });
```

#### Fixed

```tsx
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
await fetch(url, { signal: abortSignal });
```
