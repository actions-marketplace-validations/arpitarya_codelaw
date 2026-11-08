---
id: node/no-unpublished-bin
title: "A package's `bin` file is one npm publishes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to publish the file or point `bin` elsewhere is the package author's decision"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-n/no-unpublished-bin]
since: 0.5.0
---

A file that the nearest `package.json` names in `bin` (directly, without `.js`, or as a folder's `index.js`) but that its `files` list leaves out of the published package is reported at its start: the installed command would point at nothing. `.npmignore` is not read; a package without `files` publishes everything, so it is left alone.

**Why:** npm creates the command's link to a file it did not ship, and the command fails after install.

```codelaw
native: node/no-unpublished-bin
```

## How to fix

1. Add the file (or its folder) to `files` in package.json, or point `bin` at a published file.

## Examples

### JavaScript

#### Don't

```js files
// package.json
{ "name": "tool", "bin": "cli.js", "files": ["lib"] }
// cli.js
run();
```

#### Do

```js files
// package.json
{ "name": "tool", "bin": "cli.js", "files": ["lib", "cli.js"] }
// cli.js
run();
```


### JSX

#### Don't

```jsx files
// package.json
{ "name": "tool", "bin": "cli.jsx", "files": ["lib"] }
// cli.jsx
run();
```

#### Do

```jsx files
// package.json
{ "name": "tool", "bin": "cli.jsx", "files": ["lib", "cli.jsx"] }
// cli.jsx
run();
```


### TypeScript

#### Don't

```ts files
// package.json
{ "name": "tool", "bin": "cli.ts", "files": ["lib"] }
// cli.ts
run();
```

#### Do

```ts files
// package.json
{ "name": "tool", "bin": "cli.ts", "files": ["lib", "cli.ts"] }
// cli.ts
run();
```


### TSX

#### Don't

```tsx files
// package.json
{ "name": "tool", "bin": "cli.tsx", "files": ["lib"] }
// cli.tsx
run();
```

#### Do

```tsx files
// package.json
{ "name": "tool", "bin": "cli.tsx", "files": ["lib", "cli.tsx"] }
// cli.tsx
run();
```
