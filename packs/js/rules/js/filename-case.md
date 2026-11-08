---
id: js/filename-case
title: "File and folder names in one case (kebab case by default)"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "renaming a file changes every import of it"
fix_exception: restructure
options: { case: { type: "string", default: "kebabCase", description: "kebabCase, camelCase, snakeCase or pascalCase" }, cases: { type: "object", default: {}, description: "several cases allowed at once, as { kebabCase: true, pascalCase: true }; wins over case" }, ignore: { type: "string[]", default: [], description: "regular expressions: a path segment that matches one is not checked" }, checkDirectories: { type: "boolean", default: true, description: "check the folder names on the file's path too" }, directoryRoots: { type: "string[]", default: [], description: "folders (a path or a regular expression) below which folder names are checked" }, multipleFileExtensions: { type: "boolean", default: true, description: "treat everything after the first dot as the extension (`a.test.js`)" } }
stability: preview
ported_from: [eslint-plugin-unicorn/filename-case]
since: 0.5.0
---

A file's name (before its first `.`) and, by default, each folder on its path from the project root are in the chosen case — kebab case unless `case` or `cases` says otherwise — and the extension is lower case. Leading underscores, characters outside letters, digits, `-` and `_`, names starting with `$`, and `index.js`/`index.ts`/… are left as they are. The finding names the renamed file or folder. It is reported at the start of the file.

**Why:** One naming scheme makes files easy to find and avoids imports that work on one file system and fail on a case-sensitive one.

```codelaw
native: js/filename-case
```

## How to fix

1. Rename the file (or folder) to the name the message gives, and update its imports.

## Examples

### JavaScript

#### Don't

```js path=src/MyUtils.js
export const x = 1;
```

#### Do

```js path=src/my-utils.js
export const x = 1;
```


### JSX

#### Don't

```jsx path=src/MyUtils.jsx
export const x = 1;
```

#### Do

```jsx path=src/my-utils.jsx
export const x = 1;
```


### TypeScript

#### Don't

```ts path=src/MyUtils.ts
export const x = 1;
```

#### Do

```ts path=src/my-utils.ts
export const x = 1;
```


### TSX

#### Don't

```tsx path=src/MyUtils.tsx
export const x = 1;
```

#### Do

```tsx path=src/my-utils.tsx
export const x = 1;
```
