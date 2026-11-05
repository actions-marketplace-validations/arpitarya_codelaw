---
id: tailwind/enforces-negative-arbitrary-values
title: "A negative arbitrary value keeps its minus inside the brackets"
languages: [javascript, jsx, typescript, tsx, html]
fix: auto
stability: preview
ported_from: [eslint-plugin-tailwindcss/enforces-negative-arbitrary-values]
since: 0.5.0
---

A negative arbitrary value is written `m-[-4px]`, not `-m-[4px]`.

**Why:** Tailwind CSS v4 canonicalises the value form; the prefix form is an older spelling that mixes two ways of saying the same thing.

```codelaw
native: tailwind/enforces-negative-arbitrary-values
```

## How to fix

1. Drop the leading `-` and put it inside the brackets: `-m-[4px]` → `m-[-4px]`.

## Examples

### JavaScript

#### Don't

```js
export const box = cn("-m-[4px] flex");
```

#### Do

```js
export const box = cn("m-[-4px] flex");
```

### JSX

#### Don't

```jsx
export const Box = () => <div className="-m-[4px] flex" />;
```

#### Do

```jsx
export const Box = () => <div className="m-[-4px] flex" />;
```

### TypeScript

#### Don't

```ts
export const box = cn("-m-[4px] flex");
```

#### Do

```ts
export const box = cn("m-[-4px] flex");
```

### TSX

#### Don't

```tsx
export const Box = () => <div className="-m-[4px] flex" />;
```

#### Do

```tsx
export const Box = () => <div className="m-[-4px] flex" />;
```

### HTML

#### Don't

```html
<div class="-m-[4px] flex"></div>
```

#### Do

```html
<div class="m-[-4px] flex"></div>
```
