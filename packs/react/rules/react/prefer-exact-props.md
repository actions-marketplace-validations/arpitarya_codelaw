---
id: react/prefer-exact-props
title: "Prop types are exact"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
options:
  exact_wrappers: { type: "string[]", default: [], description: "the functions that make prop types exact, such as airbnb-prop-types `exact`; the first is used for the fix" }
ported_from: [eslint-plugin-react/prefer-exact-props]
since: 0.5.0
---

A component's `propTypes` object is wrapped in the project's exact-props function, so an undeclared prop is an error. With no option set, the rule does nothing.

**Why:** A misspelt prop otherwise passes silently.

```codelaw
native: react/prefer-exact-props
```

## Examples

### JavaScript

#### Don't

```js options={"exact_wrappers":["exact"]}
Card.propTypes = { title: s };
```

#### Do

```js options={"exact_wrappers":["exact"]}
Card.propTypes = exact({ title: s });
```

### JSX

#### Don't

```jsx options={"exact_wrappers":["exact"]}
Card.propTypes = { title: s };
```

#### Do

```jsx options={"exact_wrappers":["exact"]}
Card.propTypes = exact({ title: s });
```

### TSX

#### Don't

```tsx options={"exact_wrappers":["exact"]}
Card.propTypes = { title: s };
```

#### Do

```tsx options={"exact_wrappers":["exact"]}
Card.propTypes = exact({ title: s });
```
