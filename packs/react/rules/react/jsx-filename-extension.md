---
id: react/jsx-filename-extension
title: "JSX lives in .jsx and .tsx files"
languages: [javascript]
files: ["**/*.js"]
fix: manual
fix_reason: "restructure: the file is renamed, and every import of it follows"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/jsx-filename-extension]
since: 0.5.0
---

A `.js` file contains no JSX; JSX goes in a `.jsx` file (or `.tsx`). The rule reports the file once.

**Why:** The extension tells tools and readers that the file needs a JSX transform.

```codelaw
match:
  kind: program
  has: { stopBy: end, any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }] }
message: "JSX in a `.js` file; rename it to `.jsx`"
```

## How to fix

1. Rename the file to `.jsx`.
2. Update the imports that name the extension.

## Examples

### JavaScript

#### Don't

```js path=src/Button.js
export const Button = () => <button type="button" />;
```

#### Do

```js path=src/Button.js
export const label = "Save";
```
