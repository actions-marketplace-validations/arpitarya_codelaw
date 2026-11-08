---
id: js/no-conflicting-constraints
title: "No `<input>` or `<textarea>` whose lower bound is above its upper bound"
languages: [jsx, tsx]
fix: manual
fix_reason: "which bound is wrong is the author's knowledge"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-conflicting-constraints]
since: 0.5.0
---

A JSX `<input type="number">` or `type="range"` with `min` above `max`, and a `<textarea>` or text-like `<input>` (`text`, `search`, `url`, `tel`, `email`, `password`, or no type) with `minLength` above `maxLength`, is reported at the upper bound. Only static values are compared; a spread, a repeated or dynamic attribute leaves the element alone. Upstream checks CSS media and container queries and HTML too; those belong to the `css` and `html` packs.

**Why:** No value can satisfy both bounds, so the field can never be valid.

```codelaw
native: js/no-conflicting-constraints
```

## How to fix

1. Swap or correct the two bounds.

## Examples

### JSX

#### Don't

```jsx
const field = <input type="range" min="9" max="2" />;
```

#### Do

```jsx
const field = <input type="range" min="2" max="9" />;
```


### TSX

#### Don't

```tsx
const field = <input type="range" min="9" max="2" />;
```

#### Do

```tsx
const field = <input type="range" min="2" max="9" />;
```
