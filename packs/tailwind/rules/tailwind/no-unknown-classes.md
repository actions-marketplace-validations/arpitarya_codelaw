---
id: tailwind/no-unknown-classes
title: "Every class is one the project's Tailwind knows"
languages: [javascript, jsx, typescript, tsx, html]
fix: suggest
fix_reason: "an unknown class may be styled by the project's own CSS or read by a script; removing it needs a person"
stability: preview
options:
  ignore: { type: "string[]", default: [], description: "regular expressions for the project's own classes, each matched against a whole class" }
ported_from: [eslint-plugin-better-tailwindcss/no-unknown-classes]
since: 0.5.0
---

Every class in a class list is one the project's own Tailwind CSS generates, its theme and plugins included, as `@codelaw/provider-tailwind` reports. Name the project's own classes in `ignore`.

**Why:** A typo (`fleex`) or a class from another version silently styles nothing.

It reads the class table `@codelaw/provider-tailwind` builds from the project's own `tailwindcss`; without the table it finds nothing, and the run says why (`[providers] on_missing`).

```codelaw
native: tailwind/no-unknown-classes
```

## How to fix

1. Fix the typo, or remove the class.
2. If the project's CSS defines it, add a pattern for it to `ignore`.

## Examples

### JavaScript

#### Don't

```js
export const box = cn("flex fleex");
```

#### Do

```js
export const box = cn("flex");
```

### JSX

#### Don't

```jsx
export const Box = () => <div className="flex fleex" />;
```

#### Do

```jsx
export const Box = () => <div className="flex" />;
```

### TypeScript

#### Don't

```ts
export const box = cn("flex fleex");
```

#### Do

```ts
export const box = cn("flex");
```

### TSX

#### Don't

```tsx
export const Box = () => <div className="flex fleex" />;
```

#### Do

```tsx
export const Box = () => <div className="flex" />;
```

### HTML

#### Don't

```html
<div class="flex fleex"></div>
```

#### Do

```html
<div class="flex"></div>
```
