---
id: react/use-tailwind-static-class-strings
title: "A Tailwind `className` is written out in full"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the complete class names are written in a map or a conditional, which the author lays out"
fix_exception: restructure
stability: preview
ported_from: [biome/nursery/useTailwindStaticClassStrings, eslint-plugin-shadcn/require-static-classes]
since: 0.5.1
---

A `className` or `class` attribute is not a template literal with a `${…}` in it, and is not a string joined with `+`. A conditional that chooses between complete strings, and `cn(…)` calls with complete strings, are fine. Behaviour of Biome's `useTailwindStaticClassStrings`, as it reads JSX.

**Why:** Tailwind finds class names by scanning the source for whole names: `bg-${color}` never appears in it, so the style is not generated.

```codelaw
match:
  kind: jsx_attribute
  all:
    - has: { kind: property_identifier, regex: '^(className|class)$' }
    - has:
        kind: jsx_expression
        any:
          - has: { kind: template_string, has: { kind: template_substitution } }
          - has: { kind: binary_expression, regex: '^[^+]*\+' }
message: "Write the class names out in full; Tailwind cannot see a name built at run time"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const d = <div className={`bg-${color}`} />;
```

#### Do

```js
const d = <div className={color === "red" ? "bg-red-500" : "bg-blue-500"} />;
```

### JSX

#### Don't

```jsx
const d = <div className={`bg-${color}`} />;
```

#### Do

```jsx
const d = <div className={color === "red" ? "bg-red-500" : "bg-blue-500"} />;
```

### TSX

#### Don't

```tsx
const d = <div className={`bg-${color}`} />;
```

#### Do

```tsx
const d = <div className={color === "red" ? "bg-red-500" : "bg-blue-500"} />;
```
