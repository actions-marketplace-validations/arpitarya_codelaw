---
id: js/no-invalid-intl-options
title: "Intl.NumberFormat and DateTimeFormat options are spelt as the API names them"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the replacement is the nearest valid option name, which is usually but not always the one meant"
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-intl-options]
since: 0.5.0
---

`Intl.NumberFormat` and `Intl.DateTimeFormat` ignore an option name they don't know, so `maximumFractionDigit: 2` silently does nothing. A key of an inline options object that is not an option of either formatter is reported, and replaced with the valid name within two edits of it when there is exactly one. Upstream also checks values, required pairs and conflicting options, and every Intl constructor; this rule checks the two formatters' option names.

**Why:** A misspelt option fails silently: the output just isn't formatted as intended.

```codelaw
match:
  kind: property_identifier
  inside:
    kind: pair
    field: key
    inside:
      kind: object
      inside:
        kind: arguments
        inside:
          any: [{ kind: new_expression }, { kind: call_expression }]
          regex: '^(new\s+)?Intl\.(NumberFormat|DateTimeFormat)\('
  pattern: $K
constraints:
  K: { not: { regex: '^(localeMatcher|style|currency|currencyDisplay|currencySign|unit|unitDisplay|minimumIntegerDigits|minimumFractionDigits|maximumFractionDigits|minimumSignificantDigits|maximumSignificantDigits|roundingPriority|roundingIncrement|roundingMode|trailingZeroDisplay|notation|compactDisplay|useGrouping|signDisplay|numberingSystem|calendar|hour12|hourCycle|timeZone|weekday|era|year|month|day|dayPeriod|hour|minute|second|fractionalSecondDigits|timeZoneName|formatMatcher|dateStyle|timeStyle)$' } }
message: "`$K` is not an option of this Intl formatter"
fix: "$K|nearest(localeMatcher, style, currency, currencyDisplay, currencySign, unit, unitDisplay, minimumIntegerDigits, minimumFractionDigits, maximumFractionDigits, minimumSignificantDigits, maximumSignificantDigits, roundingPriority, roundingIncrement, roundingMode, trailingZeroDisplay, notation, compactDisplay, useGrouping, signDisplay, numberingSystem, calendar, hour12, hourCycle, timeZone, weekday, era, year, month, day, dayPeriod, hour, minute, second, fractionalSecondDigits, timeZoneName, formatMatcher, dateStyle, timeStyle)"
```

## Examples

### JavaScript

#### Don't

```js
const fmt = new Intl.NumberFormat("en", { maximumFractionDigit: 2 });
```

#### Do

```js
const fmt = new Intl.NumberFormat("en", { maximumFractionDigits: 2 });
```


### JSX

#### Don't

```jsx
const fmt = new Intl.NumberFormat("en", { maximumFractionDigit: 2 });
```

#### Do

```jsx
const fmt = new Intl.NumberFormat("en", { maximumFractionDigits: 2 });
```


### TypeScript

#### Don't

```ts
const fmt = new Intl.NumberFormat("en", { maximumFractionDigit: 2 });
```

#### Do

```ts
const fmt = new Intl.NumberFormat("en", { maximumFractionDigits: 2 });
```


### TSX

#### Don't

```tsx
const fmt = new Intl.NumberFormat("en", { maximumFractionDigit: 2 });
```

#### Do

```tsx
const fmt = new Intl.NumberFormat("en", { maximumFractionDigits: 2 });
```
