---
id: tailwind/no-restricted-classes
title: "No classes the project restricts"
languages: [javascript, jsx, typescript, tsx, html]
fix: suggest
fix_reason: "removing a class changes how the element looks; a person accepts the diff"
stability: preview
options:
  restrict: { type: "string[]", default: [], description: "regular expressions, each matched against a whole class" }
ported_from: [eslint-plugin-better-tailwindcss/no-restricted-classes]
since: 0.5.0
---

A class matching one of `restrict`'s patterns is not used. With no patterns the rule finds nothing.

**Why:** A team retires classes — `float-*`, a colour leaving the palette — and needs the stragglers found.

```codelaw
native: tailwind/no-restricted-classes
```

## How to fix

1. Remove the class, or replace it with the one the project uses instead.

## Examples

### JavaScript

#### Don't

```js options={"restrict":["float-.*"]}
export const box = cn("float-left p-4");
```

#### Do

```js options={"restrict":["float-.*"]}
export const box = cn("p-4");
```

### JSX

#### Don't

```jsx options={"restrict":["float-.*"]}
export const Box = () => <div className="float-left p-4" />;
```

#### Do

```jsx options={"restrict":["float-.*"]}
export const Box = () => <div className="p-4" />;
```

### TypeScript

#### Don't

```ts options={"restrict":["float-.*"]}
export const box = cn("float-left p-4");
```

#### Do

```ts options={"restrict":["float-.*"]}
export const box = cn("p-4");
```

### TSX

#### Don't

```tsx options={"restrict":["float-.*"]}
export const Box = () => <div className="float-left p-4" />;
```

#### Do

```tsx options={"restrict":["float-.*"]}
export const Box = () => <div className="p-4" />;
```

### HTML

#### Don't

```html options={"restrict":["float-.*"]}
<div class="float-left p-4"></div>
```

#### Do

```html options={"restrict":["float-.*"]}
<div class="p-4"></div>
```
