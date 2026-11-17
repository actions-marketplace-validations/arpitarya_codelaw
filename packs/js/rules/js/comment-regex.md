---
id: js/comment-regex
title: "No comment matching the project's pattern"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what to do about the comment is the author's"
fix_exception: value
stability: preview
options:
  regularExpression: { type: "string", default: "", description: "a regular expression; a comment whose text matches it is reported" }
  message: { type: "string", default: "", description: "the message shown (default: `The regular expression matches this comment.`)" }
ported_from: [eslint-plugin-sonarjs/comment-regex]
since: 0.5.2
---

A comment whose text matches `regularExpression` is reported. The text is the comment without its `//` or `/* */`, so `^\s*FIXME` matches a line comment that starts with `FIXME`. `message` replaces the default message. eslint-plugin-sonarjs 4.2.2's `comment-regex` (S124), behaviour only (LGPL-3.0, L2); a pattern that is not a valid regular expression is reported once, on the first comment of the file.

With no `regularExpression` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** A team keeps some words out of comments (`FIXME`, a ticket-less `HACK`, a person's name) or requires a form; the project's pattern says which.

```codelaw
native: js/comment-regex
```

## How to fix

1. Resolve what the comment says and delete it, or rewrite it so it no longer matches.

## Examples

### JavaScript

#### Don't

```js options={"regularExpression":"FIXME"}
// FIXME: handle an empty list
run();
```

#### Do

```js options={"regularExpression":"FIXME"}
// An empty list is handled by run()
run();
```

### JSX

#### Don't

```jsx options={"regularExpression":"FIXME"}
// FIXME: handle an empty list
run();
```

#### Do

```jsx options={"regularExpression":"FIXME"}
// An empty list is handled by run()
run();
```

### TypeScript

#### Don't

```ts options={"regularExpression":"FIXME"}
// FIXME: handle an empty list
run();
```

#### Do

```ts options={"regularExpression":"FIXME"}
// An empty list is handled by run()
run();
```

### TSX

#### Don't

```tsx options={"regularExpression":"FIXME"}
// FIXME: handle an empty list
run();
```

#### Do

```tsx options={"regularExpression":"FIXME"}
// An empty list is handled by run()
run();
```
