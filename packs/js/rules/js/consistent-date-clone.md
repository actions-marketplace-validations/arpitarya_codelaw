---
id: js/consistent-date-clone
title: "Clone a Date without getTime()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-date-clone]
since: 0.5.0
---

A `Date` is copied with `new Date(date)`, not `new Date(date.getTime())`.

**Why:** The constructor already reads the timestamp of a `Date` it is given; the extra call is noise.

```codelaw

match:
  pattern: new Date($D.getTime())
message: "`new Date($D)` clones the date; `.getTime()` is not needed"
fix: "new Date($D)"
```

## Examples

### JavaScript

#### Don't

```js
const copy = new Date(start.getTime());
```

#### Do

```js
const copy = new Date(start);
```

### JSX

#### Don't

```jsx
const copy = new Date(start.getTime());
```

#### Do

```jsx
const copy = new Date(start);
```

### TypeScript

#### Don't

```ts
const copy = new Date(start.getTime());
```

#### Do

```ts
const copy = new Date(start);
```

### TSX

#### Don't

```tsx
const copy = new Date(start.getTime());
```

#### Do

```tsx
const copy = new Date(start);
```
