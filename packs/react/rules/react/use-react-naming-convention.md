---
id: react/use-react-naming-convention
title: "React's context, id and ref variables are named for what they hold"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "renaming a variable changes every use of it, which the author does with the editor's rename"
fix_exception: restructure
stability: preview
ported_from: [biome/nursery/useReactNamingConvention, "@eslint-react/naming-convention-context-name", "@eslint-react/naming-convention-id-name", "@eslint-react/naming-convention-ref-name"]
since: 0.5.1
---

A variable set to `createContext(…)` is named in PascalCase and ends in `Context` (`ThemeContext`); one set to `useId()` is `id` or camelCase ending in `Id` (`inputId`); one set to `useRef(…)` is `ref` or camelCase ending in `Ref` (`inputRef`). `React.createContext`, `React.useId` and `React.useRef` count too. Behaviour of Biome's `useReactNamingConvention`; the call is matched by name, not by its import.

**Why:** The suffix tells a reader what the variable is before they find its declaration: a `Context` goes in `useContext`, an `Id` in an attribute, a `Ref` in a `ref` prop.

```codelaw
match:
  kind: variable_declarator
  has: { field: name, kind: identifier }
  any:
    - all:
        - has: { field: value, kind: call_expression, has: { field: function, regex: '^(React\.)?createContext$' } }
        - not: { has: { field: name, regex: '^[A-Z][A-Za-z0-9]*Context$' } }
    - all:
        - has: { field: value, kind: call_expression, has: { field: function, regex: '^(React\.)?useId$' } }
        - not: { has: { field: name, regex: '^(id|[a-z][A-Za-z0-9]*Id)$' } }
    - all:
        - has: { field: value, kind: call_expression, has: { field: function, regex: '^(React\.)?useRef$' } }
        - not: { has: { field: name, regex: '^(ref|[a-z][A-Za-z0-9]*Ref)$' } }
message: "Name this variable for what it holds: `…Context`, `…Id` or `…Ref`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const theme = createContext("light");
```

#### Do

```js
const ThemeContext = createContext("light");
```

### JSX

#### Don't

```jsx
const theme = createContext("light");
```

#### Do

```jsx
const ThemeContext = createContext("light");
```

### TSX

#### Don't

```tsx
const theme = createContext("light");
```

#### Do

```tsx
const ThemeContext = createContext("light");
```
