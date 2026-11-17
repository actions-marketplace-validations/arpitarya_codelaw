---
id: js/no-restricted-syntax
title: "No syntax the project restricts"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what replaces the syntax is the project's convention, which the code does not hold"
fix_exception: value
stability: preview
options:
  patterns: { type: "string[]", default: [], description: "ast-grep patterns for the syntax that may not be used (`for ($K in $O) $B`, `Object.assign($$$ARGS)`)" }
  kinds: { type: "string[]", default: [], description: "tree-sitter node kinds that may not be used (`for_in_statement`, `with_statement`)" }
  messages: { type: "object", default: {}, description: "a pattern or kind to the message shown for it (default: `Using '<it>' is not allowed.`)" }
ported_from: [eslint/no-restricted-syntax]
since: 0.5.2
---

Syntax the project lists is reported. ESLint 10.13.0's rule takes ESTree selectors, which a tree-sitter tree does not have, so an entry here is an ast-grep pattern in `patterns` (the same patterns a project rule's `match:` takes: `$A` is one node, `$$$A` several) or a tree-sitter node kind in `kinds`. `messages` gives the message for an entry; the default is `Using '<entry>' is not allowed.`. A pattern that does not parse is reported once, on the file's first node. Every node of the file is tried against every entry, so a long list costs a walk per file; with none set the rule is not run at all.

For a restriction that needs more than a shape (a call to a particular imported function, a rule with a fix), write a project rule with `match:` instead: a project's own rules (SR-RULE-AUTHORING).

With no `patterns` or `kinds` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** A team bans a construct (`for...in`, `with`, a legacy API) with the rule's message saying what to use.

```codelaw
native: js/no-restricted-syntax
```

## How to fix

1. Rewrite the code with what the project's message names.

## Examples

### JavaScript

#### Don't

```js options={"patterns":["Object.assign($$$ARGS)"],"messages":{"Object.assign($$$ARGS)":"UseObjectSpread"}}
const merged = Object.assign({}, base, extra);
```

#### Do

```js options={"patterns":["Object.assign($$$ARGS)"],"messages":{"Object.assign($$$ARGS)":"UseObjectSpread"}}
const merged = { ...base, ...extra };
```

### JSX

#### Don't

```jsx options={"patterns":["Object.assign($$$ARGS)"],"messages":{"Object.assign($$$ARGS)":"UseObjectSpread"}}
const merged = Object.assign({}, base, extra);
```

#### Do

```jsx options={"patterns":["Object.assign($$$ARGS)"],"messages":{"Object.assign($$$ARGS)":"UseObjectSpread"}}
const merged = { ...base, ...extra };
```

### TypeScript

#### Don't

```ts options={"patterns":["Object.assign($$$ARGS)"],"messages":{"Object.assign($$$ARGS)":"UseObjectSpread"}}
const merged = Object.assign({}, base, extra);
```

#### Do

```ts options={"patterns":["Object.assign($$$ARGS)"],"messages":{"Object.assign($$$ARGS)":"UseObjectSpread"}}
const merged = { ...base, ...extra };
```

### TSX

#### Don't

```tsx options={"patterns":["Object.assign($$$ARGS)"],"messages":{"Object.assign($$$ARGS)":"UseObjectSpread"}}
const merged = Object.assign({}, base, extra);
```

#### Do

```tsx options={"patterns":["Object.assign($$$ARGS)"],"messages":{"Object.assign($$$ARGS)":"UseObjectSpread"}}
const merged = { ...base, ...extra };
```
