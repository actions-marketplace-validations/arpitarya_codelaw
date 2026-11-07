---
id: sec-html/no-plugin-embed
title: "No <object>, <embed> or <applet>"
languages: [html]
fix: suggest
fix_reason: "an `<embed>` of a picture, a video or a sound, by its `type` or file extension, becomes `<img>`, `<video controls>` or `<audio controls>`; anything else (Flash, a document) is manual"
fix_exception: restructure
stability: preview
security: { owasp: ["A02:2025"], severity: low }
since: 0.2.0
---

The page does not use `<object>`, `<embed>` or `<applet>` (an `<object>` that holds an image is fine).

**Why:** They load a plugin or another document with the page's trust, and the plugins that needed them are gone from browsers.

```codelaw
native: sec-html/no-plugin-embed
```

## How to fix

1. Use `<img>`, `<video>` or `<audio>` for media.
2. Use a sandboxed `<iframe>` for another document.
3. Use `<a download>` for a file to fetch.

## Examples

### HTML

#### Don't

```html
<embed src="movie.mp4">
```

#### Do

```html
<video src="movie.mp4" controls></video>
```
