---
id: js/id-match
title: "Identifiers match the project's pattern"
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
ported_from: [eslint/id-match]
since: 0.5.2
---

An identifier that does not match `pattern` is reported: `pattern = "^[a-z][a-zA-Z0-9]*$"` asks for camelCase. Behaviour of ESLint 10.13.0's `id-match`, read and reimplemented, including its limits: member names and object keys are checked only with `properties`, class fields only with `classFields`, a private name (`#x`) wherever it stands, `onlyDeclarations` narrows the rule to variable and function names, and `ignoreDestructuring` leaves `const { snake_case } = source` alone. A name that is directly a call's callee or argument, and a reference to a global the file does not declare, are never checked. A pattern that is not a valid regular expression is reported once, on the first identifier of the file.

With no `pattern` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

In `codelaw.toml`, under `[rules]`: `"js/id-match" = { severity = "error", options = { pattern = "^[a-z][a-zA-Z0-9]*$" } }`.

**Why:** A naming convention is a pattern; the rule applies the one the project wrote down.

```codelaw
native: js/id-match
```

## How to fix

1. Rename it to match the pattern, in the declaration and every use.

## Examples

### JavaScript

#### Don't

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const user_name = load();
```

#### Do

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const userName = load();
```

### JSX

#### Don't

```jsx options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const user_name = load();
```

#### Do

```jsx options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const userName = load();
```

### TypeScript

#### Don't

```ts options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const user_name = load();
```

#### Do

```ts options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const userName = load();
```

### TSX

#### Don't

```tsx options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const user_name = load();
```

#### Do

```tsx options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
const userName = load();
```
