---
id: react/require-default-props
title: "Every optional prop has a default"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the default value is the author's"
stability: preview
ported_from: [eslint-plugin-react/require-default-props]
since: 0.5.0
---

Each prop in a component's `propTypes` that is not `.isRequired` has an entry in its `defaultProps`.

**Why:** An optional prop with no default is `undefined` in the component, and every use of it has to guard for that.

```codelaw
match:
  kind: pair
  all:
    - inside: { kind: object, inside: { kind: assignment_expression, has: { field: left, pattern: $C.propTypes } } }
    - has: { field: key, pattern: $K }
    - has: { field: value, not: { regex: '\.isRequired$' } }
    - not:
        inside:
          stopBy: end
          kind: program
          has:
            stopBy: end
            kind: assignment_expression
            all:
              - has: { field: left, pattern: $C.defaultProps }
              - has: { field: right, has: { kind: pair, has: { field: key, pattern: $K } } }
message: "The optional prop `$K` has no default in `$C.defaultProps`"
```

## How to fix

1. Add a default for the prop to `defaultProps`, or mark it `.isRequired`.

## Examples

### JavaScript

#### Don't

```js
Badge.propTypes = { label: PropTypes.string };
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
```

#### Do

```tsx
Badge.propTypes = { label: PropTypes.string };
Badge.defaultProps = { label: "" };
```
