---
id: js/no-ineffective-csp-directives
title: "No frame-ancestors, sandbox or report-uri in a CSP meta tag"
languages: [jsx, tsx]
fix: manual
fix_reason: "the directives only work as an HTTP header, which is set in the server's configuration, not in this file"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-unicorn/no-ineffective-csp-directives]
since: 0.5.0
---

Browsers ignore the `frame-ancestors`, `sandbox` and `report-uri` directives of a Content Security Policy delivered by `<meta http-equiv="Content-Security-Policy">`. A JSX `<meta>` whose `httpEquiv` (or `http-equiv`) is that header, with a literal `content` naming one of them, is reported. Upstream also checks HTML files and constants; this rule reads JSX literals.

**Why:** The markup looks like clickjacking protection and provides none.

```codelaw
match:
  any:
    - kind: jsx_self_closing_element
    - kind: jsx_opening_element
  has: { field: name, regex: '^meta$' }
  all:
    - has: { kind: jsx_attribute, regex: '^(httpEquiv|http-equiv)\s*=\s*["''{`]*\s*[Cc][Oo][Nn][Tt][Ee][Nn][Tt]-[Ss][Ee][Cc][Uu][Rr][Ii][Tt][Yy]-[Pp][Oo][Ll][Ii][Cc][Yy]["''}`]*$' }
    - has: { kind: jsx_attribute, regex: '^content\s*=\s*\{?\s*("([^"]*[;\s])?|''([^'']*[;\s])?)\s*(frame-ancestors|sandbox|report-uri)(\s|;|["'']|$)' }
message: "Browsers ignore this directive in a meta tag; send it as an HTTP header"
```

## How to fix

1. Send the policy as a `Content-Security-Policy` response header from the server, where these directives take effect.
2. Remove them from the `<meta>` content so the markup doesn't claim a protection it lacks.

## Examples

### JSX

#### Don't

```jsx
const head = <meta httpEquiv="Content-Security-Policy" content="frame-ancestors 'none'" />;
```

#### Do

```jsx
const head = <meta httpEquiv="Content-Security-Policy" content="default-src 'self'" />;
```


### TSX

#### Don't

```tsx
const head = <meta httpEquiv="Content-Security-Policy" content="frame-ancestors 'none'" />;
```

#### Do

```tsx
const head = <meta httpEquiv="Content-Security-Policy" content="default-src 'self'" />;
```
