---
id: js/no-transition-all
title: "No transition of every property"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which properties should animate is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-transition-all]
since: 0.5.0
---

A style set from code does not transition `all` properties: `transition: "all 0.3s"` in a style object, or assigned to `el.style.transition`, names the properties it animates.

**Why:** `all` animates every property that changes, layout ones included, which costs frames and animates changes nobody meant to.

```codelaw
utils:
  all-value: { regex: '^["''`]all(\s|["''`])' }
match:
  any:
    - kind: pair
      has: { field: key, regex: '^["'']?(transition|transitionProperty|transition-property)["'']?$' }
      all: [{ has: { field: value, matches: all-value } }]
    - pattern: "$E.style.$P = $V"
      has: { field: left, has: { field: property, regex: '^(transition|transitionProperty)$' } }
      all: [{ has: { field: right, matches: all-value } }]
message: "Name the properties to transition, not `all`"
```

## How to fix

1. Replace `all` with the properties that change, for example `opacity 0.3s, transform 0.3s`.

## Examples

### JavaScript

#### Don't

```js
const style = { transition: "all 0.3s ease" };
```

#### Do

```js
const style = { transition: "opacity 0.3s ease" };
```


### JSX

#### Don't

```jsx
const style = { transition: "all 0.3s ease" };
```

#### Do

```jsx
const style = { transition: "opacity 0.3s ease" };
```


### TypeScript

#### Don't

```ts
const style = { transition: "all 0.3s ease" };
```

#### Do

```ts
const style = { transition: "opacity 0.3s ease" };
```


### TSX

#### Don't

```tsx
const style = { transition: "all 0.3s ease" };
```

#### Do

```tsx
const style = { transition: "opacity 0.3s ease" };
```
