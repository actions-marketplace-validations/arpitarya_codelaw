---
id: js/jsdoc-no-defaults
title: "Defaults in the signature, not in JSDoc"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the default moves into the function signature, and codelaw's edits replace whole nodes, and a comment is one node, so the edit inside its text is the author's"
stability: preview
ported_from: [eslint-plugin-jsdoc/no-defaults]
since: 0.5.0
---

A `@param` is not given a default in JSDoc (`[name=value]`); the default is written in the function signature and the tag says `[name]`.

**Why:** A default written only in the comment can drift from the one the code applies, and TypeScript ignores it.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@(param|arg|argument)[ \t]*(\{[^}\n]*\}[ \t]*)?\[[^\]=\s]+='
message: "Give the parameter its default in the signature and write `[name]` here; a default in JSDoc can drift from the code"
```

## How to fix

1. Put the default in the signature: `function greet(name = "you")`.
2. Remove `=value` from the tag, leaving `@param {string} [name]`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Greets someone.
 * @param {string} [name="you"] who to greet
 */
function greet(name = "you") {}
```

#### Do

```js
/**
 * Greets someone.
 * @param {string} [name] who to greet
 */
function greet(name = "you") {}
```

### JSX

#### Don't

```jsx
/**
 * Greets someone.
 * @param {string} [name="you"] who to greet
 */
function greet(name = "you") {}
```

#### Do

```jsx
/**
 * Greets someone.
 * @param {string} [name] who to greet
 */
function greet(name = "you") {}
```

### TypeScript

#### Don't

```ts
/**
 * Greets someone.
 * @param {string} [name="you"] who to greet
 */
function greet(name = "you") {}
```

#### Do

```ts
/**
 * Greets someone.
 * @param {string} [name] who to greet
 */
function greet(name = "you") {}
```

### TSX

#### Don't

```tsx
/**
 * Greets someone.
 * @param {string} [name="you"] who to greet
 */
function greet(name = "you") {}
```

#### Do

```tsx
/**
 * Greets someone.
 * @param {string} [name] who to greet
 */
function greet(name = "you") {}
```
