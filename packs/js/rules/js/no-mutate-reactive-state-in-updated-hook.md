---
id: js/no-mutate-reactive-state-in-updated-hook
title: "No changing reactive state in Vue's `updated` hook"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where the state change belongs (a watcher, a computed value, an event) is the author's design"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-mutate-reactive-state-in-updated-hook]
since: 0.5.0
---

In an `onUpdated()` callback, or the `updated` member of a component object (`export default { … }`, `defineComponent({ … })`), an assignment to `x.value` of a `ref()` or to a property of a `reactive()` (both imported from `vue`) is reported when it always changes the value: an increment, a compound assignment, or a value computed from itself, `Date.now()`, `Math.random()` or `performance.now()`. A change under a condition or after an early return, and a constant assignment, are left alone.

**Why:** The change re-renders the component, which runs the `updated` hook again: an endless update loop.

```codelaw
native: js/no-mutate-reactive-state-in-updated-hook
```

## How to fix

1. Move the change to a watcher, a computed value or the event that causes it, or guard it so it settles.

## Examples

### JavaScript

#### Don't

```js
import { ref, onUpdated } from "vue";
const renders = ref(0);
onUpdated(() => {
  renders.value += 1;
});
```

#### Do

```js
import { ref, onUpdated } from "vue";
const renders = ref(0);
onUpdated(() => {
  renders.value = 5;
});
```


### JSX

#### Don't

```jsx
import { ref, onUpdated } from "vue";
const renders = ref(0);
onUpdated(() => {
  renders.value += 1;
});
```

#### Do

```jsx
import { ref, onUpdated } from "vue";
const renders = ref(0);
onUpdated(() => {
  renders.value = 5;
});
```


### TypeScript

#### Don't

```ts
import { ref, onUpdated } from "vue";
const renders = ref(0);
onUpdated(() => {
  renders.value += 1;
});
```

#### Do

```ts
import { ref, onUpdated } from "vue";
const renders = ref(0);
onUpdated(() => {
  renders.value = 5;
});
```


### TSX

#### Don't

```tsx
import { ref, onUpdated } from "vue";
const renders = ref(0);
onUpdated(() => {
  renders.value += 1;
});
```

#### Do

```tsx
import { ref, onUpdated } from "vue";
const renders = ref(0);
onUpdated(() => {
  renders.value = 5;
});
```
