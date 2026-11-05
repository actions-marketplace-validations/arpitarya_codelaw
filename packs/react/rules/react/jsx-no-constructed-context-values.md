---
id: react/jsx-no-constructed-context-values
title: "A context value is not rebuilt on every render"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the value is memoised with `useMemo` (or hoisted), and its dependencies are the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/jsx-no-constructed-context-values]
since: 0.5.0
---

The `value` of a `<Context.Provider>` is not an object, array, function or `new` expression written in place.

**Why:** A new value on every render re-renders every consumer of the context, changed or not.

```codelaw
match:
  kind: jsx_attribute
  regex: '^value\b'
  inside:
    any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
    has: { field: name, regex: '\.Provider$' }
  has:
    kind: jsx_expression
    has: { any: [{ kind: object }, { kind: array }, { kind: arrow_function }, { kind: function_expression }, { kind: new_expression }, { kind: class }] }
message: "This context value is rebuilt on every render; memoise it"
```

## How to fix

1. Wrap the value in `useMemo` with its dependencies, or hoist it out of the component if it never changes.

## Examples

### JavaScript

#### Don't

```js
const p = <Theme.Provider value={{ mode, toggle }}>{children}</Theme.Provider>;
```

#### Do

```js
const p = <Theme.Provider value={theme}>{children}</Theme.Provider>;
```

### JSX

#### Don't

```jsx
const p = <Theme.Provider value={{ mode, toggle }}>{children}</Theme.Provider>;
```

#### Do

```jsx
const p = <Theme.Provider value={theme}>{children}</Theme.Provider>;
```

### TSX

#### Don't

```tsx
const p = <Theme.Provider value={{ mode, toggle }}>{children}</Theme.Provider>;
```

#### Do

```tsx
const p = <Theme.Provider value={theme}>{children}</Theme.Provider>;
```
