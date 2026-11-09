---
id: sec-js/no-disabled-markup-escaping
title: "No turning off a template engine's HTML escaping"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the value that needed raw markup has to be escaped or sanitised where it is built, which is the author's call"
fix_exception: security
security: { cwe: [79], owasp: ["A05:2025"], severity: high }
stability: preview
ported_from: [eslint-plugin-security/detect-disable-mustache-escape]
since: 0.5.0
---

An assignment of `false` to a property named `escapeMarkup` (`Handlebars.escapeMarkup = false`, `view.escapeMarkup = false`) is reported: it is how Mustache-style template engines are told to stop escaping HTML entities. Other values, and other property names, are not reported.

**Why:** With escaping off, every value a template prints is inserted as markup, so any untrusted byte in it is cross-site scripting (CWE-79).

```codelaw
match:
  pattern: $OBJ.escapeMarkup = false
message: "HTML escaping is turned off here: every value this template prints is inserted as markup"
```

## How to fix

1. Remove the assignment: leave escaping on.
2. For the one value that must stay markup, sanitise it where it is built (an allow-list sanitiser) and mark only that value as safe.

## Examples

### JavaScript

#### Don't

```js
template.escapeMarkup = false;
```

#### Do

```js
template.escapeMarkup = true;
```


### JSX

#### Don't

```jsx
template.escapeMarkup = false;
```

#### Do

```jsx
template.escapeMarkup = true;
```


### TypeScript

#### Don't

```ts
template.escapeMarkup = false;
```

#### Do

```ts
template.escapeMarkup = true;
```


### TSX

#### Don't

```tsx
template.escapeMarkup = false;
```

#### Do

```tsx
template.escapeMarkup = true;
```
