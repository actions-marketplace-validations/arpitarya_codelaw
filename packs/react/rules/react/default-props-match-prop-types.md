---
id: react/default-props-match-prop-types
title: "Every default prop is a declared prop"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: either the prop is declared, or the default is removed — which one is the author's"
stability: preview
ported_from: [eslint-plugin-react/default-props-match-prop-types]
since: 0.5.0
---

Each key of a component's `defaultProps` is declared in its `propTypes`. Components with no `propTypes` assignment in the file are not checked.

**Why:** A default for an undeclared prop is a leftover or a typo: it is never validated and is usually never read.

```codelaw
match:
  kind: pair
  all:
    - inside: { kind: object, inside: { kind: assignment_expression, has: { field: left, pattern: $C.defaultProps } } }
    - has: { field: key, pattern: $K }
    - inside:
        stopBy: end
        kind: program
        has: { stopBy: end, kind: assignment_expression, has: { field: left, pattern: $C.propTypes } }
    - not:
        inside:
          stopBy: end
          kind: program
          has:
            stopBy: end
            kind: assignment_expression
            all:
              - has: { field: left, pattern: $C.propTypes }
              - has: { field: right, has: { kind: pair, has: { field: key, pattern: $K } } }
message: "`$K` has a default but is not in `$C.propTypes`"
```

## How to fix

1. Declare the prop in `propTypes`, or remove the default if nothing reads it.

## Examples

### JavaScript

#### Don't

```js
Badge.propTypes = { label: PropTypes.string };
Badge.defaultProps = { lable: "" };
```

#### Do

```js
Badge.propTypes = { label: PropTypes.string };
Badge.defaultProps = { label: "" };
```

### JSX

#### Don't

```jsx
Badge.propTypes = { label: PropTypes.string };
Badge.defaultProps = { lable: "" };
```

#### Do

```jsx
Badge.propTypes = { label: PropTypes.string };
Badge.defaultProps = { label: "" };
```

### TSX

#### Don't

```tsx
Badge.propTypes = { label: PropTypes.string };
Badge.defaultProps = { lable: "" };
```

#### Do

```tsx
Badge.propTypes = { label: PropTypes.string };
Badge.defaultProps = { label: "" };
```
