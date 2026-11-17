---
id: js/id-denylist
title: "No identifier the project denies"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the name to use instead is the author's"
fix_exception: value
stability: preview
options:
  names: { type: "string[]", default: [], description: "identifier names that may not be used" }
ported_from: [eslint/id-denylist]
since: 0.5.2
---

An identifier named in `names` is reported wherever the file declares, assigns or reads it: `names = ["data", "temp", "e"]` flags `const data`, `function f(temp)` and `catch (e)`. Behaviour of ESLint 10.13.0's `id-denylist`, read and reimplemented. As upstream, these are not checked: a name that is directly a call's callee or argument (`use(data)`), a property read (`item.data`, but `item.data = 1` is), the old name in `import { data as records }` and a destructuring key (`const { data: records } = source`), and a reference to a global the file does not declare.

Set the option to switch it on, for example `[rules] "js/id-denylist" = { severity = "error", options = { names = ["data", "temp", "e"] } }`. With no `names` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** A project denies names that say nothing (`data`, `temp`, `e`) so that each name tells the reader what the value is.

```codelaw
native: js/id-denylist
```

## How to fix

1. Rename it to a name that says what it holds, in the declaration and every use.

## Examples

### JavaScript

#### Don't

```js options={"names":["data","temp","e"]}
const data = load();
```

#### Do

```js options={"names":["data","temp","e"]}
const records = load();
```

### JSX

#### Don't

```jsx options={"names":["data","temp","e"]}
const data = load();
```

#### Do

```jsx options={"names":["data","temp","e"]}
const records = load();
```

### TypeScript

#### Don't

```ts options={"names":["data","temp","e"]}
const data = load();
```

#### Do

```ts options={"names":["data","temp","e"]}
const records = load();
```

### TSX

#### Don't

```tsx options={"names":["data","temp","e"]}
const data = load();
```

#### Do

```tsx options={"names":["data","temp","e"]}
const records = load();
```
