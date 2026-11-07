---
id: js/no-invalid-dom-token
title: "No space inside one classList token"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "two class names may have been meant, or one name with a stray space; splitting it is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-dom-token]
since: 0.5.0
---

`classList.add` and `classList.remove` are not given one string with a space in it: a token cannot hold whitespace, and the call throws. Two names are two arguments.

**Why:** `classList.add("a b")` throws an `InvalidCharacterError` at run time.

```codelaw
match:
  pattern: "$E.classList.$M($S)"
constraints:
  M: { regex: '^(add|remove)$' }
  S: { kind: string, regex: '^(["''])[^\s"'']+\s+[^\s"'']+["'']$' }
message: "A class token can't contain a space; pass two arguments"
fix:
  - edit-text: { target: $S, pattern: '^(["''])([^\s"'']+)\s+([^\s"'']+)(["''])$', replace: '${1}${2}${4}, ${1}${3}${4}' }
```

## Examples

### JavaScript

#### Don't

```js
el.classList.add("btn primary");
```

#### Do

```js
el.classList.add("btn", "primary");
```


### JSX

#### Don't

```jsx
el.classList.add("btn primary");
```

#### Do

```jsx
el.classList.add("btn", "primary");
```


### TypeScript

#### Don't

```ts
el.classList.add("btn primary");
```

#### Do

```ts
el.classList.add("btn", "primary");
```


### TSX

#### Don't

```tsx
el.classList.add("btn primary");
```

#### Do

```tsx
el.classList.add("btn", "primary");
```
