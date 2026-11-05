---
id: react/jsx-curly-brace-presence
title: "No braces around a plain string prop"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/jsx-curly-brace-presence]
since: 0.5.0
---

A prop whose value is a string literal in braces (`title={"Save"}`) is written as a plain attribute string (`title="Save"`). Strings with a backslash or a newline are left alone, since an attribute string reads them differently.

**Why:** The braces add nothing to a fixed string, and the plain form is what the rest of the markup uses.

```codelaw
match:
  kind: jsx_attribute
  all:
    - has: { nthChild: 1, pattern: $P }
    - has:
        kind: jsx_expression
        has: { kind: string, pattern: $S, not: { regex: '[\\\n]' } }
        not: { has: { kind: comment } }
message: "Write `$P=$S` without the braces"
fix: "$P=$S"
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
