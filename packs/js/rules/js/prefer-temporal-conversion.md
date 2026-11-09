---
id: js/prefer-temporal-conversion
title: "A `Temporal` conversion method instead of rebuilding the value"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the conversion is exact for a direct `from(source)`, a nanosecond-based `Instant`, a serialization into a plain type and a complete time-field rebuild, and is made; millisecond-based `Instant`s lose submillisecond precision, serialization into an `Instant` can lose seconds, and a date-bearing property bag without a calendar defaults to ISO 8601, so those are suggestions; a comment inside leaves the report without a fix"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-temporal-conversion]
since: 0.5.0
---

`Temporal.Instant.from(zoned)`, `Temporal.Instant.fromEpochNanoseconds(zoned.epochNanoseconds)`, `Temporal.PlainDate.from(zoned.toString())`, `new Temporal.PlainDate(zoned.year, zoned.month, zoned.day)` and the property-bag forms are reported when the source is known to be a `Temporal.ZonedDateTime` (to `Instant`, `PlainDateTime`, `PlainDate`, `PlainTime`) or a `Temporal.PlainDateTime` (to `PlainDate`, `PlainTime`): use `zoned.toInstant()`, `.toPlainDateTime()`, `.toPlainDate()`, `.toPlainTime()`. A source is known by `new Temporal.X()`, `Temporal.X.from()`, `Temporal.Now.zonedDateTimeISO()`/`plainDateTimeISO()`, an annotation `Temporal.X`, or a `const` alias of one. Rebuilds that copy only some fields, add or change one, mix receivers or take options are not reported.

**Why:** The conversion method keeps every field, including the sub-millisecond ones and the calendar, that a rebuild by hand can drop.

```codelaw
native: js/prefer-temporal-conversion
```

## Examples

### JavaScript

#### Don't

```js
const zoned = Temporal.Now.zonedDateTimeISO();
const day = Temporal.PlainDate.from(zoned);
```

#### Do

```js
const zoned = Temporal.Now.zonedDateTimeISO();
const day = zoned.toPlainDate();
```

#### Fixed

```js
const zoned = Temporal.Now.zonedDateTimeISO();
const day = zoned.toPlainDate();
```


### JSX

#### Don't

```jsx
const zoned = Temporal.Now.zonedDateTimeISO();
const day = Temporal.PlainDate.from(zoned);
```

#### Do

```jsx
const zoned = Temporal.Now.zonedDateTimeISO();
const day = zoned.toPlainDate();
```

#### Fixed

```jsx
const zoned = Temporal.Now.zonedDateTimeISO();
const day = zoned.toPlainDate();
```


### TypeScript

#### Don't

```ts
const zoned = Temporal.Now.zonedDateTimeISO();
const day = Temporal.PlainDate.from(zoned);
```

#### Do

```ts
const zoned = Temporal.Now.zonedDateTimeISO();
const day = zoned.toPlainDate();
```

#### Fixed

```ts
const zoned = Temporal.Now.zonedDateTimeISO();
const day = zoned.toPlainDate();
```


### TSX

#### Don't

```tsx
const zoned = Temporal.Now.zonedDateTimeISO();
const day = Temporal.PlainDate.from(zoned);
```

#### Do

```tsx
const zoned = Temporal.Now.zonedDateTimeISO();
const day = zoned.toPlainDate();
```

#### Fixed

```tsx
const zoned = Temporal.Now.zonedDateTimeISO();
const day = zoned.toPlainDate();
```
