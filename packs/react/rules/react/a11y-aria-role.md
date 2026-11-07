---
id: react/a11y-aria-role
title: "A role is a real ARIA role"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "the nearest valid role by spelling (at most two edits, and no other as near) is offered; which was meant is still the author's, and a role with no single near one is fixed by hand"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/aria-role]
since: 0.5.0
---

Every token of a `role` written as a string is a WAI-ARIA, DPUB-ARIA or Graphics-ARIA role that is not abstract.

**Why:** An unknown role is ignored, and the element is announced as whatever it was before — usually not what the author meant.

```codelaw
match:
  kind: jsx_attribute
  regex: '^role=["'']'
  has: { kind: string, pattern: $V }
  not: { regex: '^role=["'']\s*(alert|alertdialog|application|article|banner|blockquote|button|caption|cell|checkbox|code|columnheader|combobox|comment|complementary|contentinfo|definition|deletion|dialog|directory|document|emphasis|feed|figure|form|generic|grid|gridcell|group|heading|img|insertion|link|list|listbox|listitem|log|main|mark|marquee|math|menu|menubar|menuitem|menuitemcheckbox|menuitemradio|meter|navigation|none|note|option|paragraph|presentation|progressbar|radio|radiogroup|region|row|rowgroup|rowheader|scrollbar|search|searchbox|separator|slider|spinbutton|status|strong|subscript|suggestion|superscript|switch|tab|table|tablist|tabpanel|term|textbox|time|timer|toolbar|tooltip|tree|treegrid|treeitem|doc-[a-z]+|graphics-(document|object|symbol))(\s+(alert|alertdialog|application|article|banner|blockquote|button|caption|cell|checkbox|code|columnheader|combobox|comment|complementary|contentinfo|definition|deletion|dialog|directory|document|emphasis|feed|figure|form|generic|grid|gridcell|group|heading|img|insertion|link|list|listbox|listitem|log|main|mark|marquee|math|menu|menubar|menuitem|menuitemcheckbox|menuitemradio|meter|navigation|none|note|option|paragraph|presentation|progressbar|radio|radiogroup|region|row|rowgroup|rowheader|scrollbar|search|searchbox|separator|slider|spinbutton|status|strong|subscript|suggestion|superscript|switch|tab|table|tablist|tabpanel|term|textbox|time|timer|toolbar|tooltip|tree|treegrid|treeitem|doc-[a-z]+|graphics-(document|object|symbol)))*\s*["'']$' }
  inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^[a-z]' } }
message: "`$$MATCH` names a role WAI-ARIA does not define"
fix:
  - replace: { target: $V, with: "$V|nearest(alert, alertdialog, application, article, banner, blockquote, button, caption, cell, checkbox, code, columnheader, combobox, comment, complementary, contentinfo, definition, deletion, dialog, directory, document, emphasis, feed, figure, form, generic, grid, gridcell, group, heading, img, insertion, link, list, listbox, listitem, log, main, mark, marquee, math, menu, menubar, menuitem, menuitemcheckbox, menuitemradio, meter, navigation, none, note, option, paragraph, presentation, progressbar, radio, radiogroup, region, row, rowgroup, rowheader, scrollbar, search, searchbox, separator, slider, spinbutton, status, strong, subscript, suggestion, superscript, switch, tab, table, tablist, tabpanel, term, textbox, time, timer, toolbar, tooltip, tree, treegrid, treeitem)" }
```

## How to fix

1. Use a defined, non-abstract role (`button`, `navigation`, `dialog`, …) or remove the attribute.

## Examples

### JavaScript

#### Don't

```js
const d = <div role="buton" />;
```

#### Do

```js
const d = <div role="button" />;
```

### JSX

#### Don't

```jsx
const d = <div role="buton" />;
```

#### Do

```jsx
const d = <div role="button" />;
```

### TSX

#### Don't

```tsx
const d = <div role="buton" />;
```

#### Do

```tsx
const d = <div role="button" />;
```
