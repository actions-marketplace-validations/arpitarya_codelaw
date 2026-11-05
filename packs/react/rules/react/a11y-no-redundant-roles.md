---
id: react/a11y-no-redundant-roles
title: "No role an element already has"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "removing the role is safe, but whether the element is the one meant is for the author to confirm"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-redundant-roles]
since: 0.5.0
---

An element is not given the role it has by default (`<button role="button">`, `<ul role="list">`).

**Why:** The role says nothing new and invites drift when the element changes.

```codelaw
match:
  kind: jsx_attribute
  any:
    - regex: '^role=["'']button["'']$'
      inside: { kind: jsx_opening_element, has: { field: name, regex: '^button$' } }
    - regex: '^role=["'']button["'']$'
      inside: { kind: jsx_self_closing_element, has: { field: name, regex: '^button$' } }
    - regex: '^role=["'']img["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^img$' } }
    - regex: '^role=["'']list["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^(ul|ol)$' } }
    - regex: '^role=["'']listitem["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^li$' } }
    - regex: '^role=["'']heading["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^h[1-6]$' } }
    - regex: '^role=["'']table["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^table$' } }
    - regex: '^role=["'']article["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^article$' } }
    - regex: '^role=["'']complementary["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^aside$' } }
    - regex: '^role=["'']main["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^main$' } }
    - regex: '^role=["'']dialog["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^dialog$' } }
    - regex: '^role=["'']textbox["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^textarea$' } }
    - regex: '^role=["'']separator["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^hr$' } }
    - regex: '^role=["'']progressbar["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^progress$' } }
    - regex: '^role=["'']row["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^tr$' } }
    - regex: '^role=["'']option["'']$'
      inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^option$' } }
message: "`$$MATCH` is the element's own role; remove it"
```

## How to fix

1. Remove the `role` attribute.

## Examples

### JavaScript

#### Don't

```js
const b = <button role="button">Save</button>;
```

#### Do

```js
const b = <button>Save</button>;
```

### JSX

#### Don't

```jsx
const b = <button role="button">Save</button>;
```

#### Do

```jsx
const b = <button>Save</button>;
```

### TSX

#### Don't

```tsx
const b = <button role="button">Save</button>;
```

#### Do

```tsx
const b = <button>Save</button>;
```

## Differences from the source

`<nav role="navigation">` is allowed, as the source's default allows it; roles that depend on attributes (`<a href>` as `link`, `<input>` by `type`) are not checked.
