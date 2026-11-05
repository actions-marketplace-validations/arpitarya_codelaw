---
id: react/no-children-prop
title: "Children are passed as children, not as a prop"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "a self-closing element whose last attribute is children={...} moves it into the body with identical output; the createElement form and other placements stay suggest"
stability: preview
ported_from: [eslint-plugin-react/no-children-prop]
since: 0.1.0
---

Children go between an element's tags (or as the third argument of `createElement`), not in a
`children` prop.

**Why:** Passing `children` as a prop works, but hides the element's content where readers don't
look for it.

```codelaw
match:
  any:
    - kind: jsx_attribute
      regex: ^children\b
      inside:
        stopBy: neighbor
        all:
          - pattern: $EL
          - kind: jsx_self_closing_element
          - pattern: '<$N $$$A children={$C} />'
    - kind: jsx_attribute
      regex: ^children\b
      not:
        inside:
          stopBy: neighbor
          kind: jsx_self_closing_element
          pattern: '<$N $$$A children={$C} />'
    - pattern: 'createElement($T, { children: $C, $$$REST })'
    - pattern: 'React.createElement($T, { children: $C, $$$REST })'
message: "Pass children between the tags, not as the `children` prop"
fix:
  - replace: {target: $EL, with: '<$N $$$A>{$C}</$N>'}
```

## How to fix

1. Move the value between the opening and closing tags.

## Examples

### JavaScript

#### Don't

```js
const a = <Panel children={<p>Hi</p>} />;
```

#### Fixed

```js
const a = <Panel >{<p>Hi</p>}</Panel>;
```

#### Do

```js
const a = <Panel>
  <p>Hi</p>
</Panel>;
```

### JSX

#### Don't

```jsx
const a = <Panel children={<p>Hi</p>} />;
```

#### Fixed

```jsx
const a = <Panel >{<p>Hi</p>}</Panel>;
```

#### Do

```jsx
const a = <Panel>
  <p>Hi</p>
</Panel>;
```

### TSX

#### Don't

```tsx
const a = <Panel children={<p>Hi</p>} />;
```

#### Fixed

```tsx
const a = <Panel >{<p>Hi</p>}</Panel>;
```

#### Do

```tsx
const a = <Panel>
  <p>Hi</p>
</Panel>;
```
