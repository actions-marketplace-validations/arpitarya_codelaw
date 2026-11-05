---
id: react/jsx-handler-names
title: "Handlers are named handle…, handler props on…"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the new method name is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-react/jsx-handler-names]
since: 0.5.0
---

A prop named `on…` is given a method named `handle…` (`onClick={this.handleClick}`), and a `handle…` method is passed only to a prop named `on…`. Handlers passed through from props (`this.props.onClick`) are fine.

**Why:** The two prefixes say at a glance which side of an event a name is on: the prop that receives it, the method that handles it.

```codelaw
match:
  kind: jsx_attribute
  any:
    - all:
        - regex: '^on[A-Z]'
        - has: { kind: jsx_expression, has: { kind: member_expression, regex: '^this\.\w+$', not: { regex: '^this\.handle[A-Z]' } } }
    - all:
        - not: { regex: '^(on[A-Z]|ref\b)' }
        - has: { kind: jsx_expression, has: { kind: member_expression, regex: '^this\.handle[A-Z]\w*$' } }
message: "Name handler methods `handle…` and the props they go to `on…`"
```

## How to fix

1. Rename the method to `handle<Event>`, or the prop to `on<Event>`, so the two prefixes match.

## Examples

### JavaScript

#### Don't

```js
const b = <button type="button" onClick={this.save}>Save</button>;
```

#### Do

```js
const b = <button type="button" onClick={this.handleSave}>Save</button>;
```

### JSX

#### Don't

```jsx
const b = <button type="button" onClick={this.save}>Save</button>;
```

#### Do

```jsx
const b = <button type="button" onClick={this.handleSave}>Save</button>;
```

### TSX

#### Don't

```tsx
const b = <button type="button" onClick={this.save}>Save</button>;
```

#### Do

```tsx
const b = <button type="button" onClick={this.handleSave}>Save</button>;
```
