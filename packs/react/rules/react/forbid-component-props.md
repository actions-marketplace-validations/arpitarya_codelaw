---
id: react/forbid-component-props
title: "No className or style on a component"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the styling moves into the component, which then takes a narrower prop"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/forbid-component-props]
since: 0.5.0
---

A component (an element whose name is capitalised or dotted) is not passed `className` or `style`; those props belong on DOM elements.

**Why:** A component that accepts arbitrary classes and styles has no stable look: every caller can restyle its insides.

```codelaw
match:
  kind: jsx_attribute
  regex: '^(className|style)\b'
  inside:
    any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
    has: { field: name, any: [{ regex: '^[A-Z]' }, { kind: member_expression }] }
message: "Don't pass `className` or `style` to a component; give it a prop that says what varies"
```

## How to fix

1. Move the styling into the component.
2. Give it a prop that names what varies (`variant="primary"`) and pass that instead.

## Examples

### JavaScript

#### Don't

```js
const b = <Button className="big">Go</Button>;
```

#### Do

```js
const b = <Button size="large">Go</Button>;
```

### JSX

#### Don't

```jsx
const b = <Button className="big">Go</Button>;
```

#### Do

```jsx
const b = <Button size="large">Go</Button>;
```

### TSX

#### Don't

```tsx
const b = <Button className="big">Go</Button>;
```

#### Do

```tsx
const b = <Button size="large">Go</Button>;
```
