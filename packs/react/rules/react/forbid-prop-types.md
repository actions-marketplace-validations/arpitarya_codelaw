---
id: react/forbid-prop-types
title: "No PropTypes.any, array or object"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the precise type — `arrayOf(…)`, `shape({…})` — is the author's"
stability: preview
ported_from: [eslint-plugin-react/forbid-prop-types]
since: 0.5.0
---

`propTypes` don't use the vague `PropTypes.any`, `PropTypes.array` or `PropTypes.object`.

**Why:** A vague prop type validates almost nothing; `arrayOf` and `shape` say what's expected.

```codelaw
match:
  kind: member_expression
  regex: '^PropTypes\.(any|array|object)$'
  inside:
    stopBy: end
    any:
      - { kind: assignment_expression, has: { field: left, regex: '\.propTypes$' } }
      - { any: [{ kind: field_definition }, { kind: public_field_definition }], regex: '^static\s+propTypes\b' }
message: "`$$MATCH` is too vague; say the shape"
```

## How to fix

1. Use `PropTypes.arrayOf(…)`, `PropTypes.shape({…})` or `PropTypes.objectOf(…)` with the real element type.

## Examples

### JavaScript

#### Don't

```js
List.propTypes = { items: PropTypes.array };
```

#### Do

```js
List.propTypes = { items: PropTypes.arrayOf(PropTypes.string) };
```

### JSX

#### Don't

```jsx
List.propTypes = { items: PropTypes.array };
```

#### Do

```jsx
List.propTypes = { items: PropTypes.arrayOf(PropTypes.string) };
```

### TSX

#### Don't

```tsx
List.propTypes = { items: PropTypes.array };
```

#### Do

```tsx
List.propTypes = { items: PropTypes.arrayOf(PropTypes.string) };
```
