---
id: js/too-many-break-or-continue-in-loop
title: "A loop is left by at most one break or continue"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "folding the exits into the loop condition is a design change"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/too-many-break-or-continue-in-loop]
since: 0.5.0
---

A loop body with more than one `break` or `continue` that leaves or restarts this loop is reported. Jumps inside a nested loop or function belong to it, and a plain `break` inside a `switch` leaves the `switch`, so none of those count.

**Why:** Each extra exit is another path the reader has to trace to know when the loop stops.

```codelaw
native: js/too-many-break-or-continue-in-loop
```

## How to fix

1. Filter the items first, or move the exit tests into the loop's condition, so one `break` or `continue` remains.

## Examples

### JavaScript

#### Don't

```js
for (const item of items) {
  if (item.skip) continue;
  if (item.last) break;
  use(item);
}
```

#### Do

```js
for (const item of items.filter((i) => !i.skip)) {
  if (item.last) break;
  use(item);
}
```


### JSX

#### Don't

```jsx
for (const item of items) {
  if (item.skip) continue;
  if (item.last) break;
  use(item);
}
```

#### Do

```jsx
for (const item of items.filter((i) => !i.skip)) {
  if (item.last) break;
  use(item);
}
```


### TypeScript

#### Don't

```ts
for (const item of items) {
  if (item.skip) continue;
  if (item.last) break;
  use(item);
}
```

#### Do

```ts
for (const item of items.filter((i) => !i.skip)) {
  if (item.last) break;
  use(item);
}
```


### TSX

#### Don't

```tsx
for (const item of items) {
  if (item.skip) continue;
  if (item.last) break;
  use(item);
}
```

#### Do

```tsx
for (const item of items.filter((i) => !i.skip)) {
  if (item.last) break;
  use(item);
}
```
