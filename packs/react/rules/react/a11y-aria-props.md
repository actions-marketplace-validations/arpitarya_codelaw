---
id: react/a11y-aria-props
title: "ARIA attributes are real ones"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the intended attribute is the author's to pick"
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
  not: { regex: '^aria-(activedescendant|atomic|autocomplete|braillelabel|brailleroledescription|busy|checked|colcount|colindex|colindextext|colspan|controls|current|describedby|description|details|disabled|dropeffect|errormessage|expanded|flowto|grabbed|haspopup|hidden|invalid|keyshortcuts|label|labelledby|level|live|modal|multiline|multiselectable|orientation|owns|placeholder|posinset|pressed|readonly|relevant|required|roledescription|rowcount|rowindex|rowindextext|rowspan|selected|setsize|sort|valuemax|valuemin|valuenow|valuetext)(=|$)' }
message: "`$$MATCH` is not a WAI-ARIA attribute"
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
