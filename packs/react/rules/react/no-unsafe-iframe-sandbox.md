---
id: react/no-unsafe-iframe-sandbox
title: "An `<iframe>` sandbox does not combine `allow-scripts` and `allow-same-origin`"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "which of the two permissions the framed page needs is the author's decision"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noUnsafeIframeSandbox, "@eslint-react/dom-no-unsafe-iframe-sandbox"]
since: 0.5.1
---

A `sandbox="…"` string on an `<iframe>` that holds both `allow-scripts` and `allow-same-origin` is reported. Behaviour of Biome's `noUnsafeIframeSandbox`; `react/iframe-missing-sandbox` is the rule for no sandbox at all.

**Why:** A framed page with both permissions can remove its own `sandbox` attribute and run with its full origin: the sandbox is then decoration.

```codelaw
match:
  any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
  all:
    - has: { field: name, kind: identifier, regex: '^iframe$' }
    - has:
        kind: jsx_attribute
        all:
          - has: { kind: property_identifier, regex: '^sandbox$' }
          - has: { kind: string, all: [{ regex: 'allow-scripts' }, { regex: 'allow-same-origin' }] }
message: "A sandbox with `allow-scripts` and `allow-same-origin` can lift itself"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const f = <iframe sandbox="allow-scripts allow-same-origin" src={url} />;
```

#### Do

```js
const f = <iframe sandbox="allow-scripts" src={url} />;
```

### JSX

#### Don't

```jsx
const f = <iframe sandbox="allow-scripts allow-same-origin" src={url} />;
```

#### Do

```jsx
const f = <iframe sandbox="allow-scripts" src={url} />;
```

### TSX

#### Don't

```tsx
const f = <iframe sandbox="allow-scripts allow-same-origin" src={url} />;
```

#### Do

```tsx
const f = <iframe sandbox="allow-scripts" src={url} />;
```
