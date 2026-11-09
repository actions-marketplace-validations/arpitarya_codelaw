---
id: js/no-shorthand-property-overrides
title: "No CSS shorthand after a longhand it resets in a `style` object"
languages: [jsx, tsx]
fix: manual
fix_reason: "which value should win is a styling choice: move the shorthand first, fold the longhand into it, or drop the earlier one"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-shorthand-property-overrides]
since: 0.5.0
---

In the `style={{…}}` prop of an intrinsic JSX element (`<div>`, not a component), a shorthand property written after a longhand it covers — `paddingLeft: 10, padding: 20`, `backgroundRepeat: 'no-repeat', background: 'url(a.png)'` — is reported at the shorthand's key: the shorthand resets the longhand, so the earlier value is lost. Property names are camelCase or quoted CSS names, vendor prefixes match (`WebkitTransition`), and the values must be literal strings, numbers or untemplated template strings; a dynamic value, an `!important` one, an object with a spread, method or computed key, and a component's `style` prop are not checked. The shorthand table is CSS's (the affected longhands include those reset indirectly: `border-image` by `border`, `mask-border` by `mask`). Stylesheets are the `css` pack's `css/declaration-block-no-shorthand-property-overrides`; the same check for CSS in upstream's rule is not built here.

**Why:** A shorthand resets every longhand it covers, so a longhand written before it never applies.

```codelaw
native: js/no-shorthand-property-overrides
```

## How to fix

1. Write the shorthand first and the longhand after it, so the longhand refines it.
2. Or put the longhand's value inside the shorthand and delete the longhand.

## Examples

### JSX

#### Don't

```jsx
const el = <div style={{ paddingLeft: 10, padding: 20 }} />;
```

#### Do

```jsx
const el = <div style={{ padding: 20, paddingLeft: 10 }} />;
```


### TSX

#### Don't

```tsx
const el = <div style={{ paddingLeft: 10, padding: 20 }} />;
```

#### Do

```tsx
const el = <div style={{ padding: 20, paddingLeft: 10 }} />;
```
