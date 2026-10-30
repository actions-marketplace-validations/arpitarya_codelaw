---
id: js/jsdoc-require-property-description
title: "@property says what the property is"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's words"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-property-description]
since: 0.5.0
---

A JSDoc `@property` tag carries a description after the property name.

**Why:** A name and a type alone rarely say what a property holds or which values are valid.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:property|prop)\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\}[ \t]*)?(?:\[[^\]\n]*\]|[\w$.]+(?:\[\])?(?:\.[\w$]+)*)[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
message: Describe this property after its name
```

## How to fix

1. After the property name, write what the property holds.

## Examples

### JavaScript

#### Don't

```js
/**
 * @typedef {object} Point
 * @property {number} x
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
 * @property {number} x
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
 * @property {number} x
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
 * @property {number} x
 */
```

#### Do

```tsx
/**
 * @typedef {object} Point
 * @property {number} x Across.
 */
```
