---
id: js/file-header
title: "A file starts with the project's header"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a file that has a different header needs it replaced, not added to; the reviewer decides"
stability: preview
options:
  headerFormat: { type: "string", default: "", description: "the text every file starts with, or a regular expression when `isRegularExpression` is true" }
  isRegularExpression: { type: "boolean", default: false, description: "read `headerFormat` as a regular expression the start of the file must match" }
ported_from: [eslint-plugin-sonarjs/file-header, tslint/file-header]
since: 0.5.2
---

A file must start with `headerFormat`, after a `#!` line if it has one: the text as written (`// Copyright Acme`, a `/* … */` block), or, with `isRegularExpression`, a regular expression the start of the file must match. A file that does not is reported on its first line, with a suggestion to put the header first (a regular expression has no text to insert, so its finding has none). An empty file is not reported. eslint-plugin-sonarjs 4.2.2's `file-header` (S1451), behaviour only (LGPL-3.0, L2), and TSLint's `file-header`, whose successor in any tool nobody found. A header regular expression that does not compile is reported once, on the first node.

With no `headerFormat` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** A licence or copyright notice has to be at the top of every file; the rule is the check that none was forgotten.

```codelaw
native: js/file-header
```

## Examples

### JavaScript

#### Don't

```js options={"headerFormat":"//\u0020Copyright\u0020Acme\n"}
run();
```

#### Do

```js options={"headerFormat":"//\u0020Copyright\u0020Acme\n"}
// Copyright Acme
run();
```

### JSX

#### Don't

```jsx options={"headerFormat":"//\u0020Copyright\u0020Acme\n"}
run();
```

#### Do

```jsx options={"headerFormat":"//\u0020Copyright\u0020Acme\n"}
// Copyright Acme
run();
```

### TypeScript

#### Don't

```ts options={"headerFormat":"//\u0020Copyright\u0020Acme\n"}
run();
```

#### Do

```ts options={"headerFormat":"//\u0020Copyright\u0020Acme\n"}
// Copyright Acme
run();
```

### TSX

#### Don't

```tsx options={"headerFormat":"//\u0020Copyright\u0020Acme\n"}
run();
```

#### Do

```tsx options={"headerFormat":"//\u0020Copyright\u0020Acme\n"}
// Copyright Acme
run();
```
