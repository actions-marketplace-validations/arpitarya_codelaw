---
id: react/void-dom-elements-no-children
title: "Void elements have no children"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the children have to go somewhere else in the markup, which is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-react/void-dom-elements-no-children]
since: 0.5.0
---

A void DOM element (`<br>`, `<img>`, `<input>`, …) has no children, no `children` prop and no `dangerouslySetInnerHTML`.

**Why:** React throws when a void element is given children.

```codelaw
match:
  any:
    - kind: jsx_element
      has: { field: open_tag, has: { field: name, regex: '^(area|base|br|col|embed|hr|img|input|keygen|link|menuitem|meta|param|source|track|wbr)$' } }
    - kind: jsx_self_closing_element
      all:
        - has: { field: name, regex: '^(area|base|br|col|embed|hr|img|input|keygen|link|menuitem|meta|param|source|track|wbr)$' }
        - has: { kind: jsx_attribute, regex: '^(children|dangerouslySetInnerHTML)\b' }
message: "A void element can't have children"
```

## How to fix

1. Move the children next to the element, or wrap both in a container.

## Examples

### JavaScript

#### Don't

```js
const l = <br>Line</br>;
```

#### Do

```js
const l = <>Line<br /></>;
```

### JSX

#### Don't

```jsx
const l = <br>Line</br>;
```

#### Do

```jsx
const l = <>Line<br /></>;
```

### TSX

#### Don't

```tsx
const l = <br>Line</br>;
```

#### Do

```tsx
const l = <>Line<br /></>;
```
