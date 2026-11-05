---
id: js/jsdoc-ts-ban-ts-comment
title: "No @ts-ignore or @ts-nocheck in JavaScript"
languages: [javascript, jsx]
fix: manual
fix_reason: "the reason the error is expected has to come from the author"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/ts-ban-ts-comment]
since: 0.5.0
---

In a JavaScript file checked by TypeScript, type-checking is not switched off with `@ts-ignore` or `@ts-nocheck` (in a line, block or JSDoc comment), and a `@ts-expect-error` says why. TypeScript files are `ts/ban-ts-comment`'s.

**Why:** `@ts-ignore` hides an error for good, even after the code it excused changes; `@ts-expect-error` fails as soon as the error is gone.

```codelaw
match:
  kind: comment
  any:
  - regex: ^(?://|/\*\*?)[ \t]*@ts-(?:ignore|nocheck)\b
  - regex: ^(?://|/\*\*?)[ \t]*@ts-expect-error[ \t]*(?:\*/)?$
message: 'Don''t silence the type checker with `@ts-ignore` or `@ts-nocheck`; use `@ts-expect-error: <why>`'
```

## How to fix

1. Fix the type error if you can.
2. Otherwise replace the comment with `// @ts-expect-error: <why>`.

## Examples

### JavaScript

#### Don't

```js
// @ts-ignore
const n = Math.max("1");
```

#### Do

```js
// @ts-expect-error: the fixture checks the checker's message
const n = Math.max("1");
```

### JSX

#### Don't

```jsx
// @ts-ignore
const n = Math.max("1");
```

#### Do

```jsx
// @ts-expect-error: the fixture checks the checker's message
const n = Math.max("1");
```
