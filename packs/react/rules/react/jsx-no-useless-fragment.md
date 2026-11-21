---
id: react/jsx-no-useless-fragment
title: "No fragment around a single element"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "a fragment around one element is replaced by it, so nothing changes; a fragment around one expression inside another element is replaced by the braced expression, which renders the same; one anywhere else becomes the bare expression, a different kind of value, so that is a suggestion"
stability: preview
ported_from: [eslint-plugin-react/jsx-no-useless-fragment]
since: 0.5.0
---

A short fragment `<>…</>` that holds exactly one element, and no text or expression, is replaced by that element. A fragment that holds exactly one expression is reported too: inside another element it is replaced by the braced expression (`<div><>{items}</></div>` is `<div>{items}</div>`, an `auto` fix); anywhere else it is replaced by the bare expression (`const r = <>{items}</>` is `const r = items`), a suggestion, since the value is then no longer a React element. A fragment holding a spread, a comment or a sequence is left alone.

**Why:** A fragment exists to group several children; around one child it's noise.

```codelaw
match:
  kind: jsx_element
  has: { field: open_tag, regex: '^<>$' }
  any:
    - pattern: $F
      all:
        - not: { has: { kind: jsx_expression } }
        - not: { has: { kind: jsx_text, regex: '\S' } }
        - has:
            any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }]
            pattern: $C
            not:
              any:
                - precedes: { stopBy: end, any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }] }
                - follows: { stopBy: end, any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }] }
    - pattern: $G
      inside: { kind: jsx_element }
      all:
        - not: { has: { kind: jsx_text, regex: '\S' } }
        - not: { has: { nthChild: 4 } }
        - has:
            nthChild: 2
            kind: jsx_expression
            pattern: $X
            not: { has: { kind: comment } }
            has:
              nthChild: 1
              not: { any: [{ kind: spread_element }, { kind: sequence_expression }] }
    - pattern: $H
      not: { inside: { kind: jsx_element } }
      all:
        - not: { has: { kind: jsx_text, regex: '\S' } }
        - not: { has: { nthChild: 4 } }
        - has:
            nthChild: 2
            kind: jsx_expression
            not: { has: { kind: comment } }
            has:
              nthChild: 1
              pattern: $E
              not: { any: [{ kind: spread_element }, { kind: sequence_expression }] }
auto_when:
  any:
    - not: { has: { kind: jsx_expression } }
    - inside: { kind: jsx_element }
message: "This fragment wraps a single child; drop it"
fix:
  - replace: { target: $F, with: '$C' }
  - replace: { target: $G, with: '$X' }
  - replace: { target: $H, with: '$E' }
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
