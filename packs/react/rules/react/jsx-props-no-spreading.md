---
id: react/jsx-props-no-spreading
title: "No spread props in JSX"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: each prop the spread carried is named, which needs the set the author means to pass"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/jsx-props-no-spreading]
since: 0.5.0
---

JSX passes props by name, never by spreading an object (`<Input {...props} />`).

**Why:** A spread hides which props reach the element, and passes along ones it should not see.

```codelaw
match:
  kind: jsx_expression
  inside: { any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }] }
  has: { kind: spread_element }
message: "Name the props instead of spreading them"
```

## How to fix

1. List the props the element needs by name.

## Examples

### JavaScript

#### Don't

```js
const i = <Input {...props} />;
```

#### Do

```js
const i = <Input value={props.value} onChange={props.onChange} />;
```

### JSX

#### Don't

```jsx
const i = <Input {...props} />;
```

#### Do

```jsx
const i = <Input value={props.value} onChange={props.onChange} />;
```

### TSX

#### Don't

```tsx
const i = <Input {...props} />;
```

#### Do

```tsx
const i = <Input value={props.value} onChange={props.onChange} />;
```
