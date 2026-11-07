---
id: js/default-export-style
title: "A default-exported function or class declared inline with export default"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/default-export-style]
since: 0.5.0
---

A function or class declaration that the module then exports with `export default name;` is declared with `export default` in front instead: `class Foo {} … export default Foo;` is `export default class Foo {}`. This is upstream's default style (`inline`); a `const` arrow function exported by name is left, since turning it into a declaration would change how it binds `this`. In TypeScript a class name is a type name to the parser, and only functions are matched there.

**Why:** The export is visible where the thing is declared, not at the end of the file.

```codelaw
match:
  kind: export_statement
  pattern: "export default $N;"
  follows:
    stopBy: end
    any:
      - kind: class_declaration
      - kind: function_declaration
      - kind: generator_function_declaration
    has: { field: name, pattern: $N }
    not: { inside: { kind: export_statement } }
    pattern: $DECL
  inside: { kind: program }
constraints:
  N: { any: [{ kind: identifier }] }
message: "Declare `$N` with `export default`"
fix:
  - insert-before: { target: $DECL, text: "export default " }
  - remove
```

## Examples

### JavaScript

#### Don't

```js
function main() {}
export default main;
```

#### Do

```js
export default function main() {}
```


### JSX

#### Don't

```jsx
function main() {}
export default main;
```

#### Do

```jsx
export default function main() {}
```


### TypeScript

#### Don't

```ts
function main() {}
export default main;
```

#### Do

```ts
export default function main() {}
```


### TSX

#### Don't

```tsx
function main() {}
export default main;
```

#### Do

```tsx
export default function main() {}
```
