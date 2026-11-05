---
id: node/no-process-env
title: "No process.env outside the config module"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "reading the environment in one place means adding a config module and changing every reader to use it"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-n/no-process-env]
since: 0.5.0
---

Code reads its settings from a config module, not from `process.env` directly. The rule is meant for scopes that exclude that one module.

**Why:** Environment reads scattered through the code are untyped, unvalidated and hard to find; one module that reads them all is the single list of what the program needs.

```codelaw
match:
  pattern: $P.env
constraints:
  P: { regex: ^process$ }
where:
  $P: undeclared
message: "Read this setting from the config module, not `process.env`"
```

## How to fix

1. Read the variable once in the config module, validate it there, and export it.
2. Import the value from that module here.

## Examples

### JavaScript

#### Don't

```js
const port = process.env.PORT;
```

#### Do

```js
const { port } = require("./config");
```

### JSX

#### Don't

```jsx
const port = process.env.PORT;
```

#### Do

```jsx
const { port } = require("./config");
```

### TypeScript

#### Don't

```ts
const port = process.env.PORT;
```

#### Do

```ts
const { port } = require("./config");
```

### TSX

#### Don't

```tsx
const port = process.env.PORT;
```

#### Do

```tsx
const { port } = require("./config");
```
