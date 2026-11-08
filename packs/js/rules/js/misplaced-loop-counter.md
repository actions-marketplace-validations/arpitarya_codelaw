---
id: js/misplaced-loop-counter
title: "No `for` loop whose update changes something its condition doesn't test"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which of the two, the condition or the update, is wrong is the author's knowledge"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/misplaced-loop-counter]
since: 0.5.0
---

A `for` loop whose update (`i++`, `i = …`, `x.next()`) changes nothing its condition reads is reported at `for`: the counter being changed is not the one being tested. A loop without a condition, or whose update changes nothing (a plain call), is left alone.

**Why:** Such a loop never ends, or ends for a reason the reader can't see.

```codelaw
native: js/misplaced-loop-counter
```

## How to fix

1. Update the variable the condition tests, or test the one the update changes.

## Examples

### JavaScript

#### Don't

```js
for (let i = 0; i < n; j++) {
  use(i);
}
```

#### Do

```js
for (let i = 0; i < n; i++) {
  use(i);
}
```


### JSX

#### Don't

```jsx
for (let i = 0; i < n; j++) {
  use(i);
}
```

#### Do

```jsx
for (let i = 0; i < n; i++) {
  use(i);
}
```


### TypeScript

#### Don't

```ts
for (let i = 0; i < n; j++) {
  use(i);
}
```

#### Do

```ts
for (let i = 0; i < n; i++) {
  use(i);
}
```


### TSX

#### Don't

```tsx
for (let i = 0; i < n; j++) {
  use(i);
}
```

#### Do

```tsx
for (let i = 0; i < n; i++) {
  use(i);
}
```
