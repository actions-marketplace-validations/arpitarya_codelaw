---
id: react/no-misplaced-list-elements
title: "`<li>` sits in a list"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the element around the items becomes a `<ul>` or `<ol>`, or the items become something else"
fix_exception: restructure
stability: preview
ported_from: [biome/nursery/noMisplacedListElements, "@html-eslint/require-li-container"]
since: 0.5.1
---

A JSX `<li>` whose parent element is a lowercase HTML element other than `<ul>`, `<ol>` or `<menu>` is reported. An `<li>` whose parent is a component, a fragment or an expression is left alone, since the wrapper is not known. Behaviour of Biome's `noMisplacedListElements`.

**Why:** A list item outside a list has no list semantics: assistive technology does not announce the list or its length.

```codelaw
match:
  kind: jsx_element
  all:
    - has: { field: open_tag, has: { field: name, kind: identifier, regex: '^li$' } }
    - inside:
        kind: jsx_element
        has:
          field: open_tag
          has:
            field: name
            kind: identifier
            all: [{ regex: '^[a-z]' }, { not: { regex: '^(ul|ol|menu)$' } }]
message: "`<li>` belongs inside `<ul>`, `<ol>` or `<menu>`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const l = <div><li>One</li></div>;
```

#### Do

```js
const l = <ul><li>One</li></ul>;
```

### JSX

#### Don't

```jsx
const l = <div><li>One</li></div>;
```

#### Do

```jsx
const l = <ul><li>One</li></ul>;
```

### TSX

#### Don't

```tsx
const l = <div><li>One</li></div>;
```

#### Do

```tsx
const l = <ul><li>One</li></ul>;
```
