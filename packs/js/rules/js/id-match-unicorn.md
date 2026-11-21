---
id: js/id-match-unicorn
title: "Identifiers match the project's pattern (unicorn's)"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new name is the author's"
fix_exception: value
stability: preview
options:
  pattern: { type: "string", default: "", description: "a regular expression every identifier must match" }
  properties: { type: "boolean", default: false, description: "also check property names (object keys, assigned members)" }
  classFields: { type: "boolean", default: false, description: "also check class field names" }
  onlyDeclarations: { type: "boolean", default: false, description: "check only the names of variables and functions at their declaration" }
  ignoreDestructuring: { type: "boolean", default: false, description: "leave names that destructuring takes from an object as they are" }
  checkNamedSpecifiers: { type: "boolean", default: true, description: "check the names in `import { x }` and `export { x } from`; false leaves them to the module that owns them" }
ported_from: [eslint-plugin-unicorn/id-match]
since: 0.5.2
---

`js/id-match` with one more option, as eslint-plugin-unicorn 79.0.0's `id-match` has it: `checkNamedSpecifiers` (default `true`); set to `false`, a name in `import { get_user } from "./users"` or `export { get_user } from "./users"` is not reported, since the module that owns it chose it. Everything else is `js/id-match`'s. Unicorn's rule also reads CSS and HTML; those halves are the `css` and `html` packs' own rules, not this one.

With no `pattern` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

In `codelaw.toml`, under `[rules]`: `"js/id-match-unicorn" = { severity = "error", options = { pattern = "^[a-z][a-zA-Z0-9]*$", checkNamedSpecifiers = true } }`.

**Why:** The same naming convention as `js/id-match`, with a switch for names a project imports from code it does not own.

```codelaw
native: js/id-match-unicorn
```

## How to fix

1. Rename it to match the pattern, in the declaration and every use.

## Examples

### JavaScript

#### Don't

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":true}
import { get_user } from "./users";
```

#### Do

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":true}
import { getUser } from "./users";
```

### JSX

#### Don't

```jsx options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":true}
import { get_user } from "./users";
```

#### Do

```jsx options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":true}
import { getUser } from "./users";
```

### TypeScript

#### Don't

```ts options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":true}
import { get_user } from "./users";
```

#### Do

```ts options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":true}
import { getUser } from "./users";
```

### TSX

#### Don't

```tsx options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":true}
import { get_user } from "./users";
```

#### Do

```tsx options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":true}
import { getUser } from "./users";
```
