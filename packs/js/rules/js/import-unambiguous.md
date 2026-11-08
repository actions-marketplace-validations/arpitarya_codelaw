---
id: js/import-unambiguous
title: "A module has at least one `import` or `export`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the file is a module or a script is the author's decision"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/unambiguous]
since: 0.5.0
---

A `.js`, `.mjs`, `.jsx`, `.ts` or `.tsx` file — which ESLint 10 parses as an ES module — with no `import` or `export` statement (`export {}` counts) could be parsed as a script too, and is reported at its start. `.cjs` and `.cts` files are scripts and are left alone; so is an empty file. A dynamic `import()` does not make a file a module.

**Why:** Tools that decide by content treat such a file as a script: no strict mode, top-level names become globals.

```codelaw
native: js/import-unambiguous
```

## How to fix

1. Add an `export` (an `export {};` is enough), or name the file `.cjs` if it is a script.

## Examples

### JavaScript

#### Don't

```js
const answer = 42;
console.log(answer);
```

#### Do

```js
export const answer = 42;
```


### JSX

#### Don't

```jsx
const answer = 42;
console.log(answer);
```

#### Do

```jsx
export const answer = 42;
```


### TypeScript

#### Don't

```ts
const answer = 42;
console.log(answer);
```

#### Do

```ts
export const answer = 42;
```


### TSX

#### Don't

```tsx
const answer = 42;
console.log(answer);
```

#### Do

```tsx
export const answer = 42;
```
