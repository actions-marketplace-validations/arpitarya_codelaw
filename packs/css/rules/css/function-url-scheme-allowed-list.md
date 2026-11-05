---
id: css/function-url-scheme-allowed-list
title: "Only URL schemes the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the URL scheme is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the schemes allowed (`/re/` for a regular expression)" }
ported_from: [stylelint/function-url-scheme-allowed-list]
since: 0.5.0
---

Every URL scheme is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/function-url-scheme-allowed-list
```

## How to fix

1. Use an allowed URL scheme, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["https"]}
a {
  background: url("http://example.com/a.png");
}
```

#### Do

```css options={"list":["https"]}
a {
  background: url("https://example.com/a.png");
}
```

### SCSS

#### Don't

```scss options={"list":["https"]}
a {
  background: url("http://example.com/a.png");
}
```

#### Do

```scss options={"list":["https"]}
a {
  background: url("https://example.com/a.png");
}
```
