---
id: node/prefer-global-url
title: "Use the global URL"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a bundler for the browser may resolve `url` to a polyfill package rather than Node's built-in, which the global is not"
stability: preview
ported_from: [eslint-plugin-n/prefer-global/url]
since: 0.5.0
---

Code uses the global `URL` rather than importing it from `url` (the upstream default, `"always"`). Reported: a `require` or `import` that binds only that name, which the fix removes; other shapes are left alone.

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
              not: { has: { any: [{kind: pair_pattern}, {kind: rest_pattern}, {kind: object_assignment_pattern}, {kind: shorthand_property_identifier_pattern, not: { regex: '^(URL)$' }}] } }
      not: { has: { kind: variable_declarator, nthChild: { position: 2, ofRule: { kind: variable_declarator } } } }
    - kind: import_statement
      has: { field: source, pattern: $M }
      all:
        - has:
            kind: import_clause
            has: { kind: named_imports, has: { kind: import_specifier } }
            not: { has: { any: [{kind: identifier}, {kind: namespace_import}] } }
        - not: { has: { kind: import_clause, has: { kind: named_imports, has: { kind: import_specifier, any: [{has: {field: alias, regex: "."}}, {not: {regex: '^(URL)$'}}] } } } }
    - pattern: const URL = require($M).URL
    - pattern: let URL = require($M).URL
    - pattern: var URL = require($M).URL
constraints:
  M: { regex: '^.(node:)?url.$' }
message: "`URL` is a global; drop this import"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
const { URL } = require("url");
const home = new URL("https://example.com");
```

#### Do

```js
const home = new URL("https://example.com");
```

### JSX

#### Don't

```jsx
const { URL } = require("url");
const home = new URL("https://example.com");
```

#### Do

```jsx
const home = new URL("https://example.com");
```

### TypeScript

#### Don't

```ts
const { URL } = require("url");
const home = new URL("https://example.com");
```

#### Do

```ts
const home = new URL("https://example.com");
```

### TSX

#### Don't

```tsx
const { URL } = require("url");
const home = new URL("https://example.com");
```

#### Do

```tsx
const home = new URL("https://example.com");
```
