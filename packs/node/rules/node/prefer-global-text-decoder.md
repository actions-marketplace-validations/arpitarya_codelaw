---
id: node/prefer-global-text-decoder
title: "Use the global TextDecoder"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a bundler for the browser may resolve `util` to a polyfill package rather than Node's built-in, which the global is not"
stability: preview
ported_from: [eslint-plugin-n/prefer-global/text-decoder]
since: 0.5.0
---

Code uses the global `TextDecoder` rather than importing it from `util` (the upstream default, `"always"`). Reported: a `require` or `import` that binds only that name, which the fix removes; other shapes are left alone.

**Why:** The global is the same object as the module's export; the import is one more line that says nothing and invites the two to be confused.

```codelaw
match:
  any:
    - any: [{kind: lexical_declaration}, {kind: variable_declaration}]
      has:
        kind: variable_declarator
        all:
          - has: { field: value, pattern: require($M) }
          - has:
              field: name
              kind: object_pattern
              has: { kind: shorthand_property_identifier_pattern }
              not: { has: { any: [{kind: pair_pattern}, {kind: rest_pattern}, {kind: object_assignment_pattern}, {kind: shorthand_property_identifier_pattern, not: { regex: '^(TextDecoder)$' }}] } }
      not: { has: { kind: variable_declarator, nthChild: { position: 2, ofRule: { kind: variable_declarator } } } }
    - kind: import_statement
      has: { field: source, pattern: $M }
      all:
        - has:
            kind: import_clause
            has: { kind: named_imports, has: { kind: import_specifier } }
            not: { has: { any: [{kind: identifier}, {kind: namespace_import}] } }
        - not: { has: { kind: import_clause, has: { kind: named_imports, has: { kind: import_specifier, any: [{has: {field: alias, regex: "."}}, {not: {regex: '^(TextDecoder)$'}}] } } } }
    - pattern: const TextDecoder = require($M).TextDecoder
    - pattern: let TextDecoder = require($M).TextDecoder
    - pattern: var TextDecoder = require($M).TextDecoder
constraints:
  M: { regex: '^.(node:)?util.$' }
message: "`TextDecoder` is a global; drop this import"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
const { TextDecoder } = require("util");
const text = new TextDecoder().decode(bytes);
```

#### Do

```js
const text = new TextDecoder().decode(bytes);
```

### JSX

#### Don't

```jsx
const { TextDecoder } = require("util");
const text = new TextDecoder().decode(bytes);
```

#### Do

```jsx
const text = new TextDecoder().decode(bytes);
```

### TypeScript

#### Don't

```ts
const { TextDecoder } = require("util");
const text = new TextDecoder().decode(bytes);
```

#### Do

```ts
const text = new TextDecoder().decode(bytes);
```

### TSX

#### Don't

```tsx
const { TextDecoder } = require("util");
const text = new TextDecoder().decode(bytes);
```

#### Do

```tsx
const text = new TextDecoder().decode(bytes);
```
