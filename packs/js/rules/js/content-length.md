---
id: js/content-length
title: "A file upload parser has a size limit"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the right limit is the endpoint owner's"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/content-length]
since: 0.5.0
---

`formidable()`, `new IncomingForm()` and `new Formidable()` with no options accept uploads up to formidable's large default. A call without an options object, or whose options set no `maxFileSize`, is reported. Upstream also compares explicit sizes with a threshold and checks `multer` and `body-parser`; this rule reports the missing limit.

**Why:** An unbounded upload lets a client fill the disk or memory.

```codelaw
match:
  any:
    - kind: call_expression
      has: { field: function, regex: '^(formidable|formidableModule)$' }
    - kind: new_expression
      has: { field: constructor, regex: '^(IncomingForm|Formidable|formidable\.IncomingForm|formidable\.Formidable)$' }
  not:
    has:
      field: arguments
      any:
        - has: { kind: object, has: { kind: pair, has: { field: key, regex: '^["'']?maxFileSize["'']?$' } } }
        - has: { kind: identifier }
        - has: { kind: object, has: { kind: spread_element } }
message: "Set `maxFileSize` on this upload parser"
```

## How to fix

1. Pass `{ maxFileSize: … }` sized for what the endpoint accepts (and `maxFiles`, `maxFields` where it matters).

## Examples

### JavaScript

#### Don't

```js
const form = formidable();
```

#### Do

```js
const form = formidable({ maxFileSize: 8 * 1024 * 1024 });
```


### JSX

#### Don't

```jsx
const form = formidable();
```

#### Do

```jsx
const form = formidable({ maxFileSize: 8 * 1024 * 1024 });
```


### TypeScript

#### Don't

```ts
const form = formidable();
```

#### Do

```ts
const form = formidable({ maxFileSize: 8 * 1024 * 1024 });
```


### TSX

#### Don't

```tsx
const form = formidable();
```

#### Do

```tsx
const form = formidable({ maxFileSize: 8 * 1024 * 1024 });
```
