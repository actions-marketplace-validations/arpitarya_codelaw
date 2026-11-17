---
id: js/jsdoc-text-escaping
title: "JSDoc text escapes what Markdown or HTML would read"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
options:
  escapeHTML: { type: "boolean", default: false, description: "escape `<` and `&` (not before whitespace, and not as part of an entity) in descriptions" }
  escapeMarkdown: { type: "boolean", default: false, description: "escape the first backtick of each paired backtick sequence in descriptions" }
ported_from: [eslint-plugin-jsdoc/text-escaping]
since: 0.5.2
---

With `escapeHTML`, a `<` or an `&` in a block's description that is followed by text and does not start an entity (`&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`) becomes `&lt;` or `&amp;`: `Array<string>` is `Array&lt;string>`. With `escapeMarkdown`, the first backtick of a paired backtick sequence gets a backslash: `` `code` `` is `` \`code` ``. The description is checked first and, if it is clean, each tag's description (never an `@example`'s); a fenced code block is skipped. As upstream, when both options are set only HTML is read. eslint-plugin-jsdoc 65.4.2's `text-escaping`, read and reimplemented; where upstream reports that neither option is set, this rule says nothing.

With no `escapeHTML` or `escapeMarkdown` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

**Why:** JSDoc is rendered as HTML or Markdown by the tools that read it, and a stray `<` or backtick pair changes what a reader sees.

```codelaw
native: js/jsdoc-text-escaping
```

## Examples

### JavaScript

#### Don't

```js options={"escapeHTML":true}
/** Returns Array<string> of names. */
function names() {}
```

#### Do

```js options={"escapeHTML":true}
/** Returns Array&lt;string> of names. */
function names() {}
```

#### Fixed

```js options={"escapeHTML":true}
/** Returns Array&lt;string> of names. */
function names() {}
```

### JSX

#### Don't

```jsx options={"escapeHTML":true}
/** Returns Array<string> of names. */
function names() {}
```

#### Do

```jsx options={"escapeHTML":true}
/** Returns Array&lt;string> of names. */
function names() {}
```

#### Fixed

```jsx options={"escapeHTML":true}
/** Returns Array&lt;string> of names. */
function names() {}
```

### TypeScript

#### Don't

```ts options={"escapeHTML":true}
/** Returns Array<string> of names. */
function names() {}
```

#### Do

```ts options={"escapeHTML":true}
/** Returns Array&lt;string> of names. */
function names() {}
```

#### Fixed

```ts options={"escapeHTML":true}
/** Returns Array&lt;string> of names. */
function names() {}
```

### TSX

#### Don't

```tsx options={"escapeHTML":true}
/** Returns Array<string> of names. */
function names() {}
```

#### Do

```tsx options={"escapeHTML":true}
/** Returns Array&lt;string> of names. */
function names() {}
```

#### Fixed

```tsx options={"escapeHTML":true}
/** Returns Array&lt;string> of names. */
function names() {}
```
