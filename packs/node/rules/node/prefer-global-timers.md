---
id: node/prefer-global-timers
title: "Use the global timer functions"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a bundler for the browser may resolve `timers` to a polyfill package rather than Node's built-in, which the global is not"
stability: preview
ported_from: [eslint-plugin-n/prefer-global/timers]
since: 0.5.0
---

Code uses the global `timer functions` rather than importing it from `timers` (the upstream default, `"always"`). Reported: a `require` or `import` that binds only those names, which the fix removes; other shapes are left alone.

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
              not: { has: { any: [{kind: pair_pattern}, {kind: rest_pattern}, {kind: object_assignment_pattern}, {kind: shorthand_property_identifier_pattern, not: { regex: '^(setTimeout|setInterval|setImmediate|clearTimeout|clearInterval|clearImmediate)$' }}] } }
      not: { has: { kind: variable_declarator, nthChild: { position: 2, ofRule: { kind: variable_declarator } } } }
    - kind: import_statement
      has: { field: source, pattern: $M }
      all:
        - has:
            kind: import_clause
            has: { kind: named_imports, has: { kind: import_specifier } }
            not: { has: { any: [{kind: identifier}, {kind: namespace_import}] } }
        - not: { has: { kind: import_clause, has: { kind: named_imports, has: { kind: import_specifier, any: [{has: {field: alias, regex: "."}}, {not: {regex: '^(setTimeout|setInterval|setImmediate|clearTimeout|clearInterval|clearImmediate)$'}}] } } } }
    - pattern: const setTimeout = require($M).setTimeout
    - pattern: let setTimeout = require($M).setTimeout
    - pattern: var setTimeout = require($M).setTimeout
    - pattern: const setInterval = require($M).setInterval
    - pattern: let setInterval = require($M).setInterval
    - pattern: var setInterval = require($M).setInterval
    - pattern: const setImmediate = require($M).setImmediate
    - pattern: let setImmediate = require($M).setImmediate
    - pattern: var setImmediate = require($M).setImmediate
    - pattern: const clearTimeout = require($M).clearTimeout
    - pattern: let clearTimeout = require($M).clearTimeout
    - pattern: var clearTimeout = require($M).clearTimeout
    - pattern: const clearInterval = require($M).clearInterval
    - pattern: let clearInterval = require($M).clearInterval
    - pattern: var clearInterval = require($M).clearInterval
    - pattern: const clearImmediate = require($M).clearImmediate
    - pattern: let clearImmediate = require($M).clearImmediate
    - pattern: var clearImmediate = require($M).clearImmediate
constraints:
  M: { regex: '^.(node:)?timers.$' }
message: "`timer functions` is a global; drop this import"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
const { setTimeout, clearTimeout } = require("timers");
const t = setTimeout(tick, 100);
clearTimeout(t);
```

#### Do

```js
const t = setTimeout(tick, 100);
clearTimeout(t);
```

### JSX

#### Don't

```jsx
const { setTimeout, clearTimeout } = require("timers");
const t = setTimeout(tick, 100);
clearTimeout(t);
```

#### Do

```jsx
const t = setTimeout(tick, 100);
clearTimeout(t);
```

### TypeScript

#### Don't

```ts
const { setTimeout, clearTimeout } = require("timers");
const t = setTimeout(tick, 100);
clearTimeout(t);
```

#### Do

```ts
const t = setTimeout(tick, 100);
clearTimeout(t);
```

### TSX

#### Don't

```tsx
const { setTimeout, clearTimeout } = require("timers");
const t = setTimeout(tick, 100);
clearTimeout(t);
```

#### Do

```tsx
const t = setTimeout(tick, 100);
clearTimeout(t);
```
