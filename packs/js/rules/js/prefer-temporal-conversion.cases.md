# js/prefer-temporal-conversion — cases

### case: Instant from a zoned value

```js
const z = Temporal.Now.zonedDateTimeISO();
const i = Temporal.Instant.from(z);
```

```expect
2:11 js/prefer-temporal-conversion
```

```fixed
const z = Temporal.Now.zonedDateTimeISO();
const i = z.toInstant();
```

### case: nanoseconds are exact

```js
const z = Temporal.Now.zonedDateTimeISO();
const i = Temporal.Instant.fromEpochNanoseconds(z.epochNanoseconds);
```

```expect
2:11 js/prefer-temporal-conversion
```

```fixed
const z = Temporal.Now.zonedDateTimeISO();
const i = z.toInstant();
```

### case: milliseconds are a suggestion

```js
const z = Temporal.Now.zonedDateTimeISO();
const i = Temporal.Instant.fromEpochMilliseconds(z.epochMilliseconds);
```

```expect
2:11 js/prefer-temporal-conversion
```

```fixed
const z = Temporal.Now.zonedDateTimeISO();
const i = z.toInstant();
```

### case: a plain date from a string

```js
const z = Temporal.Now.zonedDateTimeISO();
const d = Temporal.PlainDate.from(z.toString());
```

```expect
2:11 js/prefer-temporal-conversion
```

```fixed
const z = Temporal.Now.zonedDateTimeISO();
const d = z.toPlainDate();
```

### case: date fields in a constructor

```js
const z = Temporal.Now.zonedDateTimeISO();
const d = new Temporal.PlainDate(z.year, z.month, z.day);
```

```expect
2:11 js/prefer-temporal-conversion
```

```fixed
const z = Temporal.Now.zonedDateTimeISO();
const d = z.toPlainDate();
```

### case: a property bag with the calendar

```js
const z = Temporal.Now.plainDateTimeISO();
const d = Temporal.PlainDate.from({ year: z.year, month: z.month, day: z.day, calendar: z.calendarId });
```

```expect
2:11 js/prefer-temporal-conversion
```

```fixed
const z = Temporal.Now.plainDateTimeISO();
const d = z.toPlainDate();
```

### case: every time field

```js
const z = Temporal.Now.plainDateTimeISO();
const t = new Temporal.PlainTime(z.hour, z.minute, z.second, z.millisecond, z.microsecond, z.nanosecond);
```

```expect
2:11 js/prefer-temporal-conversion
```

```fixed
const z = Temporal.Now.plainDateTimeISO();
const t = z.toPlainTime();
```

### case: a missing field

```js
const z = Temporal.Now.plainDateTimeISO();
const d = new Temporal.PlainDate(z.year, z.month, 1);
```

```expect
(none)
```

### case: a source of another type

```js
const z = Temporal.Now.instant();
const d = Temporal.PlainDate.from(z);
```

```expect
(none)
```

### case: an unknown source

```js
const d = Temporal.PlainDate.from(value);
```

```expect
(none)
```

### case: mixed receivers

```js
const a = Temporal.Now.plainDateTimeISO();
const b = Temporal.Now.plainDateTimeISO();
const d = new Temporal.PlainDate(a.year, b.month, a.day);
```

```expect
(none)
```

### case: an annotated parameter

```ts
function f(z: Temporal.ZonedDateTime) {
  return Temporal.PlainTime.from(z.toJSON());
}
```

```expect
2:10 js/prefer-temporal-conversion
```

```fixed
function f(z: Temporal.ZonedDateTime) {
  return z.toPlainTime();
}
```
