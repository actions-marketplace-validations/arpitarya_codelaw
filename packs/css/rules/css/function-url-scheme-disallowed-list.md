---
id: css/function-url-scheme-disallowed-list
title: "No URL scheme the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the URL scheme is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the schemes disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/function-url-scheme-disallowed-list]
since: 0.5.0
---

No URL scheme named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/function-url-scheme-disallowed-list
```

## How to fix

1. Use an other URL scheme, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["http"]}
a {
  background: url("http://example.com/a.png");
}
```

#### Do

```css options={"list":["http"]}
a {
  background: url("https://example.com/a.png");
}
```

### SCSS

#### Don't

```scss options={"list":["http"]}
a {
  background: url("http://example.com/a.png");
}
```

#### Do

```scss options={"list":["http"]}
a {
  background: url("https://example.com/a.png");
}
```
