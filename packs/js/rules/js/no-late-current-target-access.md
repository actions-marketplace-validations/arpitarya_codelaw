---
id: js/no-late-current-target-access
title: "No `event.currentTarget` after the handler has awaited"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the value has to be read earlier in the handler, which moves code"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-late-current-target-access]
since: 0.5.0
---

`e.currentTarget` (the parameter named `e`, `event`, `evt` or `…Event`, or typed as an event) is read only while the event is being dispatched: not after an `await` or `yield` in the handler, not in a loop that awaits, and not in a nested function (a callback, a timer). By then the browser has set it to `null`.

**Why:** `currentTarget` is only set during dispatch; a read after the handler suspends gets `null` and fails later.

```codelaw
native: js/no-late-current-target-access
```

## How to fix

1. Read `const target = event.currentTarget;` at the start of the handler and use `target` after the `await`.

## Examples

### JavaScript

#### Don't

```js
button.onclick = async (event) => {
  await save();
  event.currentTarget.disabled = false;
};
```

#### Do

```js
button.onclick = async (event) => {
  const target = event.currentTarget;
  await save();
  target.disabled = false;
};
```


### JSX

#### Don't

```jsx
button.onclick = async (event) => {
  await save();
  event.currentTarget.disabled = false;
};
```

#### Do

```jsx
button.onclick = async (event) => {
  const target = event.currentTarget;
  await save();
  target.disabled = false;
};
```


### TypeScript

#### Don't

```ts
button.onclick = async (event) => {
  await save();
  event.currentTarget.disabled = false;
};
```

#### Do

```ts
button.onclick = async (event) => {
  const target = event.currentTarget;
  await save();
  target.disabled = false;
};
```


### TSX

#### Don't

```tsx
button.onclick = async (event) => {
  await save();
  event.currentTarget.disabled = false;
};
```

#### Do

```tsx
button.onclick = async (event) => {
  const target = event.currentTarget;
  await save();
  target.disabled = false;
};
```
