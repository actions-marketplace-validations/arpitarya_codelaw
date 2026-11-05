---
id: react/a11y-no-noninteractive-tabindex
title: "Non-interactive elements are not in the tab order"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "removing tabIndex takes the element out of the tab order, while -1 keeps it focusable from code only; the reviewer picks, or gives the element an interactive role instead"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-noninteractive-tabindex]
since: 0.5.0
---

A non-interactive element with no interactive role does not get a `tabIndex` of 0 or more.

**Why:** Tabbing to something that does nothing wastes keyboard users' time and confuses screen-reader users.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^[a-z]'}
  - not:
      has: {field: name, regex: ^(a|area|button|input|select|textarea|option|summary|details|iframe|embed|audio|video)$}
  - has: {kind: jsx_attribute, regex: '^tabIndex=(["'']|\{\s*)[0-9]', pattern: $T}
  - not:
      has: {kind: jsx_attribute, regex: '^role=["''](button|checkbox|combobox|gridcell|link|listbox|menu|menubar|menuitem|menuitemcheckbox|menuitemradio|option|radio|radiogroup|scrollbar|searchbox|slider|spinbutton|switch|tab|tablist|textbox|tree|treegrid|treeitem|tabpanel)["'']|^role=\{'}
message: Remove `tabIndex` from a non-interactive element
fix:
  choices:
    - label: remove tabIndex
      fix:
        - remove: $T
    - label: focus from code only
      fix:
        - replace: { target: $T, with: "tabIndex={-1}" }
```

## How to fix

1. Remove `tabIndex`, or give the element an interactive role and keyboard behaviour.

## Examples

### JavaScript

#### Don't

```js
const a = <article tabIndex={0}>…</article>;
```

#### Fixed (remove tabIndex)

```js
const a = <article >…</article>;
```

#### Fixed (focus from code only)

```js
const a = <article tabIndex={-1}>…</article>;
```

#### Do

```js
const a = <article>…</article>;
```

### JSX

#### Don't

```jsx
const a = <article tabIndex={0}>…</article>;
```

#### Fixed (remove tabIndex)

```jsx
const a = <article >…</article>;
```

#### Fixed (focus from code only)

```jsx
const a = <article tabIndex={-1}>…</article>;
```

#### Do

```jsx
const a = <article>…</article>;
```

### TSX

#### Don't

```tsx
const a = <article tabIndex={0}>…</article>;
```

#### Fixed (remove tabIndex)

```tsx
const a = <article >…</article>;
```

#### Fixed (focus from code only)

```tsx
const a = <article tabIndex={-1}>…</article>;
```

#### Do

```tsx
const a = <article>…</article>;
```
