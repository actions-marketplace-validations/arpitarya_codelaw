---
id: js/jsdoc-check-syntax
title: "No Closure optional syntax in JSDoc"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "codelaw's edits replace whole nodes, and a comment is one node, so the edit inside its text is the author's (`{T=} name` becomes `{T} [name]`)"
stability: preview
ported_from: [eslint-plugin-jsdoc/check-syntax]
since: 0.5.0
---

An optional `@param` or `@property` is written `{Type} [name]`, not with Closure's trailing `=` in the type (`{Type=} name`).

**Why:** `{Type=}` is Google Closure syntax; JSDoc and TypeScript tooling read the bracketed name, so the trailing `=` is either ignored or misread.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@(param|arg|argument|property|prop)[ \t]*\{[^{}\n]*=\}'
message: "`{Type=}` is Closure syntax for an optional value; write `{Type} [name]`"
```

## How to fix

1. Remove the trailing `=` from the type.
2. Wrap the parameter or property name in brackets: `@param {string} [label]`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Greets someone.
 * @param {string=} name who to greet
 */
function greet(name) {}
```

#### Do

```js
/**
 * Greets someone.
 * @param {string} [name] who to greet
 */
function greet(name) {}
```

### JSX

#### Don't

```jsx
/**
 * Greets someone.
 * @param {string=} name who to greet
 */
function greet(name) {}
```

#### Do

```jsx
/**
 * Greets someone.
 * @param {string} [name] who to greet
 */
function greet(name) {}
```

### TypeScript

#### Don't

```ts
/**
 * Greets someone.
 * @param {string=} name who to greet
 */
function greet(name) {}
```

#### Do

```ts
/**
 * Greets someone.
 * @param {string} [name] who to greet
 */
function greet(name) {}
```

### TSX

#### Don't

```tsx
/**
 * Greets someone.
 * @param {string=} name who to greet
 */
function greet(name) {}
```

#### Do

```tsx
/**
 * Greets someone.
 * @param {string} [name] who to greet
 */
function greet(name) {}
```
