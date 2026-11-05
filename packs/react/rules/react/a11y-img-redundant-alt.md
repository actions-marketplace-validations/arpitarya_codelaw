---
id: react/a11y-img-redundant-alt
title: "Alt text does not say \"image\""
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the better description is the author's"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/img-redundant-alt]
since: 0.5.0
---

An `<img>`'s `alt` does not contain the words image, photo or picture.

**Why:** Screen readers already announce an image as one; "image of" is said twice.

```codelaw
match:
  kind: jsx_attribute
  regex: '^alt=["''].*\b([Ii][Mm][Aa][Gg][Ee]|[Pp][Hh][Oo][Tt][Oo]|[Pp][Ii][Cc][Tt][Uu][Rr][Ee])\b'
  inside:
    any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
    has: { field: name, regex: '^img$' }
    not: { has: { kind: jsx_attribute, regex: '^aria-hidden(=(["'']true|\{\s*true)|$)' } }
message: "Drop the word image, photo or picture from the `alt`; screen readers already say it"
```

## How to fix

1. Describe what the image shows without saying it is an image.

## Examples

### JavaScript

#### Don't

```js
const p = <img src="cat.jpg" alt="Photo of a cat" />;
```

#### Do

```js
const p = <img src="cat.jpg" alt="A cat asleep on a sofa" />;
```

### JSX

#### Don't

```jsx
const p = <img src="cat.jpg" alt="Photo of a cat" />;
```

#### Do

```jsx
const p = <img src="cat.jpg" alt="A cat asleep on a sofa" />;
```

### TSX

#### Don't

```tsx
const p = <img src="cat.jpg" alt="Photo of a cat" />;
```

#### Do

```tsx
const p = <img src="cat.jpg" alt="A cat asleep on a sofa" />;
```
