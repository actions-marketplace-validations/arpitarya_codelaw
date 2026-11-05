---
id: tailwind/enforces-canonical-classname
title: "Classes are written as Tailwind spells them"
languages: [javascript, jsx, typescript, tsx, html]
fix: auto
stability: preview
ported_from: [eslint-plugin-tailwindcss/enforces-canonical-classname]
since: 0.5.0
---

A class is written in the form the project's Tailwind CSS calls canonical: `p-4`, not `p-[1rem]`; `p-4!`, not `!p-4`.

**Why:** The canonical form names the theme value, and two spellings of one class make the same style look different.

It reads the class table `@codelaw/provider-tailwind` builds from the project's own `tailwindcss`; without the table it finds nothing, and the run says why (`[providers] on_missing`).

```codelaw
native: tailwind/enforces-canonical-classname
```

## How to fix

1. Write the class Tailwind's canonical way, as the finding names it.

## Examples

### JavaScript

#### Don't

```js
export const box = cn("p-[1rem] flex");
```

#### Do

```js
export const box = cn("p-4 flex");
```

### JSX

#### Don't

```jsx
export const Box = () => <div className="p-[1rem] flex" />;
```

#### Do

```jsx
export const Box = () => <div className="p-4 flex" />;
```

### TypeScript

#### Don't

```ts
export const box = cn("p-[1rem] flex");
```

#### Do

```ts
export const box = cn("p-4 flex");
```

### TSX

#### Don't

```tsx
export const Box = () => <div className="p-[1rem] flex" />;
```

#### Do

```tsx
export const Box = () => <div className="p-4 flex" />;
```

### HTML

#### Don't

```html
<div class="p-[1rem] flex"></div>
```

#### Do

```html
<div class="p-4 flex"></div>
```
