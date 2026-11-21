---
id: js/comment-type
title: "Comments are written in the allowed types"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a comment says the same in any type, so rewriting it changes no code; a directive, a pragma and a comment between two pieces of code on one line are left alone"
stability: preview
options: { types: { type: "string[]", default: ["singleline", "doc"], description: "the comment types the project writes: `singleline` (`//`), `multiline` (`/* */`) and `doc` (`/** */`, also spelt `jsdoc`)" } }
ported_from: [tslint/comment-type]
since: 0.5.0
---

A comment is written in one of the `types` the project allows — by default `singleline` and `doc`, so a plain `/* … */` block is reported — and any other comment is rewritten as the first allowed type that can hold it: a block as `//` lines (a comment with code after it on its line puts the code on the next line), a run of `//` lines as one block, a doc comment as a plain block. Left alone: directives and pragmas (`/// <reference>`, `// @ts-…`, `/* eslint-… */`, `/*! licence */`, `/*#__PURE__*/`, `//# sourceMappingURL`), a comment in JSX braces, a block with code on both sides of it on its line (`f(/* x */ 1)`), and a `//` comment whose text holds `*/`. TSLint's `jsdoc` type is read as `doc`.

**Why:** One comment style per codebase is read faster, and a block comment that holds no code is a line comment in a longer coat.

```codelaw
native: js/comment-type
```

## Examples

### JavaScript

#### Don't

```js
/* total price with tax */
const total = price * 1.2;
```

#### Do

```js
// total price with tax
const total = price * 1.2;
```

#### Fixed

```js
// total price with tax
const total = price * 1.2;
```


### JSX

#### Don't

```jsx
/* total price with tax */
const total = price * 1.2;
```

#### Do

```jsx
// total price with tax
const total = price * 1.2;
```

#### Fixed

```jsx
// total price with tax
const total = price * 1.2;
```


### TypeScript

#### Don't

```ts
/* total price with tax */
const total = price * 1.2;
```

#### Do

```ts
// total price with tax
const total = price * 1.2;
```

#### Fixed

```ts
// total price with tax
const total = price * 1.2;
```


### TSX

#### Don't

```tsx
/* total price with tax */
const total = price * 1.2;
```

#### Do

```tsx
// total price with tax
const total = price * 1.2;
```

#### Fixed

```tsx
// total price with tax
const total = price * 1.2;
```
