---
id: js/better-dom-traversing
title: "firstChild and firstElementChild rather than indexing the child lists"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "on an element with no children the index gives `undefined` and the property gives `null`; code that tests for `undefined` needs review"
stability: preview
ported_from: [eslint-plugin-unicorn/better-dom-traversing]
since: 0.5.0
---

The first node of `element.childNodes` is `element.firstChild`, and the first element of `element.children` is `element.firstElementChild`. `props.children` is component data, not the DOM, and is left. Upstream also rewrites chains of `parentElement` and of `querySelector`; those need the selector the author means, and are not reported here.

**Why:** The named property says which child is read, and doesn't build or index a collection.

```codelaw
match:
  kind: subscript_expression
  any:
    - all:
        - pattern: "$E.childNodes[0]"
        - pattern: $W1
    - all:
        - pattern: "$E.children[0]"
        - pattern: $W2
  not: { any: [{ has: { stopBy: end, kind: optional_chain } }, { inside: { kind: assignment_expression, field: left } }] }
constraints:
  E: { not: { any: [{ regex: '^props$' }, { regex: '\.props$' }] } }
message: "Use the named child property"
fix:
  - replace: { target: $W1, with: "$E.firstChild" }
  - replace: { target: $W2, with: "$E.firstElementChild" }
```

## Examples

### JavaScript

#### Don't

```js
const first = list.children[0];
```

#### Do

```js
const first = list.firstElementChild;
```


### JSX

#### Don't

```jsx
const first = list.children[0];
```

#### Do

```jsx
const first = list.firstElementChild;
```


### TypeScript

#### Don't

```ts
const first = list.children[0];
```

#### Do

```ts
const first = list.firstElementChild;
```


### TSX

#### Don't

```tsx
const first = list.children[0];
```

#### Do

```tsx
const first = list.firstElementChild;
```
