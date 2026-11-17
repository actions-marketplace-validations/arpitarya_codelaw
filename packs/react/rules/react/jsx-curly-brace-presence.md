---
id: react/jsx-curly-brace-presence
title: "No braces around a plain string prop"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/jsx-curly-brace-presence]
since: 0.5.0
---

A prop whose value is a string literal in braces (`title={"Save"}`) is written as a plain attribute string (`title="Save"`). A child that is a string literal in braces (`<b>{"Save"}</b>`) is written as plain text (`<b>Save</b>`). Strings with a backslash or a newline are left alone, since an attribute string reads them differently. A child string is also left alone when it is empty, starts or ends with a space, or holds `<`, `>`, `{`, `}` or `&`, which text reads as markup; and when its neighbour is text on another line (or two braced children are on separate lines), since JSX joins the lines of a text with a space and the fix would change the string it renders.

**Why:** The braces add nothing to a fixed string, and the plain form is what the rest of the markup uses.

```codelaw
match:
  any:
    - kind: jsx_attribute
      pattern: $A
      all:
        - has: { nthChild: 1, pattern: $P }
        - has:
            kind: jsx_expression
            has: { kind: string, pattern: $S, not: { regex: '[\\\n]' } }
            not: { has: { kind: comment } }
    - kind: jsx_expression
      pattern: $X
      inside: { kind: jsx_element }
      has:
        kind: string
        pattern: $C
        all:
          - regex: '^["''][^\s](.*[^\s])?["'']$'
          - not: { regex: '[\\<>{}&]' }
      all:
        - not: { has: { kind: comment } }
        - not: { follows: { kind: jsx_text, regex: '\S\s*\n\s*$' } }
        - not: { precedes: { kind: jsx_text, regex: '^\s*\n\s*\S' } }
        - not: { follows: { kind: jsx_text, regex: '^\s*\n\s*$', follows: { kind: jsx_expression } } }
        - not: { precedes: { kind: jsx_text, regex: '^\s*\n\s*$', precedes: { kind: jsx_expression } } }
transform:
  T: { substring: { source: $C, startChar: 1, endChar: -1 } }
message: "Write this string without the braces"
fix:
  - replace: { target: $A, with: '$P=$S' }
  - replace: { target: $X, with: '$T' }
```

## Examples

### JavaScript

#### Don't

```js
const b = <Button title={"Save"} />;
```

#### Do

```js
const b = <Button title="Save" />;
```

### JSX

#### Don't

```jsx
const b = <Button title={"Save"} />;
```

#### Do

```jsx
const b = <Button title="Save" />;
```

### TSX

#### Don't

```tsx
const b = <Button title={"Save"} />;
```

#### Do

```tsx
const b = <Button title="Save" />;
```
