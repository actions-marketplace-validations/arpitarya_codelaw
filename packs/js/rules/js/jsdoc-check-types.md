---
id: js/jsdoc-check-types
title: "Primitive type names in JSDoc"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a type written `String` may mean the wrapper object on purpose; the author confirms the primitive is meant"
stability: preview
ported_from: [eslint-plugin-jsdoc/check-types]
since: 0.5.0
---

JSDoc types use the primitive names `string`, `number`, `boolean`, `symbol`, `bigint` and `object`, and the generic form `Array<T>`, not the wrapper objects (`String`, `Object`) or the dotted `Array.<T>`. `Object<K, V>` is left alone.

**Why:** `String` is the wrapper object, not the primitive a function takes; TypeScript reads `{String}` as that object type, so checked JavaScript types come out wrong.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '@[A-Za-z]+[ \t]*\{[^}\n]*(\b(String|Number|Boolean|Symbol|BigInt)\b|\bObject[^<.\w]|\.<)'
message: "Use the primitive type names in JSDoc (`string`, `number`, `boolean`, `object`) and `Array<T>` rather than `Array.<T>`"
fix:
  - edit-text: { pattern: '(@[A-Za-z]+[ \t]*\{[^}\n]*)\bString\b', replace: '${1}string' }
  - edit-text: { pattern: '(@[A-Za-z]+[ \t]*\{[^}\n]*)\bNumber\b', replace: '${1}number' }
  - edit-text: { pattern: '(@[A-Za-z]+[ \t]*\{[^}\n]*)\bBoolean\b', replace: '${1}boolean' }
  - edit-text: { pattern: '(@[A-Za-z]+[ \t]*\{[^}\n]*)\bSymbol\b', replace: '${1}symbol' }
  - edit-text: { pattern: '(@[A-Za-z]+[ \t]*\{[^}\n]*)\bBigInt\b', replace: '${1}bigint' }
  - edit-text: { pattern: '(@[A-Za-z]+[ \t]*\{[^}\n]*)\bObject([^<.\w])', replace: '${1}object${2}' }
  - edit-text: { pattern: '(@[A-Za-z]+[ \t]*\{[^}\n]*)\.<', replace: '${1}<' }
```

## How to fix

1. Lower-case the wrapper type: `String` → `string`, `Number` → `number`, `Boolean` → `boolean`, `Symbol` → `symbol`, `BigInt` → `bigint`, `Object` → `object`.
2. Drop the dot of a dotted generic: `Array.<string>` → `Array<string>`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Shouts.
 * @param {String} text what to shout
 */
function shout(text) {}
```

#### Do

```js
/**
 * Shouts.
 * @param {string} text what to shout
 */
function shout(text) {}
```

### JSX

#### Don't

```jsx
/**
 * Shouts.
 * @param {String} text what to shout
 */
function shout(text) {}
```

#### Do

```jsx
/**
 * Shouts.
 * @param {string} text what to shout
 */
function shout(text) {}
```

### TypeScript

#### Don't

```ts
/**
 * Shouts.
 * @param {String} text what to shout
 */
function shout(text) {}
```

#### Do

```ts
/**
 * Shouts.
 * @param {string} text what to shout
 */
function shout(text) {}
```

### TSX

#### Don't

```tsx
/**
 * Shouts.
 * @param {String} text what to shout
 */
function shout(text) {}
```

#### Do

```tsx
/**
 * Shouts.
 * @param {string} text what to shout
 */
function shout(text) {}
```
