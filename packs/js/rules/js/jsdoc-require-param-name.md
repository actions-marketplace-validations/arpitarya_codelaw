---
id: js/jsdoc-require-param-name
title: "@param names its parameter"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which parameter the tag is for is the author's to say"
fix_pending: "suggest"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-param-name]
since: 0.5.0
---

A JSDoc `@param` tag names the parameter it documents.

**Why:** A `@param` with only a type cannot be matched to a parameter, so tools ignore it.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:param|arg|argument)\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\})?[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
message: Name the parameter this `@param` documents
```

## How to fix

1. Write the parameter's name after the `@param` type.

## Examples

### JavaScript

#### Don't

```js
/**
 * Greets someone.
 * @param {string}
 */
function greet(name) {}
```

#### Do

```js
/**
 * Greets someone.
 * @param {string} name Who to greet.
 */
function greet(name) {}
```

### JSX

#### Don't

```jsx
/**
 * Greets someone.
 * @param {string}
 */
function greet(name) {}
```

#### Do

```jsx
/**
 * Greets someone.
 * @param {string} name Who to greet.
 */
function greet(name) {}
```

### TypeScript

#### Don't

```ts
/**
 * Greets someone.
 * @param {string}
 */
function greet(name) {}
```

#### Do

```ts
/**
 * Greets someone.
 * @param {string} name Who to greet.
 */
function greet(name) {}
```

### TSX

#### Don't

```tsx
/**
 * Greets someone.
 * @param {string}
 */
function greet(name) {}
```

#### Do

```tsx
/**
 * Greets someone.
 * @param {string} name Who to greet.
 */
function greet(name) {}
```
