---
id: js/jsdoc-require-property
title: "An object @typedef lists its properties"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the property names and their descriptions are the author's"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-property]
since: 0.5.0
---

A `@typedef` or `@namespace` whose type is a plain object (`object`, `Object`, `PlainObject`) has `@property` tags.

**Why:** An object type with no listed properties tells the reader and the type checker nothing about its shape.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:typedef|namespace)[ \t]*\{(?:object|Object|PlainObject)\}
  - not:
      regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:property|prop)\b
message: List this object type's properties with `@property` tags
```

## How to fix

1. Add a `@property {type} name description` line for each property of the object.

## Examples

### JavaScript

#### Don't

```js
/**
 * @typedef {object} Point
 */
```

#### Do

```js
/**
 * @typedef {object} Point
 * @property {number} x Across.
 * @property {number} y Down.
 */
```

### JSX

#### Don't

```jsx
/**
 * @typedef {object} Point
 */
```

#### Do

```jsx
/**
 * @typedef {object} Point
 * @property {number} x Across.
 * @property {number} y Down.
 */
```

### TypeScript

#### Don't

```ts
/**
 * @typedef {object} Point
 */
```

#### Do

```ts
/**
 * @typedef {object} Point
 * @property {number} x Across.
 * @property {number} y Down.
 */
```

### TSX

#### Don't

```tsx
/**
 * @typedef {object} Point
 */
```

#### Do

```tsx
/**
 * @typedef {object} Point
 * @property {number} x Across.
 * @property {number} y Down.
 */
```
