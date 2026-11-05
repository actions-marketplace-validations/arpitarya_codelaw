---
id: tailwind/no-concatenated-classes
title: "Classes are not joined with +"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a template literal converts its parts with String(); `+` with an object operand calls valueOf first"
stability: preview
ported_from: [eslint-plugin-better-tailwindcss/no-concatenated-classes]
since: 0.5.0
---

A class list is not built with `+`. Write one template literal, or pass the parts to a class function (`cn`, `clsx`).

**Why:** `+` hides where the spaces go — `"p-4" + size` glues two classes into one — and neither the sorter nor the other rules can read the parts.

```codelaw
native: tailwind/no-concatenated-classes
```

## How to fix

1. Write the chain as one template literal.
2. Or pass the parts to `cn(...)`/`clsx(...)` as separate arguments.

## Examples

### JavaScript

#### Don't

```js
export const box = cn("p-4 " + size);
```

#### Do

```js
export const box = cn(`p-4 ${size}`);
```

### JSX

#### Don't

```jsx
export const Box = ({ size }) => <div className={"p-4 " + size} />;
```

#### Do

```jsx
export const Box = ({ size }) => <div className={`p-4 ${size}`} />;
```

### TypeScript

#### Don't

```ts
export const box = cn("p-4 " + size);
```

#### Do

```ts
export const box = cn(`p-4 ${size}`);
```

### TSX

#### Don't

```tsx
export const Box = ({ size }: { size: string }) => <div className={"p-4 " + size} />;
```

#### Do

```tsx
export const Box = ({ size }: { size: string }) => <div className={`p-4 ${size}`} />;
```
