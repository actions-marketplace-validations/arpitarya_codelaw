---
id: md/table-column-count
title: "Table rows have as many cells as the header"
languages: [markdown]
fix: auto
fix_reason: "a row ending in `|` with fewer cells than the header gets empty ones, which GFM already renders, so nothing changes on the page; a row with more cells loses content either way and is manual"
fix_exception: value
stability: preview
ported_from: [markdownlint/MD056]
since: 0.1.0
---

Every row of a pipe table has the same number of cells as its header row.

**Why:** Renderers drop extra cells and leave missing ones empty, so a misplaced pipe silently loses or shifts data.

```codelaw
native: md/table-column-count
```

## How to fix

1. Compare the row with the header.
2. Add the missing cells or remove the extra ones, or add a header column if the data needs it.

## Examples

### Markdown

#### Don't

````md
| Name | Type | Note |
| ---- | ---- | ---- |
| id | number |
````

#### Fixed

````md
| Name | Type | Note |
| ---- | ---- | ---- |
| id | number | |
````

#### Do

````md
| Name | Type | Note |
| ---- | ---- | ---- |
| id | number | required |
````

