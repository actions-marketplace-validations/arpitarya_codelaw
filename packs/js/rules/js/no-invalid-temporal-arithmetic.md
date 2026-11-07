---
id: js/no-invalid-temporal-arithmetic
title: "No days or weeks added to a Temporal.Instant"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a calendar day is not always 24 hours; the hours form is what an instant can take, and whether that was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-temporal-arithmetic]
since: 0.5.0
---

`Temporal.Instant#add` and `#subtract` throw a `RangeError` for `days` or `weeks`: an instant has no calendar or time zone to count them in. On an instant from `Temporal.Now.instant()` or a `Temporal.Instant.from…` call, `{ days: n }` is `{ hours: 24 * n }` and `{ weeks: n }` is `{ hours: 168 * n }`. Upstream also checks rounding options, units and `relativeTo` on every Temporal type; those need the receiver's type, and are not reported here.

**Why:** The call throws every time it runs.

```codelaw
utils:
  instant:
    any:
      - pattern: "Temporal.Now.instant()"
      - kind: call_expression
        has: { field: function, regex: '^Temporal\.Instant\.(from|fromEpochMilliseconds|fromEpochNanoseconds)$' }
match:
  kind: call_expression
  all:
  - has: { field: function, kind: member_expression, all: [{ has: { field: object, matches: instant } }, { has: { field: property, regex: '^(add|subtract)$' } }] }
  - has:
     field: arguments
     has:
      nthChild: 1
      kind: object
      any:
        - has: { kind: pair, all: [{ has: { field: key, regex: '^days$' } }, { has: { field: value, pattern: $D } }], pattern: $PD }
        - has: { kind: pair, all: [{ has: { field: key, regex: '^weeks$' } }, { has: { field: value, pattern: $WK } }], pattern: $PW }
message: "An instant can't add calendar units; use hours"
fix:
  - replace: { target: $PD, with: "hours: 24 * $D" }
  - replace: { target: $PW, with: "hours: 168 * $WK" }
```

## Examples

### JavaScript

#### Don't

```js
const later = Temporal.Now.instant().add({ days: 1 });
```

#### Do

```js
const later = Temporal.Now.instant().add({ hours: 24 * 1 });
```


### JSX

#### Don't

```jsx
const later = Temporal.Now.instant().add({ days: 1 });
```

#### Do

```jsx
const later = Temporal.Now.instant().add({ hours: 24 * 1 });
```


### TypeScript

#### Don't

```ts
const later = Temporal.Now.instant().add({ days: 1 });
```

#### Do

```ts
const later = Temporal.Now.instant().add({ hours: 24 * 1 });
```


### TSX

#### Don't

```tsx
const later = Temporal.Now.instant().add({ days: 1 });
```

#### Do

```tsx
const later = Temporal.Now.instant().add({ hours: 24 * 1 });
```
