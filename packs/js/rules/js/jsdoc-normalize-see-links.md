---
id: js/jsdoc-normalize-see-links
title: "One form for labelled links in @see tags"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { enable_fixer: { type: "boolean", default: true, description: "offer the rewrite" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/normalize-see-links]
since: 0.5.0
---

A labelled link in a `@see` tag is written `{@link target|label}`: a Markdown link `[label](target)` and a prefix link `[label]{@link target}` are rewritten to it (the plugin's default `canonicalForm: "pipe"`). A link that cannot be rewritten safely — one written `{@link target label}`, a Markdown link inside a code span, one to a scoped package (`@scope/pkg`), one with an empty label, or text that looks like a link and is not — is reported as `@see link cannot be safely normalized.`, with no fix. Targets are percent-encoded where `|`, `{` or `}` would end the link. The `canonicalForm: "prefix"` and `wrapBareUrls` options are not built.

**Why:** Documentation tools read `{@link}`, not Markdown, in a `@see`; a mix of forms means some links come out as plain text.

```codelaw
native: js/jsdoc-normalize-see-links
```

## How to fix

1. Write the link as `{@link target|label}` (the fix does).

## Examples

### JavaScript

#### Don't

```js
/**
 * @see [the docs](https://example.com/docs)
 */
function f() {}
```

#### Do

```js
/**
 * @see {@link https://example.com/docs|the docs}
 */
function f() {}
```

#### Fixed

```js
/**
 * @see {@link https://example.com/docs|the docs}
 */
function f() {}
```

### JSX

#### Don't

```jsx
/**
 * @see [the docs](https://example.com/docs)
 */
function f() {}
```

#### Do

```jsx
/**
 * @see {@link https://example.com/docs|the docs}
 */
function f() {}
```

#### Fixed

```jsx
/**
 * @see {@link https://example.com/docs|the docs}
 */
function f() {}
```

### TypeScript

#### Don't

```ts
/**
 * @see [the docs](https://example.com/docs)
 */
function f() {}
```

#### Do

```ts
/**
 * @see {@link https://example.com/docs|the docs}
 */
function f() {}
```

#### Fixed

```ts
/**
 * @see {@link https://example.com/docs|the docs}
 */
function f() {}
```

### TSX

#### Don't

```tsx
/**
 * @see [the docs](https://example.com/docs)
 */
function f() {}
```

#### Do

```tsx
/**
 * @see {@link https://example.com/docs|the docs}
 */
function f() {}
```

#### Fixed

```tsx
/**
 * @see {@link https://example.com/docs|the docs}
 */
function f() {}
```
