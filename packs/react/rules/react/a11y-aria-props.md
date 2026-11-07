---
id: react/a11y-aria-props
title: "ARIA attributes are real ones"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "the nearest valid attribute by spelling (at most two edits, and no other as near) is offered; which was meant is still the author's, and a attribute with no single near one is fixed by hand"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/aria-props]
since: 0.5.0
---

Every `aria-*` attribute is one WAI-ARIA 1.2 defines.

**Why:** A misspelled ARIA attribute (`aria-labeledby`) is silently ignored by assistive technology.

```codelaw
match:
  kind: jsx_attribute
  regex: '^aria-'
  has: { kind: property_identifier, pattern: $NAME }
  not: { regex: '^aria-(activedescendant|atomic|autocomplete|braillelabel|brailleroledescription|busy|checked|colcount|colindex|colindextext|colspan|controls|current|describedby|description|details|disabled|dropeffect|errormessage|expanded|flowto|grabbed|haspopup|hidden|invalid|keyshortcuts|label|labelledby|level|live|modal|multiline|multiselectable|orientation|owns|placeholder|posinset|pressed|readonly|relevant|required|roledescription|rowcount|rowindex|rowindextext|rowspan|selected|setsize|sort|valuemax|valuemin|valuenow|valuetext)(=|$)' }
message: "`$$MATCH` is not a WAI-ARIA attribute"
fix:
  - replace: { target: $NAME, with: "$NAME|nearest(aria-activedescendant, aria-atomic, aria-autocomplete, aria-braillelabel, aria-brailleroledescription, aria-busy, aria-checked, aria-colcount, aria-colindex, aria-colindextext, aria-colspan, aria-controls, aria-current, aria-describedby, aria-description, aria-details, aria-disabled, aria-dropeffect, aria-errormessage, aria-expanded, aria-flowto, aria-grabbed, aria-haspopup, aria-hidden, aria-invalid, aria-keyshortcuts, aria-label, aria-labelledby, aria-level, aria-live, aria-modal, aria-multiline, aria-multiselectable, aria-orientation, aria-owns, aria-placeholder, aria-posinset, aria-pressed, aria-readonly, aria-relevant, aria-required, aria-roledescription, aria-rowcount, aria-rowindex, aria-rowindextext, aria-rowspan, aria-selected, aria-setsize, aria-sort, aria-valuemax, aria-valuemin, aria-valuenow, aria-valuetext)" }
```

## How to fix

1. Correct the spelling to the ARIA attribute you meant (for example `aria-labelledby`).

## Examples

### JavaScript

#### Don't

```js
const f = <input aria-labeledby="name" />;
```

#### Do

```js
const f = <input aria-labelledby="name" />;
```

### JSX

#### Don't

```jsx
const f = <input aria-labeledby="name" />;
```

#### Do

```jsx
const f = <input aria-labelledby="name" />;
```

### TSX

#### Don't

```tsx
const f = <input aria-labeledby="name" />;
```

#### Do

```tsx
const f = <input aria-labelledby="name" />;
```
