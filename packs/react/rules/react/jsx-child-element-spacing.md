---
id: react/jsx-child-element-spacing
title: "No inline element glued to text across a line break"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: whether the words were meant to run together is the author's; a `{\" \"}` changes the rendered text"
stability: preview
ported_from: [eslint-plugin-react/jsx-child-element-spacing]
since: 0.5.0
---

An inline element (`<a>`, `<strong>`, …) is not separated from neighbouring text only by a line break: JSX drops that whitespace, so the two render glued together.

**Why:** `<b>Hi</b>` on one line and `there` on the next renders `Hithere`, which the source does not show.

```codelaw
match:
  kind: jsx_text
  any:
    - all:
        - regex: '^[ \t]*\n\s*\S'
        - follows: { any: [{ kind: jsx_element, has: { field: open_tag, has: { field: name, regex: '^(a|abbr|b|bdo|cite|code|dfn|em|i|kbd|q|samp|small|span|strong|sub|sup|var)$' } } }] }
    - all:
        - regex: '\S[ \t]*\n\s*$'
        - precedes: { any: [{ kind: jsx_element, has: { field: open_tag, has: { field: name, regex: '^(a|abbr|b|bdo|cite|code|dfn|em|i|kbd|q|samp|small|span|strong|sub|sup|var)$' } } }] }
message: "JSX drops this line break, so the text runs into the element"
```

## How to fix

1. Put `{" "}` where a space is meant, or join the lines if no space is.

## Examples

### JavaScript

#### Don't

```js
const t = (
  <p>
    <b>Hi</b>
    there
  </p>
);
```

#### Do

```js
const t = (
  <p>
    <b>Hi</b>{" "}
    there
  </p>
);
```

### JSX

#### Don't

```jsx
const t = (
  <p>
    <b>Hi</b>
    there
  </p>
);
```

#### Do

```jsx
const t = (
  <p>
    <b>Hi</b>{" "}
    there
  </p>
);
```

### TSX

#### Don't

```tsx
const t = (
  <p>
    <b>Hi</b>
    there
  </p>
);
```

#### Do

```tsx
const t = (
  <p>
    <b>Hi</b>{" "}
    there
  </p>
);
```
