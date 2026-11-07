---
id: js/import-no-dynamic-require
title: "No require() of a computed path"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "listing the modules that can be loaded is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-import/no-dynamic-require]
since: 0.5.0
---

`require()` is given a string literal, not a value computed at run time.

**Why:** A computed path hides the dependency from bundlers and readers, and can load a file the author never meant.

```codelaw
match:
  pattern: "$R($P)"
constraints:
  R: { regex: '^require$' }
  P:
    not:
      any:
        - kind: string
        - { kind: template_string, not: { has: { kind: template_substitution } } }
where:
  $R: undeclared
message: "Require a literal path"
```

## How to fix

1. Require each possible module by its literal path (for example from an object of loaders), or use a dynamic `import()` with a known set.

## Examples

### JavaScript

#### Don't

```js
const plugin = require(`./plugins/${name}`);
```

#### Do

```js
const plugins = { csv: require("./plugins/csv"), json: require("./plugins/json") };
const plugin = plugins[name];
```


### JSX

#### Don't

```jsx
const plugin = require(`./plugins/${name}`);
```

#### Do

```jsx
const plugins = { csv: require("./plugins/csv"), json: require("./plugins/json") };
const plugin = plugins[name];
```


### TypeScript

#### Don't

```ts
const plugin = require(`./plugins/${name}`);
```

#### Do

```ts
const plugins = { csv: require("./plugins/csv"), json: require("./plugins/json") };
const plugin = plugins[name];
```


### TSX

#### Don't

```tsx
const plugin = require(`./plugins/${name}`);
```

#### Do

```tsx
const plugins = { csv: require("./plugins/csv"), json: require("./plugins/json") };
const plugin = plugins[name];
```
