---
id: react/no-obsolete-tags
title: "No obsolete HTML element"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the replacement — CSS, a semantic element or nothing — depends on the intent"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noObsoleteTags, "@html-eslint/no-obsolete-tags"]
since: 0.5.1
---

A JSX element is not one HTML has made obsolete: `<center>`, `<font>`, `<marquee>`, `<frame>`, `<tt>`, `<big>`, `<strike>` and the rest of the list. Behaviour of Biome's `noObsoleteTags`; `html/deprecated` is the same check on HTML files.

**Why:** Obsolete elements have no defined meaning and are rendered differently, or not at all, by current browsers.

```codelaw
match:
  any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
  has: { field: name, kind: identifier, regex: '^(acronym|applet|basefont|bgsound|big|blink|center|dir|font|frame|frameset|isindex|keygen|listing|marquee|menuitem|multicol|nextid|nobr|noembed|noframes|plaintext|rb|rtc|spacer|strike|tt|xmp)$' }
message: "This element is obsolete in HTML; use CSS or a current element"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const a = <center>Hi</center>;
```

#### Do

```js
const a = <div className="centered">Hi</div>;
```

### JSX

#### Don't

```jsx
const a = <center>Hi</center>;
```

#### Do

```jsx
const a = <div className="centered">Hi</div>;
```

### TSX

#### Don't

```tsx
const a = <center>Hi</center>;
```

#### Do

```tsx
const a = <div className="centered">Hi</div>;
```
