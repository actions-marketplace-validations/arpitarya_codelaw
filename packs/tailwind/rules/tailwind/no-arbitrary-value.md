---
id: tailwind/no-arbitrary-value
title: "No arbitrary values"
languages: [javascript, jsx, typescript, tsx, html]
fix: manual
fix_reason: "the right theme value is the author's choice; the code holds only the arbitrary one"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-tailwindcss/no-arbitrary-value]
since: 0.5.0
---

A class takes its value from the theme (`w-4`), not an arbitrary one in brackets (`w-[13px]`).

**Why:** Arbitrary values bypass the design scale; every one is a size, colour or spacing nobody chose for the system.

```codelaw
native: tailwind/no-arbitrary-value
```

## How to fix

1. Pick the theme value closest to the intent (`w-[13px]` → `w-3` or `w-3.5`).
2. If the system needs the value, add it to the theme (`@theme { --spacing-… }`) and use its name.

## Examples

### JavaScript

#### Don't

```js
export const box = cn("w-[13px] flex");
```

#### Do

```js
export const box = cn("w-4 flex");
```

### JSX

#### Don't

```jsx
export const Box = () => <div className="w-[13px] flex" />;
```

#### Do

```jsx
export const Box = () => <div className="w-4 flex" />;
```

### TypeScript

#### Don't

```ts
export const box = cn("w-[13px] flex");
```

#### Do

```ts
export const box = cn("w-4 flex");
```

### TSX

#### Don't

```tsx
export const Box = () => <div className="w-[13px] flex" />;
```

#### Do

```tsx
export const Box = () => <div className="w-4 flex" />;
```

### HTML

#### Don't

```html
<div class="w-[13px] flex"></div>
```

#### Do

```html
<div class="w-4 flex"></div>
```
