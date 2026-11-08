---
id: react/a11y-autocomplete-valid
title: "autoComplete holds a valid token list"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "the nearest valid field name by spelling (at most two edits, and no other as near) is offered; which was meant is still the author's, and a field name with no single near one is fixed by hand"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/autocomplete-valid]
since: 0.5.0
---

An `<input>`'s `autoComplete` written as a string is `on`, `off`, or an HTML autofill detail token list ending in a field name.

**Why:** Browsers and password managers ignore an invalid value, and users with motor or memory impairments lose autofill.

```codelaw
match:
  kind: jsx_attribute
  regex: '^autoComplete=["'']'
  has: { kind: string, pattern: $V }
  not: { regex: '^autoComplete=["'']\s*(on|off|(section-\S+\s+)?((shipping|billing)\s+)?((home|work|mobile|fax|pager)\s+)?(name|honorific-prefix|given-name|additional-name|family-name|honorific-suffix|nickname|username|new-password|current-password|one-time-code|organization-title|organization|street-address|address-line[123]|address-level[1-4]|country|country-name|postal-code|cc-name|cc-given-name|cc-additional-name|cc-family-name|cc-number|cc-exp|cc-exp-month|cc-exp-year|cc-csc|cc-type|transaction-currency|transaction-amount|language|bday|bday-day|bday-month|bday-year|sex|url|photo|tel|tel-country-code|tel-national|tel-area-code|tel-local|tel-local-prefix|tel-local-suffix|tel-extension|email|impp)(\s+webauthn)?)\s*["'']$' }
  inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^input$' } }
message: "`$$MATCH` is not a valid autofill value"
fix:
  - replace: { target: $V, with: "$V|nearest(name, honorific-prefix, given-name, additional-name, family-name, honorific-suffix, nickname, username, new-password, current-password, one-time-code, organization-title, organization, street-address, country, country-name, postal-code, cc-name, cc-given-name, cc-additional-name, cc-family-name, cc-number, cc-exp, cc-exp-month, cc-exp-year, cc-csc, cc-type, transaction-currency, transaction-amount, language, bday, bday-day, bday-month, bday-year, sex, url, photo, tel, tel-country-code, tel-national, tel-area-code, tel-local, tel-local-prefix, tel-local-suffix, tel-extension, email, impp, address-line1, address-line2, address-line3, address-level1, address-level2, address-level3, address-level4, on, off)" }
```

## How to fix

1. Use a field name from the HTML autofill list (`email`, `given-name`, `postal-code`, …), or `on`/`off`.

## Examples

### JavaScript

#### Don't

```js
const e = <input name="email" autoComplete="mail" />;
```

#### Do

```js
const e = <input name="email" autoComplete="email" />;
```

### JSX

#### Don't

```jsx
const e = <input name="email" autoComplete="mail" />;
```

#### Do

```jsx
const e = <input name="email" autoComplete="email" />;
```

### TSX

#### Don't

```tsx
const e = <input name="email" autoComplete="mail" />;
```

#### Do

```tsx
const e = <input name="email" autoComplete="email" />;
```
