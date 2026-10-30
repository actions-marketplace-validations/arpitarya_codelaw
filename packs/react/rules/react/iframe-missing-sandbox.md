---
id: react/iframe-missing-sandbox
title: "An <iframe> has a sandbox"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "an empty `sandbox` blocks scripts, forms and same-origin access in the frame, which a page that needs them must loosen by name"
stability: preview
ported_from: [eslint-plugin-react/iframe-missing-sandbox]
since: 0.5.0
---

Every `<iframe>` has a `sandbox` attribute.

**Why:** An unsandboxed frame runs its scripts with the full rights the browser gives its origin.

```codelaw
match:
  any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
  all:
    - has: { field: name, regex: '^iframe$', pattern: $N }
    - not: { has: { kind: jsx_attribute, regex: '^sandbox\b' } }
    - not: { has: { kind: jsx_expression, has: { kind: spread_element } } }
message: "Add a `sandbox` to this `<iframe>`"
fix:
  - insert-after: { target: $N, text: ' sandbox=""' }
```

## Examples

### JavaScript

#### Don't

```js
const f = <iframe src={url} title="Map" />;
```

#### Do

```js
const f = <iframe sandbox="" src={url} title="Map" />;
```

### JSX

#### Don't

```jsx
const f = <iframe src={url} title="Map" />;
```

#### Do

```jsx
const f = <iframe sandbox="" src={url} title="Map" />;
```

### TSX

#### Don't

```tsx
const f = <iframe src={url} title="Map" />;
```

#### Do

```tsx
const f = <iframe sandbox="" src={url} title="Map" />;
```
