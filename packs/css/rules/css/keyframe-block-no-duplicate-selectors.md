---
id: css/keyframe-block-no-duplicate-selectors
title: "No keyframe selector twice in one `@keyframes`"
languages: [css, scss]
fix: suggest
fix_reason: "the choices are merging the two blocks in source order (the default: what browsers render today, the later declarations winning), keeping the first or keeping the last; the author often meant another offset, which the code doesn't say, and a selector shared with other keyframes in one block is manual"
fix_exception: value
stability: stable
ported_from: [stylelint/keyframe-block-no-duplicate-selectors]
since: 0.1.0
---

A `@keyframes` block names each keyframe selector once.

**Why:** Two blocks for one keyframe merge silently, which is usually a copy-paste slip.

```codelaw
native: css/keyframe-block-no-duplicate-selectors
```

## How to fix

1. Merge the two blocks, or change one selector to the keyframe it was meant for.

## Examples

### CSS

#### Don't

```css
@keyframes fade {
  0% {
    opacity: 0;
  }

  0% {
    opacity: 1;
  }
}
```

#### Fixed (merge)

```css
@keyframes fade {
  0% {
    opacity: 0;
    opacity: 1;
  }

}
```

#### Fixed (keep first)

```css
@keyframes fade {
  0% {
    opacity: 0;
  }

}
```

#### Fixed (keep last)

```css
@keyframes fade {

  0% {
    opacity: 1;
  }
}
```

#### Do

```css
@keyframes fade {
  0% {
    opacity: 0;
  }

  100% {
    opacity: 1;
  }
}
```
### SCSS

#### Don't

```scss
@keyframes fade {
  0% {
    opacity: 0;
  }

  0% {
    opacity: 1;
  }
}
```

#### Fixed (merge)

```scss
@keyframes fade {
  0% {
    opacity: 0;
    opacity: 1;
  }

}
```

#### Fixed (keep first)

```scss
@keyframes fade {
  0% {
    opacity: 0;
  }

}
```

#### Fixed (keep last)

```scss
@keyframes fade {

  0% {
    opacity: 1;
  }
}
```

#### Do

```scss
@keyframes fade {
  0% {
    opacity: 0;
  }

  100% {
    opacity: 1;
  }
}
```
