---
id: html/deprecated-class
title: "No class the project has deprecated"
languages: [html]
fix: auto
fix_reason: "auto when the option gives the class's `replacement`; a class with only a message is reported without an edit"
stability: preview
options:
  classes: { type: "object", default: {}, description: "deprecated class → its replacement (empty for none)" }
ported_from: [html-validate/deprecated-class]
since: 0.5.0
---

A class named in `classes` isn't used: the option maps each deprecated class to its replacement, or to `""` when it has none. With no option set, the rule does nothing.

**Why:** A design system retires classes as it changes.

```codelaw
native: html/deprecated-class
```

## Examples

### HTML

#### Don't

```html options={"classes":{"btn-old":"btn"}}
<button class="btn-old">x</button>
```

#### Do

```html options={"classes":{"btn-old":"btn"}}
<button class="btn">x</button>
```
