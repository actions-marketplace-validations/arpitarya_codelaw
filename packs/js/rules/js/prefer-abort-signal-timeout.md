---
id: js/prefer-abort-signal-timeout
title: "`AbortSignal.timeout()` for a controller aborted only by a timer"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the controller variable becomes a signal and may be renamed, which the author should accept"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-abort-signal-timeout]
since: 0.5.0
---

`const controller = new AbortController()` followed directly by `setTimeout(() => controller.abort(), delay)` (not stored, nothing else in the callback), where the controller is otherwise only read as `controller.signal`, is reported at its name. The suggestion makes it `AbortSignal.timeout(delay)`, renames `controller` to `signal` (`abortController` to `abortSignal`) when the name is free, replaces each `.signal` read and removes the timer.

**Why:** `AbortSignal.timeout()` is the timer and the signal in one, and is cleaned up by the platform.

```codelaw
native: js/prefer-abort-signal-timeout
```

## Examples

### JavaScript

#### Don't

```js
const controller = new AbortController();
setTimeout(() => controller.abort(), 500);
fetch(url, { signal: controller.signal });
```

#### Do

```js
const signal = AbortSignal.timeout(500);
fetch(url, { signal });
```

#### Fixed

```js
const signal = AbortSignal.timeout(500);
fetch(url, { signal: signal });
```


### JSX

#### Don't

```jsx
const controller = new AbortController();
setTimeout(() => controller.abort(), 500);
fetch(url, { signal: controller.signal });
```

#### Do

```jsx
const signal = AbortSignal.timeout(500);
fetch(url, { signal });
```

#### Fixed

```jsx
const signal = AbortSignal.timeout(500);
fetch(url, { signal: signal });
```


### TypeScript

#### Don't

```ts
const controller = new AbortController();
setTimeout(() => controller.abort(), 500);
fetch(url, { signal: controller.signal });
```

#### Do

```ts
const signal = AbortSignal.timeout(500);
fetch(url, { signal });
```

#### Fixed

```ts
const signal = AbortSignal.timeout(500);
fetch(url, { signal: signal });
```


### TSX

#### Don't

```tsx
const controller = new AbortController();
setTimeout(() => controller.abort(), 500);
fetch(url, { signal: controller.signal });
```

#### Do

```tsx
const signal = AbortSignal.timeout(500);
fetch(url, { signal });
```

#### Fixed

```tsx
const signal = AbortSignal.timeout(500);
fetch(url, { signal: signal });
```
