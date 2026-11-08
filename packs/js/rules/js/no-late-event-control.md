---
id: js/no-late-event-control
title: "No `preventDefault()` or `stopPropagation()` after the handler has awaited"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the call has to move before the first `await`, which reorders the handler"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-late-event-control]
since: 0.5.0
---

`preventDefault()`, `stopPropagation()` and `stopImmediatePropagation()` on an event parameter are called while the event is being dispatched: not after an `await` or `yield` in the handler, not in a loop that awaits, not in a nested function, and not in a generator handler.

**Why:** Once the handler suspends, dispatch has finished and the default action has already happened: the late call does nothing.

```codelaw
native: js/no-late-event-control
```

## How to fix

1. Call it at the start of the handler, before the first `await`.

## Examples

### JavaScript

#### Don't

```js
form.onsubmit = async (event) => {
  await validate();
  event.preventDefault();
};
```

#### Do

```js
form.onsubmit = async (event) => {
  event.preventDefault();
  await validate();
};
```


### JSX

#### Don't

```jsx
form.onsubmit = async (event) => {
  await validate();
  event.preventDefault();
};
```

#### Do

```jsx
form.onsubmit = async (event) => {
  event.preventDefault();
  await validate();
};
```


### TypeScript

#### Don't

```ts
form.onsubmit = async (event) => {
  await validate();
  event.preventDefault();
};
```

#### Do

```ts
form.onsubmit = async (event) => {
  event.preventDefault();
  await validate();
};
```


### TSX

#### Don't

```tsx
form.onsubmit = async (event) => {
  await validate();
  event.preventDefault();
};
```

#### Do

```tsx
form.onsubmit = async (event) => {
  event.preventDefault();
  await validate();
};
```
