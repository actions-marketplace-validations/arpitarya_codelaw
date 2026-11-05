---
id: js/no-angular-bypass-sanitization
title: "No Angular sanitiser bypass"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "only the author knows whether the value is trusted"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/no-angular-bypass-sanitization]
since: 0.5.0
---

`DomSanitizer.bypassSecurityTrust*` is not called.

**Why:** A value marked trusted is inserted without sanitising, so user input becomes XSS.

```codelaw

match:
  kind: call_expression
  has: { field: function, regex: '\.bypassSecurityTrust(Html|Style|Script|Url|ResourceUrl)$' }
message: "This disables Angular's sanitiser for the value; make sure it can never come from a user"
```

## How to fix

1. Sanitise the value with `DomSanitizer.sanitize`, or prove it is a constant the user cannot influence.

## Examples

### JavaScript

#### Don't

```js
this.html = this.sanitizer.bypassSecurityTrustHtml(input);
```

#### Do

```js
this.html = this.sanitizer.sanitize(SecurityContext.HTML, input);
```

### JSX

#### Don't

```jsx
this.html = this.sanitizer.bypassSecurityTrustHtml(input);
```

#### Do

```jsx
this.html = this.sanitizer.sanitize(SecurityContext.HTML, input);
```

### TypeScript

#### Don't

```ts
this.html = this.sanitizer.bypassSecurityTrustHtml(input);
```

#### Do

```ts
this.html = this.sanitizer.sanitize(SecurityContext.HTML, input);
```

### TSX

#### Don't

```tsx
this.html = this.sanitizer.bypassSecurityTrustHtml(input);
```

#### Do

```tsx
this.html = this.sanitizer.sanitize(SecurityContext.HTML, input);
```
