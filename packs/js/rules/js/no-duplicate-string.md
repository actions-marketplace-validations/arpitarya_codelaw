---
id: js/no-duplicate-string
title: "No string literal written three or more times"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the constant needs a name, which is the author's"
fix_exception: value
options: { threshold: { type: "integer", default: 3, description: "how many times a string may be written before it is reported" } }
stability: preview
ported_from: [eslint-plugin-sonarjs/no-duplicate-string]
since: 0.5.0
---

A string of ten or more characters written `threshold` (3) or more times in a file is defined once as a constant. Single words, module paths, `require` arguments, JSX attribute values, object keys, type literals and directives such as `"use strict"` are left. The first use is reported.

**Why:** One constant changes in one place, and its name says what the text is.

```codelaw
native: js/no-duplicate-string
```

## How to fix

1. Declare the string once, `const NAME = "…";`, and use the constant at each place.

## Examples

### JavaScript

#### Don't

```js
log("Request failed, retrying");
log("Request failed, retrying");
log("Request failed, retrying");
```

#### Do

```js
const RETRYING = "Request failed, retrying";
log(RETRYING);
log(RETRYING);
log(RETRYING);
```


### JSX

#### Don't

```jsx
log("Request failed, retrying");
log("Request failed, retrying");
log("Request failed, retrying");
```

#### Do

```jsx
const RETRYING = "Request failed, retrying";
log(RETRYING);
log(RETRYING);
log(RETRYING);
```


### TypeScript

#### Don't

```ts
log("Request failed, retrying");
log("Request failed, retrying");
log("Request failed, retrying");
```

#### Do

```ts
const RETRYING = "Request failed, retrying";
log(RETRYING);
log(RETRYING);
log(RETRYING);
```


### TSX

#### Don't

```tsx
log("Request failed, retrying");
log("Request failed, retrying");
log("Request failed, retrying");
```

#### Do

```tsx
const RETRYING = "Request failed, retrying";
log(RETRYING);
log(RETRYING);
log(RETRYING);
```
