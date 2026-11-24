---
id: react/no-obsolete-attributes
title: "No obsolete HTML attribute"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the replacement is CSS or another element, and which depends on the intent"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noObsoleteAttributes, "@html-eslint/no-obsolete-attrs"]
since: 0.5.1
---

A JSX element of the table below does not set an attribute HTML has made obsolete on it: `align`, `valign`, `bgcolor`, `border` and `cellpadding` on tables and their cells, `frameborder` and `scrolling` on `<iframe>`, `language` on `<script>`, `hspace`/`vspace` on images, `link`/`text`/`background` on `<body>`, and the others listed in the rule. Only intrinsic (lowercase) elements are read. Behaviour of Biome's `noObsoleteAttributes`.

**Why:** The browser still reads some of these and ignores others; the presentation belongs in CSS where one place decides it.

```codelaw
match:
  all:
    - any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
    - any:
        - all:
            - has: { field: name, kind: identifier, regex: '^(caption|col|colgroup|div|h1|h2|h3|h4|h5|h6|hr|iframe|img|input|legend|p|table|tbody|td|tfoot|th|thead|tr)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(align)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(body|table|td|th|tr)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(bgcolor)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(img|object|table)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(border)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(table)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(cellpadding|cellspacing|frame|rules|summary)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(col|colgroup|tbody|td|tfoot|th|thead|tr)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(valign|char|charoff)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(iframe)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(frameborder|marginheight|marginwidth|scrolling|longdesc)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(img|object)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(hspace|vspace)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(td|th)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(nowrap|axis|abbr)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(script)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(language)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(body)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(alink|background|link|text|vlink)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(dl|menu|ol|ul)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(compact)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(hr)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(noshade|size)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(br)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(clear)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(a|link)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(charset|rev|shape|coords)$', pattern: $A } }
        - all:
            - has: { field: name, kind: identifier, regex: '^(html)$' }
            - has: { kind: jsx_attribute, has: { kind: property_identifier, regex: '^(version)$', pattern: $A } }
message: "`$A` is an obsolete HTML attribute on this element; use CSS"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const t = <table cellpadding="4" />;
```

#### Do

```js
const t = <table className="roomy" />;
```

### JSX

#### Don't

```jsx
const t = <table cellpadding="4" />;
```

#### Do

```jsx
const t = <table className="roomy" />;
```

### TSX

#### Don't

```tsx
const t = <table cellpadding="4" />;
```

#### Do

```tsx
const t = <table className="roomy" />;
```
