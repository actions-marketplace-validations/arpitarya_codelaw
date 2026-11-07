---
id: js/prefer-temporal
title: "Temporal.Now rather than Date.now()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-temporal]
since: 0.5.0
---

`Date.now()` is `Temporal.Now.instant().epochMilliseconds`: the same number of milliseconds since the epoch, read through the API that replaces `Date`. Upstream also reports `new Date(…)`; its Temporal form depends on what the date is for, and is not reported here. Temporal needs Node.js 26 or a recent browser, or its polyfill.

**Why:** Temporal is the standard date-time API, with explicit types for instants, dates and zones.

```codelaw
match:
  pattern: "Date.now()"
message: "Read the time through `Temporal.Now`"
fix: "Temporal.Now.instant().epochMilliseconds"
```

## Examples

### JavaScript

#### Don't

```js
const started = Date.now();
```

#### Do

```js
const started = Temporal.Now.instant().epochMilliseconds;
```


### JSX

#### Don't

```jsx
const started = Date.now();
```

#### Do

```jsx
const started = Temporal.Now.instant().epochMilliseconds;
```


### TypeScript

#### Don't

```ts
const started = Date.now();
```

#### Do

```ts
const started = Temporal.Now.instant().epochMilliseconds;
```


### TSX

#### Don't

```tsx
const started = Date.now();
```

#### Do

```tsx
const started = Temporal.Now.instant().epochMilliseconds;
```
