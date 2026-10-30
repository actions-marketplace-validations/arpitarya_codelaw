---
id: react/incompatible-library
title: "No library API that defeats memoisation"
languages: [javascript, jsx, tsx]
severity: warn
fix: manual
fix_reason: "restructure: the component either opts out of compilation or reads the value through the library's memo-safe API"
stability: preview
ported_from: [eslint-plugin-react-hooks/incompatible-library]
since: 0.5.0
---

A component does not use library APIs that return functions or objects whose results change without their identity changing, which automatic memoisation cannot follow: TanStack Table's `useReactTable`, TanStack Virtual's `useVirtualizer` and `useWindowVirtualizer`, and React Hook Form's `watch` taken from `useForm`.

**Why:** The React Compiler memoises by identity; these APIs mutate in place, so memoised output goes stale.

```codelaw
match:
  any:
    - kind: call_expression
      has: { field: function, kind: identifier, not: { regex: '^useForm$' }, pattern: $F }
    - kind: variable_declarator
      all:
        - has: { field: name, kind: object_pattern, has: { kind: shorthand_property_identifier_pattern, regex: '^watch$' } }
        - has: { field: value, kind: call_expression, has: { field: function, kind: identifier, pattern: $F } }
where:
  $F:
    any:
      - { imported-from: { module: "@tanstack/react-table", name: useReactTable } }
      - { imported-from: { module: "@tanstack/react-virtual", name: useVirtualizer } }
      - { imported-from: { module: "@tanstack/react-virtual", name: useWindowVirtualizer } }
      - { imported-from: { module: react-hook-form, name: useForm } }
message: "This library API changes in place, which automatic memoisation can't follow"
```

## How to fix

1. Add the `"use no memo"` directive to the component, or use the library's memo-safe API (`useWatch` for React Hook Form).

## Examples

### JavaScript

#### Don't

```js
import { useForm } from "react-hook-form";
function F() {
  const { watch } = useForm();
  return watch("name");
}
```

#### Do

```js
import { useForm, useWatch } from "react-hook-form";
function F() {
  const { control } = useForm();
  return useWatch({ control, name: "name" });
}
```

### JSX

#### Don't

```jsx
import { useForm } from "react-hook-form";
function F() {
  const { watch } = useForm();
  return watch("name");
}
```

#### Do

```jsx
import { useForm, useWatch } from "react-hook-form";
function F() {
  const { control } = useForm();
  return useWatch({ control, name: "name" });
}
```

### TSX

#### Don't

```tsx
import { useForm } from "react-hook-form";
function F() {
  const { watch } = useForm();
  return watch("name");
}
```

#### Do

```tsx
import { useForm, useWatch } from "react-hook-form";
function F() {
  const { control } = useForm();
  return useWatch({ control, name: "name" });
}
```
