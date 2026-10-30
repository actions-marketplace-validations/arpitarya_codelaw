---
id: react/no-invalid-html-attribute
title: "rel holds only valid link types"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: which link type was meant is the author's"
stability: preview
ported_from: [eslint-plugin-react/no-invalid-html-attribute]
since: 0.5.0
---

The `rel` of `<a>`, `<area>`, `<link>` and `<form>` holds only link types the HTML standard defines (`noopener`, `stylesheet`, `preload`, …).

**Why:** A browser ignores a `rel` token it doesn't know, so a misspelled `noopener` silently does nothing.

```codelaw
match:
  kind: jsx_attribute
  regex: '^rel\s*=\s*["'']'
  not: { regex: '^rel\s*=\s*["'']\s*((alternate|author|bookmark|canonical|dns-prefetch|external|help|icon|license|manifest|me|modulepreload|next|nofollow|noopener|noreferrer|opener|pingback|preconnect|prefetch|preload|prerender|prev|search|shortlink|stylesheet|tag|apple-touch-icon|apple-touch-startup-image|mask-icon|shortcut)(\s+(alternate|author|bookmark|canonical|dns-prefetch|external|help|icon|license|manifest|me|modulepreload|next|nofollow|noopener|noreferrer|opener|pingback|preconnect|prefetch|preload|prerender|prev|search|shortlink|stylesheet|tag|apple-touch-icon|apple-touch-startup-image|mask-icon|shortcut))*)?\s*["'']$' }
  inside:
    any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
    has: { field: name, regex: '^(a|area|link|form)$' }
message: "This `rel` holds a link type HTML doesn't define"
```

## How to fix

1. Correct or remove each unknown token; see the HTML standard's list of link types.

## Examples

### JavaScript

#### Don't

```js
const a = <a href={url} rel="noopenr">Docs</a>;
```

#### Do

```js
const a = <a href={url} rel="noopener">Docs</a>;
```

### JSX

#### Don't

```jsx
const a = <a href={url} rel="noopenr">Docs</a>;
```

#### Do

```jsx
const a = <a href={url} rel="noopener">Docs</a>;
```

### TSX

#### Don't

```tsx
const a = <a href={url} rel="noopenr">Docs</a>;
```

#### Do

```tsx
const a = <a href={url} rel="noopener">Docs</a>;
```
