---
id: css/no-unused-classes
title: "A class a global stylesheet declares is used by a file that loads it"
languages: [css, scss]
scope: project
severity: warn
fix: suggest
fix_reason: "a class can be built at run time or used by markup outside the repo, so deleting its rule needs a person to accept the diff"
stability: preview
ported_from: [biome/nursery/noUnusedClasses]
since: 0.5.2
---

Every class a global stylesheet declares is written by some JavaScript, TypeScript or HTML file that loads with it: a file whose entries reach the stylesheet (the file imports it, an importer does, a page links it, or another stylesheet `@import`s it). Behaviour of Biome's `noUnusedClasses`, read from its documentation and rewritten.

**Why:** a class nobody writes is dead CSS every page downloads, and it hides the classes the markup really uses.

A word counts as a use wherever a file writes it: a string, a template, an attribute value (`className`, `class`, `classList.add`, a library's prop), and `@apply` or `composes` in a stylesheet. The static start of a name built at run time (`btn-${size}`) keeps every class that begins with it. A stylesheet nothing loads is left alone, and so is one the rule cannot read whole (a class named by interpolation), an entry point and anything in `[graph] keep`. A CSS Module is not a global stylesheet: its classes are judged by `css-modules/no-unused-class`.

```codelaw
graph:
  sheets:
    classes: unused
message: "`.$CLASS` is declared in `$FILE` but no file that loads it uses it"
fix:
  - else:
    do: [remove-rule]
```

## How to fix

1. Search for the class: a name built at run time counts only through its static start.
2. If markup outside the repo uses it, add the stylesheet to `[graph] keep`.
3. Otherwise delete the rule (`codelaw fix --unsafe` does it when the class is the rule's only selector).

## Examples

### CSS

#### Don't

```css files
/* src/styles.css */
.card { padding: 1rem; }
.legacy { color: red; }
// src/Card.jsx
import "./styles.css";
export const Card = () => <div className="card" />;
```

#### Do

```css files
/* src/styles.css */
.card { padding: 1rem; }
// src/Card.jsx
import "./styles.css";
export const Card = () => <div className="card" />;
```

### SCSS

#### Don't

```scss files
/* src/styles.scss */
.card { padding: 1rem; }
.legacy { color: red; }
// src/Card.jsx
import "./styles.scss";
export const Card = () => <div className="card" />;
```

#### Do

```scss files
/* src/styles.scss */
.card { padding: 1rem; }
// src/Card.jsx
import "./styles.scss";
export const Card = () => <div className="card" />;
```

## Differences from the source

Biome's `noUnusedClasses` reads the files that import one stylesheet. This rule reads what loads with it through the project graph, so a class used by any file under the same entry counts. `graph/no-unused-css-classes` (purgecss) counts a word written anywhere in the project; this one counts only files the stylesheet loads with, so two apps in one repository are judged apart.
