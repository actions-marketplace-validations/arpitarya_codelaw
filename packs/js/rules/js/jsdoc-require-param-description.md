---
id: js/jsdoc-require-param-description
title: "@param says what the parameter is"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's words"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-param-description]
since: 0.5.0
---

A JSDoc `@param` tag carries a description after the parameter name.

**Why:** A name and a type alone rarely say what a parameter means or which values it accepts.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:param|arg|argument)\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\}[ \t]*)?(?:\[[^\]\n]*\]|[\w$.]+(?:\[\])?(?:\.[\w$]+)*)[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
message: Describe this parameter after its name
```

## How to fix

1. After the parameter name, write what the parameter means and what values it takes.

## Examples

### JavaScript

#### Don't

```js
/**
 * Greets someone.
 * @param {string} name
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
 * @param {string} name
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
 * @param {string} name
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
 * @param {string} name
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
