---
id: js/jsdoc-require-property-name
title: "@property names its property"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which property the tag is for is the author's to say"
fix_pending: "suggest"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-property-name]
since: 0.5.0
---

A JSDoc `@property` tag names the property it documents.

**Why:** A `@property` with only a type adds no property to the type, so tools ignore it.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:property|prop)\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\})?[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
message: Name the property this `@property` documents
```

## How to fix

1. Write the property's name after the `@property` type.

## Examples

### JavaScript

#### Don't

```js
/**
 * @typedef {object} Point
 * @property {number}
 */
```

#### Do

```js
/**
 * @typedef {object} Point
 * @property {number} x Across.
 */
```

### JSX

#### Don't

```jsx
/**
 * @typedef {object} Point
 * @property {number}
 */
```

#### Do

```jsx
/**
 * @typedef {object} Point
 * @property {number} x Across.
 */
```

### TypeScript

#### Don't

```ts
/**
 * @typedef {object} Point
 * @property {number}
 */
```

#### Do

```ts
/**
 * @typedef {object} Point
 * @property {number} x Across.
 */
```

### TSX

#### Don't

```tsx
/**
 * @typedef {object} Point
 * @property {number}
 */
```

#### Do

```tsx
/**
 * @typedef {object} Point
 * @property {number} x Across.
 */
```
