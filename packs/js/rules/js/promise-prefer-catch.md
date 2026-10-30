---
id: js/promise-prefer-catch
title: "catch, not a second then argument"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`.catch(b).then(a)` passes the handler's result on to `a` where `.then(a, b)` did not; review the order before accepting"
stability: preview
ported_from: [eslint-plugin-promise/prefer-catch]
since: 0.5.0
---

Rejections are handled with `.catch()`, not with `.then()`'s second argument.

**Why:** A reader looking for error handling looks for `.catch`; the second argument of `.then` is easy to miss.

```codelaw
match:
  pattern: $P.then($A, $B)
message: "Handle the rejection with `.catch()` instead of `.then()`'s second argument"
fix: "$P.catch($B).then($A)"
```

## Examples

### JavaScript

#### Don't

```js
load().then(show, report);
```

#### Do

```js
load().catch(report).then(show);
```

### JSX

#### Don't

```jsx
load().then(show, report);
```

#### Do

```jsx
load().catch(report).then(show);
```

### TypeScript

#### Don't

```ts
load().then(show, report);
```

#### Do

```ts
load().catch(report).then(show);
```

### TSX

#### Don't

```tsx
load().then(show, report);
```

#### Do

```tsx
load().catch(report).then(show);
```
