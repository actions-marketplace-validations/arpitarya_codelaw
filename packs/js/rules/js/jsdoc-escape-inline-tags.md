---
id: js/jsdoc-escape-inline-tags
title: "A bare @word in a description is escaped"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "codelaw's edits replace whole nodes, and a comment is one node, so the edit inside its text is the author's (the word is wrapped in backticks)"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint-plugin-jsdoc/escape-inline-tags]
since: 0.5.0
---

In a JSDoc description line, an `@` word after a space (`see @types/node`) is wrapped in backticks or escaped as `\@`. `{@link …}` and other braced inline tags, e-mail addresses and text already inside backticks are left alone; lines that start with a tag are not checked.

**Why:** TypeScript reads an `@` after whitespace as the start of a new tag, so the rest of the sentence is cut from the description and filed under a tag that does not exist.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '(^/\*\*|\n[ \t]*\*?)[ \t]*[^@\s*`][^\n`]*[ \t]@[A-Za-z]'
message: "TypeScript reads `@word` after a space as a new tag; wrap it in backticks or write `\\@`"
```

## How to fix

1. Wrap the `@` word in backticks (`` `@types/node` ``), or escape it as `\@`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Needs the types from @types/node to build.
 */
function build() {}
```

#### Do

```js
/**
 * Needs the types from `@types/node` to build.
 */
function build() {}
```

### JSX

#### Don't

```jsx
/**
 * Needs the types from @types/node to build.
 */
function build() {}
```

#### Do

```jsx
/**
 * Needs the types from `@types/node` to build.
 */
function build() {}
```

### TypeScript

#### Don't

```ts
/**
 * Needs the types from @types/node to build.
 */
function build() {}
```

#### Do

```ts
/**
 * Needs the types from `@types/node` to build.
 */
function build() {}
```

### TSX

#### Don't

```tsx
/**
 * Needs the types from @types/node to build.
 */
function build() {}
```

#### Do

```tsx
/**
 * Needs the types from `@types/node` to build.
 */
function build() {}
```
