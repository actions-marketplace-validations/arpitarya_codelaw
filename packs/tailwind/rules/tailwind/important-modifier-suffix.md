---
id: tailwind/important-modifier-suffix
title: "The important modifier comes last"
languages: [javascript, jsx, typescript, tsx, html]
fix: auto
stability: preview
ported_from: [eslint-plugin-tailwindcss/enforces-important-modifier-suffix]
since: 0.5.0
---

A class marked important writes `!` at its end — `p-4!`, `md:p-4!` — as Tailwind CSS v4 spells it, not at the start of its utility (`!p-4`, v3's spelling).

**Why:** v4 reads both, but one spelling keeps searches and reviews simple, and the leading form is deprecated in v4.

```codelaw
native: tailwind/important-modifier-suffix
```

## How to fix

1. Move the `!` from the start of the utility to the end of the class.

## Examples

### JavaScript

#### Don't

```js
export const box = cn("!p-4 flex");
```

#### Do

```js
export const box = cn("p-4! flex");
```

### JSX

#### Don't

```jsx
export const Box = () => <div className="!p-4 flex" />;
```

#### Do

```jsx
export const Box = () => <div className="p-4! flex" />;
```

### TypeScript

#### Don't

```ts
export const box = cn("!p-4 flex");
```

#### Do

```ts
export const box = cn("p-4! flex");
```

### TSX

#### Don't

```tsx
export const Box = () => <div className="!p-4 flex" />;
```

#### Do

```tsx
export const Box = () => <div className="p-4! flex" />;
```

### HTML

#### Don't

```html
<div class="!p-4 flex"></div>
```

#### Do

```html
<div class="p-4! flex"></div>
```
