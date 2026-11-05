---
id: next/inline-script-id
title: "An inline next/script has an id"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the id is the author's to name"
fix_exception: value
stability: preview
ported_from: ["@next/eslint-plugin-next/inline-script-id"]
since: 0.5.0
---

A `<Script>` from `next/script` with inline content (children or `dangerouslySetInnerHTML`) has an `id`.

**Why:** Next.js uses the `id` to track and de-duplicate an inline script; without it the script can run more than once.

```codelaw
match:
  any:
  - kind: jsx_element
    all:
    - has:
        field: open_tag
        has: {field: name, regex: ^Script$}
        not:
          has: {kind: jsx_attribute, regex: ^id\b}
    - has:
        any:
        - {kind: jsx_expression}
        - {kind: jsx_text, regex: \S}
  - all:
    - any:
      - {kind: jsx_opening_element}
      - {kind: jsx_self_closing_element}
    - has: {field: name, regex: ^Script$}
    - has: {kind: jsx_attribute, regex: ^dangerouslySetInnerHTML\b}
    - not:
        has: {kind: jsx_attribute, regex: ^id\b}
  inside:
    kind: program
    stopBy: end
    has: {kind: import_statement, regex: 'from\s*["'']next/script["'']'}
message: Give this inline `<Script>` an `id`
```

## How to fix

1. Add a unique `id` to the `<Script>`.

## Examples

### JavaScript

#### Don't

```js
import Script from "next/script";
const s = <Script>{`console.log(1)`}</Script>;
```

#### Do

```js
import Script from "next/script";
const s = <Script id="log">{`console.log(1)`}</Script>;
```

### JSX

#### Don't

```jsx
import Script from "next/script";
const s = <Script>{`console.log(1)`}</Script>;
```

#### Do

```jsx
import Script from "next/script";
const s = <Script id="log">{`console.log(1)`}</Script>;
```

### TSX

#### Don't

```tsx
import Script from "next/script";
const s = <Script>{`console.log(1)`}</Script>;
```

#### Do

```tsx
import Script from "next/script";
const s = <Script id="log">{`console.log(1)`}</Script>;
```
