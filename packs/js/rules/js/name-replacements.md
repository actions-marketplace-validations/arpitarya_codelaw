---
id: js/name-replacements
title: "Descriptive names rather than abbreviations"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a variable with exactly one replacement is renamed everywhere it is used, with the name made free first; with several replacements each is a suggestion; an exported name, a name used in JSX, a parameter a JSDoc `@param` documents, a TypeScript parameter property, a filename and a property are reported without an edit"
stability: preview
options:
  checkProperties: { type: "boolean", default: false, description: "also check property names (object keys, assigned members, class and interface members, exported names)" }
  checkVariables: { type: "boolean", default: true, description: "check variable, parameter, function, class and type names" }
  checkDefaultAndNamespaceImports: { type: "string", default: "internal", description: "default, namespace and require() bindings: internal (only modules of the project), all or none" }
  checkShorthandImports: { type: "string", default: "internal", description: "`import { x }` with no alias: internal, all or none" }
  checkShorthandProperties: { type: "boolean", default: false, description: "check `{ x }` and `const { x } = …` names" }
  checkFilenames: { type: "boolean", default: true, description: "check the file's own name" }
  extendDefaultReplacements: { type: "boolean", default: true, description: "add replacements to the built-in ones instead of replacing them" }
  replacements: { type: "object", default: {}, description: "discouraged word to { replacement: true } or false to stop reporting it; a lowercase word also matches inside longer names" }
  extendDefaultAllowList: { type: "boolean", default: true, description: "add allowList to the built-in names (defaultProps, getServerSideProps, iOS …) instead of replacing them" }
  allowList: { type: "object", default: {}, description: "whole names to allow, as { name: true }" }
  ignore: { type: "string[]", default: [], description: "regular expressions: a name that matches one is not checked (i18n, l10n, a11y, e2e, jQuery are built in)" }
ported_from: [eslint-plugin-unicorn/name-replacements]
since: 0.5.0
---

A name with a discouraged word in it is reported at its declaration: `err` should be `error`, `cb` `callback`, `retVal` `returnValue`, `Btn` `Button`. The words are upstream's default table (about a hundred: `acc`, `arg`, `btn`, `cfg`, `ctx`, `db`, `dir`, `e`, `el`, `env`, `err`, `fn`, `i`, `j`, `len`, `msg`, `obj`, `opts`, `prop`, `ref`, `req`, `res`, `src`, `str`, `tmp`, `val` …) and `replacements` extends or turns them off; a lowercase word matches the whole name and each word of it in camelCase or snake_case (`createCmd`), a camelCase one only the whole name; an all-capitals name and the allow list are never reported. Every binding is checked — variables, parameters, functions, classes, imports (default and namespace imports and `require()` of the project's own modules, and shorthand `import { x }` likewise), TypeScript types — unless it is a shorthand `{ x }` value (`checkShorthandProperties`). The file's own name is checked too (`utils.js` should be `utilities.js`), and with `checkProperties` the names written as properties. One replacement renames the binding and every use of it, with `{ e }` becoming `{ e: error }` and `export { e }` becoming `export { error as e }`; a replacement that is taken in the scopes of the uses gets underscores; several replacements are choices. Behaviour of eslint-plugin-unicorn v79.0.0 (read, not copied); its CSS, HTML and data checks are the `css`, `html` and `data` packs' own files, not this rule's. As upstream's table says, `i` and `j` are discouraged too.

**Why:** Code is written once and read many times: `cb`, `err` and `tmp` make every reader decode them, and a clear name costs nothing.

```codelaw
native: js/name-replacements
```

## Examples

### JavaScript

#### Don't

```js
const err = new Error('failed');
```

#### Do

```js
const error = new Error('failed');
```

#### Fixed

```js
const error = new Error('failed');
```


### JSX

#### Don't

```jsx
const err = new Error('failed');
```

#### Do

```jsx
const error = new Error('failed');
```

#### Fixed

```jsx
const error = new Error('failed');
```


### TypeScript

#### Don't

```ts
const err = new Error('failed');
```

#### Do

```ts
const error = new Error('failed');
```

#### Fixed

```ts
const error = new Error('failed');
```


### TSX

#### Don't

```tsx
const err = new Error('failed');
```

#### Do

```tsx
const error = new Error('failed');
```

#### Fixed

```tsx
const error = new Error('failed');
```
