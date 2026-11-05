---
id: react/a11y-media-has-caption
title: "Audio and video have captions"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the captions file is the author's to provide"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsx-a11y/media-has-caption]
since: 0.5.0
---

An `<audio>` or `<video>` that is not `muted` has a `<track kind="captions">` inside it.

**Why:** Deaf and hard-of-hearing users get nothing from media without captions.

```codelaw
match:
  any:
  - kind: jsx_self_closing_element
    all:
    - has: {field: name, regex: ^(audio|video)$}
    - not:
        has: {kind: jsx_attribute, regex: ^muted\b}
    - not:
        has: {kind: jsx_expression}
  - kind: jsx_element
    has:
      field: open_tag
      has: {field: name, regex: ^(audio|video)$}
      not:
        any:
        - has: {kind: jsx_attribute, regex: ^muted\b}
        - has: {kind: jsx_expression}
    not:
      has:
        stopBy: end
        any:
        - {kind: jsx_expression}
        - all:
          - any:
            - {kind: jsx_opening_element}
            - {kind: jsx_self_closing_element}
          - has: {field: name, regex: ^track$}
          - has: {kind: jsx_attribute, regex: '^kind=["'']captions["'']'}
message: Add a `<track kind="captions">` to this media element
```

## How to fix

1. Add `<track kind="captions" src="…" />` inside the element, or `muted` if it has no sound.

## Examples

### JavaScript

#### Don't

```js
const v = <video src="intro.mp4" />;
```

#### Do

```js
const v = <video src="intro.mp4"><track kind="captions" src="intro.vtt" /></video>;
```

### JSX

#### Don't

```jsx
const v = <video src="intro.mp4" />;
```

#### Do

```jsx
const v = <video src="intro.mp4"><track kind="captions" src="intro.vtt" /></video>;
```

### TSX

#### Don't

```tsx
const v = <video src="intro.mp4" />;
```

#### Do

```tsx
const v = <video src="intro.mp4"><track kind="captions" src="intro.vtt" /></video>;
```
