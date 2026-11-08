---
id: js/prefer-identifier-import-export-specifiers
title: "An import or export name written as an identifier, not a string"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-identifier-import-export-specifiers]
since: 0.5.0
---

A string-literal name in an import or export specifier, or a quoted import-attribute key, that is a valid identifier name is written without quotes: `import { 'foo' as foo }` is `import { foo as foo }`, `export { x as 'y' }` is `export { x as y }`, `with { 'type': 'json' }` is `with { type: 'json' }`. A name that is not an identifier (`'a-b'`) needs the quotes and stays.

**Why:** The string form exists for names an identifier can't spell; using it elsewhere makes a reader look for a reason.

```codelaw
match:
  kind: string
  regex: '^["''][A-Za-z_$][A-Za-z0-9_$]*["'']$'
  any:
    - inside: { kind: import_specifier }
    - inside: { kind: export_specifier }
    - all:
        - inside: { kind: pair, field: key, inside: { kind: object, inside: { kind: import_attribute } } }
  has: { kind: string_fragment, pattern: $NAME }
message: "Write `$NAME` without quotes"
fix: "$NAME"
```

## Examples

### JavaScript

#### Don't

```js
export { parse as 'parseValue' } from "./parse.js";
```

#### Do

```js
export { parse as parseValue } from "./parse.js";
```


### JSX

#### Don't

```jsx
export { parse as 'parseValue' } from "./parse.js";
```

#### Do

```jsx
export { parse as parseValue } from "./parse.js";
```


### TypeScript

#### Don't

```ts
export { parse as 'parseValue' } from "./parse.js";
```

#### Do

```ts
export { parse as parseValue } from "./parse.js";
```


### TSX

#### Don't

```tsx
export { parse as 'parseValue' } from "./parse.js";
```

#### Do

```tsx
export { parse as parseValue } from "./parse.js";
```
