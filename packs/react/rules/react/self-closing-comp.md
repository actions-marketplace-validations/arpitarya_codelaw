---
id: react/self-closing-comp
title: "An element with no children closes itself"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "an element with nothing, or only a line break, between its tags renders the same self-closed; spaces alone on one line are a text child that renders, so closing the element drops them, a suggestion"
stability: preview
ported_from: [eslint-plugin-react/self-closing-comp]
since: 0.5.0
---

An element with no children is written self-closing: `<Spinner />`, not `<Spinner></Spinner>`. An element with only white space between its tags is reported too. White space with a line break is dropped by JSX, so that fix is `auto`; spaces alone on one line are a text child that renders, so removing them is a suggestion. A non-breaking space or a character reference is content and is left alone.

**Why:** The closing tag carries no information when there is nothing between the tags.

```codelaw
match:
  kind: jsx_element
  pattern: $E
  all:
    - has: { field: open_tag, not: { regex: '^<>$' } }
    - has: { field: close_tag, regex: '.' }
    - not: { has: { nthChild: 4 } }
    - any:
        - not: { has: { nthChild: 3 } }
        - has: { nthChild: 2, kind: jsx_text, regex: '^[ \t\r\n]+$' }
auto_when:
  not: { has: { kind: jsx_text } }
message: "Write this element self-closing"
fix:
  - edit-text: { target: $E, pattern: '\s*>\s*</[^<>]*>\s*$', replace: ' />' }
```

## Examples

### JavaScript

#### Don't

```js
const s = <Spinner size="sm"></Spinner>;
```

#### Do

```js
const s = <Spinner size="sm" />;
```

### JSX

#### Don't

```jsx
const s = <Spinner size="sm"></Spinner>;
```

#### Do

```jsx
const s = <Spinner size="sm" />;
```

### TSX

#### Don't

```tsx
const s = <Spinner size="sm"></Spinner>;
```

#### Do

```tsx
const s = <Spinner size="sm" />;
```
