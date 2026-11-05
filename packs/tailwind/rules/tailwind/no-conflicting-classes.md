---
id: tailwind/no-conflicting-classes
title: "No two classes set the same properties"
languages: [javascript, jsx, typescript, tsx, html]
fix: suggest
fix_reason: "the losing class does nothing in Tailwind's own CSS, but a selector of the project's could still read it"
stability: preview
ported_from: [eslint-plugin-better-tailwindcss/no-conflicting-classes]
since: 0.5.0
---

Two classes in one list do not set exactly the same CSS properties under the same variants (`p-2 p-4`). Tailwind's order decides which one wins, not the order they are written in; the other does nothing.

**Why:** The class written last looks like the winner and often is not.

It reads the class table `@codelaw/provider-tailwind` builds from the project's own `tailwindcss`; without the table it finds nothing, and the run says why (`[providers] on_missing`).

```codelaw
native: tailwind/no-conflicting-classes
```

## How to fix

1. Keep the class you mean and remove the other.

## Examples

### JavaScript

#### Don't

```js
export const box = cn("p-2 p-4");
```

#### Do

```js
export const box = cn("p-4");
```

### JSX

#### Don't

```jsx
export const Box = () => <div className="p-2 p-4" />;
```

#### Do

```jsx
export const Box = () => <div className="p-4" />;
```

### TypeScript

#### Don't

```ts
export const box = cn("p-2 p-4");
```

#### Do

```ts
export const box = cn("p-4");
```

### TSX

#### Don't

```tsx
export const Box = () => <div className="p-2 p-4" />;
```

#### Do

```tsx
export const Box = () => <div className="p-4" />;
```

### HTML

#### Don't

```html
<div class="p-2 p-4"></div>
```

#### Do

```html
<div class="p-4"></div>
```
