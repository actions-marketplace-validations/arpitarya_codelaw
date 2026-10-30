---
id: react/jsx-no-useless-fragment
title: "No fragment around a single element"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/jsx-no-useless-fragment]
since: 0.5.0
---

A short fragment `<>…</>` that holds exactly one element, and no text or expression, is replaced by that element. A fragment holding one expression is left alone for now.

**Why:** A fragment exists to group several children; around one element it's noise.

```codelaw
match:
  kind: jsx_element
  all:
    - has: { field: open_tag, regex: '^<>$' }
    - not: { has: { kind: jsx_expression } }
    - not: { has: { kind: jsx_text, regex: '\S' } }
    - has:
        any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }]
        pattern: $C
        not:
          any:
            - precedes: { stopBy: end, any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }] }
            - follows: { stopBy: end, any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }] }
message: "This fragment wraps one element; drop it"
fix: "$C"
```

## Examples

### JavaScript

#### Don't

```js
const r = <><Row /></>;
```

#### Do

```js
const r = <Row />;
```

### JSX

#### Don't

```jsx
const r = <><Row /></>;
```

#### Do

```jsx
const r = <Row />;
```

### TSX

#### Don't

```tsx
const r = <><Row /></>;
```

#### Do

```tsx
const r = <Row />;
```
